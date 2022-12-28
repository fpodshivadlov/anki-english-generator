use log::{info, warn};
use regex::Regex;
use scraper::{ElementRef, Html, Selector};

use super::models::LearnerExample;
use super::models::WordBaseData;
use super::models::WordFamily;
use super::models::WordInfoData;
use super::models::WordMetadata;
use super::utils::ensure_absolute_url;

fn extract_text_content(node: &ElementRef) -> String {
  node.inner_html()
}

fn extract_text_content_or_none(node: &ElementRef) -> Option<String> {
  let result = extract_text_content(node);
  if result.is_empty() {
    return None;
  }
  Some(result)
}

fn extract_href_content(node: &ElementRef, base_node: &ElementRef) -> String {
  let href = node.value().attr("href").unwrap_or_default();
  let base_href = base_node.value().attr("href").unwrap_or_default();

  let result = ensure_absolute_url(href, base_href);
  match result {
    Some(value) => value,
    None => href.to_string(),
  }
}

fn is_valid_details_url(url: &str) -> bool {
  // some urls are invalid and look like 'https://www.englishprofile.org/american-english/words/usdetail/'
  let has_word_id_regex = Regex::new(r"\d+$").unwrap();
  let result = has_word_id_regex.is_match(&url);
  result
}

pub fn parse_list_html(html: &str) -> Vec<WordMetadata> {
  let base_url_selector = Selector::parse("head base").unwrap();
  let trs_nodes_selector = Selector::parse("tbody tr").unwrap();
  let base_word_selector = Selector::parse("td:nth-child(1)").unwrap();
  let guide_word_selector = Selector::parse("td:nth-child(2)").unwrap();
  let level_selector = Selector::parse("td:nth-child(3) .label").unwrap();
  let part_of_speech_selector = Selector::parse("td:nth-child(4)").unwrap();
  let topic_selector = Selector::parse("td:nth-child(5)").unwrap();
  let word_details_url_selector = Selector::parse("td:last-child a").unwrap();

  let document = Html::parse_document(&html);
  let base_url = document.select(&base_url_selector).next().unwrap();
  let trs_nodes = document.select(&trs_nodes_selector);
  let words = trs_nodes.filter_map(|trs_node| {
      let base_word = trs_node.select(&base_word_selector).next().unwrap();
      let guideWord = trs_node.select(&guide_word_selector).next().unwrap();
      let level = trs_node.select(&level_selector).next().unwrap();
      let partOfSpeech = trs_node.select(&part_of_speech_selector).next().unwrap();
      let topic = trs_node.select(&topic_selector).next().unwrap();
      let wordDetailsUrlNode = trs_node.select(&word_details_url_selector).next().unwrap();

      let wordDetailsUrl = extract_href_content(&wordDetailsUrlNode, &base_url);
      if wordDetailsUrl.is_empty() {
        panic!("wordDetailsUrl {} is blank", wordDetailsUrl);
      }

      let base_word_value = extract_text_content(&base_word);

      if !is_valid_details_url(&wordDetailsUrl) {
        warn!("'{}': invalid URL {}", base_word_value, wordDetailsUrl);
        return None;
      }

      Some(WordMetadata {
        baseData: WordBaseData {
          baseWord: base_word_value,
          level: extract_text_content(&level),
          partOfSpeech: extract_text_content_or_none(&partOfSpeech),
          wordDetailsUrl,
        },
        topic: extract_text_content_or_none(&topic),
        guideWord: extract_text_content_or_none(&guideWord),
      })
  });

  words.collect::<Vec<_>>()
}

