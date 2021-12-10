import { JSDOM } from 'jsdom';

import { WordInfoData, WordMetadata } from './models';

export function parseListHtml(html: string): WordMetadata[] {
  const wordsDom = new JSDOM(html);
  const document = wordsDom.window.document;

  let result = [];
  const trsNodes = document.querySelectorAll('tbody tr');
  for (const trNode of trsNodes) {
    const baseWord = trNode.querySelector('td:nth-child(1)').textContent;
    const guideWord = trNode.querySelector('td:nth-child(2)').textContent;
    const level = trNode.querySelector('td:nth-child(3) .label').textContent;
    const partOfSpeech = trNode.querySelector('td:nth-child(4)').textContent;
    const topic = trNode.querySelector('td:nth-child(5)').textContent;
    const wordDetailsUrl = trNode.querySelector('td:last-child a').href;
    
    if (!wordDetailsUrl) {
      throw new Error(`wordDetailsUrl (${wordDetailsUrl}) is blank`);
    }

    if (!wordDetailsUrl.match(/\d+$/)) {
      // some urls are invalid and look like 'https://www.englishprofile.org/american-english/words/usdetail/'
      console.warn(`'${baseWord}': invalid URL ${wordDetailsUrl}`);
      continue;
    }

    result.push({
      baseWord,
      guideWord,
      level,
      partOfSpeech,
      topic,
      wordDetailsUrl,
    });
  }

  return result;
}

function parseWordInfo(word: WordMetadata, infoNode: ParentNode, baseInfoNode: ParentNode): WordInfoData {
  try {

    const wordFamilyNodes = baseInfoNode.querySelectorAll('.wf_pos_block');
    const dictionaryExampleNodes = infoNode.querySelectorAll('.example .blockquote');
  
    const wordFamilies = Array.from(wordFamilyNodes).map((div) => {
      return {
        partOfSpeech: div.querySelector('.wf_pos').textContent,
        words: Array.from(div.querySelectorAll('.wf_word'))
          .map(c => c.textContent),
      };
    });

    const learnerNode = infoNode.querySelector('.learner');
    const learnerExample = learnerNode ? {
      example: learnerNode.querySelector('.learnerexamp').textContent,
      cite: learnerNode.querySelector('.cite').textContent,
    } : null;

    return {
      headword: baseInfoNode.querySelector('.headword').textContent,
      partOfSpeech: baseInfoNode.querySelector('.pos').textContent,
      transcription: baseInfoNode.querySelector('.written').textContent,
      wordFamilies,
      infoTitle: infoNode.querySelector('.sense_title').textContent,
      level: infoNode.querySelector('.label').textContent,
      grammar: infoNode.querySelector('.grammar')?.textContent,
      definition: infoNode.querySelector('.definition')?.textContent,
      dictionaryExamples: Array.from(dictionaryExampleNodes).map(p => p?.textContent),
      learnerExample: learnerExample,
    };
	}
  catch (error) {
		console.error(`Error caught for ${word.wordDetailsUrl}: ${error.message}\n${JSON.stringify(word)}`);
		throw(error);
	}
}

export async function parseWordHtml(html: string, word: WordMetadata): Promise<WordInfoData> {
  const wordsDom = new JSDOM(html);
  const document = wordsDom.window.document;

  let expectedWordTitle = `${word.baseWord}`;
  if (word.guideWord){
    expectedWordTitle += ` (${word.guideWord})`;
  }

  const baseInfoNodes = document.querySelectorAll('.pos_section');
  for (const baseInfoNode of baseInfoNodes) {
    const infoNodes = baseInfoNode.querySelectorAll('.info.sense');
    for (const infoNode of infoNodes) {
      const infoBody = parseWordInfo(word, infoNode, baseInfoNode);
      if (infoBody.infoTitle == expectedWordTitle) {
        return infoBody;
      }
    }
  }
  
  console.warn(`'${word.baseWord}': no meaning found for '${expectedWordTitle}' on ${word.wordDetailsUrl}`);
  return null;
}
