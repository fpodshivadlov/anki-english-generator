use futures::future;
use log::info;

mod models;
mod client;
mod parser;
mod utils;

use models::WordInfoData;
use client::fetch_words_html;
use client::fetch_word_details;
use parser::parse_list_html;
use parser::parseWordHtml;

use crate::models::CerfLevelEnum;
use crate::models::TopicEnum;

pub async fn fetchEnglishProfile(
  useAmerican: bool,
  levelsFilter: &Option<Vec<CerfLevelEnum>>,
  topicFilter: &Option<TopicEnum>,
  wordsFilter: &Option<Vec<String>>
) -> Vec<Option<WordInfoData>>
{
  info!("Processing the word list");
  let listHtml = fetch_words_html(useAmerican, &levelsFilter, &topicFilter).await;

  let mut wordList: Vec<_> = parse_list_html(&listHtml);
  info!("Total words count: {}", wordList.len());

  // ToDo:
  // if let Some(wordsFilter) = wordsFilter {
  //   wordList = wordList.iter()
  //     .filter(|word| {
  //       wordsFilter.contains(&word.baseData.baseWord)
  //     })
  //     .collect::V();
  //   info!("Filtered words count: {}", wordList.len());
  // }

  let jobs: Vec<_> = wordList.iter()
    .map(|word| async move {
      info!("Processing the word '{}' (level: {}, guide word: {})", word.baseData.baseWord, word.baseData.level, word.guideWord.clone().unwrap_or("-".to_string()));
      let wordHtml = fetch_word_details(&word.baseData.wordDetailsUrl).await;
      let wordDetails = parseWordHtml(&wordHtml, &word);
      wordDetails
    })
    .collect();

  let result = future::join_all(jobs).await;

  result
}