fn parseWordInfo(word: &WordMetadata, infoNode: &ElementRef, baseInfoNode: &ElementRef) -> WordInfoData {
  let wordFamilyNodes_selector = Selector::parse(".wf_pos_block").unwrap();
  let dictionaryExampleNodes_selector = Selector::parse(".example .blockquote").unwrap();
  let wordFamiliespartOfSpeech_selector = Selector::parse(".wf_pos").unwrap();
  let wordFamilies_selector = Selector::parse(".wf_word").unwrap();
  let learnerNodeRoot_selector = Selector::parse(".learner").unwrap();
  let learnerExample_selector = Selector::parse(".learnerexamp").unwrap();
  let cite_selector = Selector::parse(".cite").unwrap();
  let headword_selector = Selector::parse(".headword").unwrap();
  let partOfSpeech_selector = Selector::parse(".pos").unwrap();
  let transcription_selector = Selector::parse(".written").unwrap();
  let infoTitle_selector = Selector::parse(".sense_title").unwrap();
  let level_selector = Selector::parse(".label").unwrap();
  let grammar_selector = Selector::parse(".grammar").unwrap();
  let definition_selector = Selector::parse(".definition").unwrap();

  // ToDo: migrate try-catch
  let dictionaryExampleNodes = infoNode.select(&dictionaryExampleNodes_selector);

  let wordFamilyNodes = baseInfoNode.select(&wordFamilyNodes_selector);
  let wordFamilies = wordFamilyNodes.map(|div| {
    let partOfSpeech = div.select(&wordFamiliespartOfSpeech_selector).next().unwrap();
    let wordFamilyNodes = baseInfoNode.select(&wordFamilies_selector);

    WordFamily {
      partOfSpeech: extract_text_content(&partOfSpeech),
      words: wordFamilyNodes
        .map(|c| {
          extract_text_content(&c)
        })
        .collect::<Vec<_>>(),
    }
  });

  let learnerNode = infoNode.select(&learnerNodeRoot_selector).next();
  let learnerExample = match learnerNode {
    Some(nodeValue) => {
      let example = nodeValue.select(&learnerExample_selector).next().unwrap();
      let cite = nodeValue.select(&cite_selector).next().unwrap();
      Some(LearnerExample {
        example: extract_text_content(&example),
        cite: Some(extract_text_content(&cite)),
      })
    }
    None => None
  };

  let headword = baseInfoNode.select(&headword_selector).next().unwrap();
  let partOfSpeech = baseInfoNode.select(&partOfSpeech_selector).next().unwrap();
  let transcription = baseInfoNode.select(&transcription_selector).next().unwrap();
  let infoTitle = infoNode.select(&infoTitle_selector).next().unwrap();
  let level = infoNode.select(&level_selector).next().unwrap();
  let grammar = infoNode.select(&grammar_selector).next().unwrap();
  let definition = infoNode.select(&definition_selector).next().unwrap();

  WordInfoData {
    baseData: WordBaseData {
      baseWord: word.baseData.baseWord.clone(),
      partOfSpeech: extract_text_content_or_none(&partOfSpeech),
      level: extract_text_content(&level),
      wordDetailsUrl: word.baseData.wordDetailsUrl.clone(),
    },
    headword: extract_text_content(&headword),
    transcription: extract_text_content(&transcription),
    wordFamilies: wordFamilies
      .collect::<Vec<_>>(),
    infoTitle: extract_text_content(&infoTitle),
    grammar: Some(extract_text_content(&grammar)),
    definition: extract_text_content(&definition),
    dictionaryExamples: dictionaryExampleNodes
      .map(|p| extract_text_content(&p))
      .collect::<Vec<_>>(),
    learnerExample: learnerExample,
  }
}

fn sameOrEmpty(value1: &str, value2: &str) -> bool{
  return value1.is_empty() || value2.is_empty() || value1 == value2;
}

pub fn parseWordHtml(html: &str, word: &WordMetadata) -> Option<WordInfoData> {
  let baseInfoNodes_selector = Selector::parse(".pos_section").unwrap();
  let infoNodes_selector = Selector::parse(".info.sense").unwrap();

  let document = Html::parse_document(&html);

  let mut expectedWordTitle = word.baseData.baseWord.clone();
  if let Some(guide_word_value) = word.guideWord.clone() {
    expectedWordTitle = format!("{} ({})", expectedWordTitle, guide_word_value);
  }

  let baseInfoNodes = document.select(&baseInfoNodes_selector);
  let resolvedWordDetailsList = baseInfoNodes
    .flat_map(|baseInfoNode| {
      let infoNodes = baseInfoNode.select(&infoNodes_selector);
      infoNodes.flat_map(|baseInfoNode| {
        baseInfoNode
          .children()
          .map(move |infoNode| {
            let infoNode = ElementRef::wrap(infoNode).unwrap();
            let resolvedWordDetails = parseWordInfo(&word, &infoNode, &baseInfoNode);
            resolvedWordDetails
          })
      })
    })
    .collect::<Vec<WordInfoData>>();
  
  // let exact_match_word_details = resolvedWordDetailsList.iter()
  //   .find(|&word_details| word_details.infoTitle == expectedWordTitle);
  // if let Some(&value) = exact_match_word_details {
  //   return Some(value);
  // }

  // info!("'{}': no exact meaning found for '{}' on {}, trying to fallback", word.baseData.baseWord, expectedWordTitle, word.baseData.wordDetailsUrl);

  // let expectedWordTitle = word.baseData.baseWord;
  // let fallbackMatchedList = resolvedWordDetailsList.iter()
  //   .filter(|&&x|
  //     sameOrEmpty(x.infoTitle.as_str(), expectedWordTitle.as_str())
  //     && sameOrEmpty(x.baseData.level.as_str(), word.baseData.level.as_str())
  //     && sameOrEmpty(x.baseData.partOfSpeech.as_str(), word.baseData.partOfSpeech.as_str())
  //   )
  //   .collect::<Vec<_>>();

  // if fallbackMatchedList.len() == 1 {
  //   let first = fallbackMatchedList[0];
  //   return Some(first.clone());
  // }

  // warn!("'{}': no exact ({}) fallback meaning found for '{}' on {}", word.baseData.baseWord, fallbackMatchedList.len(), expectedWordTitle, word.baseData.wordDetailsUrl);

  return None;
}

#[cfg(test)]
mod tests;
