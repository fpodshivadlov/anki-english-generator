import { fetchWordsHtml, fetchWordDetails } from './client';
import { WordInfoData, WordMetadata } from './models';
import { parseListHtml, parseWordHtml } from './parser';

export * from './models';

export async function fetchEnglishProfile(levelsFilter?: string[]): Promise<WordInfoData[]> {
  console.log(`Processing the word list`);
  const listHtml = await fetchWordsHtml(levelsFilter);
  const wordList = parseListHtml(listHtml);
  console.log(`Total word count: '${wordList.length})`);

  let result = [];
  for (const word of wordList) {
    console.log(`Processing the word '${word.baseWord}' (level: ${word.level}, guide word: ${word.guideWord})`);
    const wordHtml = await fetchWordDetails(word.wordDetailsUrl);
    const wordDetails = await parseWordHtml(wordHtml, word);

    if (wordDetails)
      result.push(wordDetails);
  }

  return result;
}
