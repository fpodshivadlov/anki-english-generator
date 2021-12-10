export interface WordMetadata {
  baseWord: string;
  guideWord: string;
  level: string;
  partOfSpeech: string;
  topic: string;
  wordDetailsUrl: string;
}

export interface WordFamily {
  partOfSpeech: string;
  words: string[];
}

export interface LearnerExample {
  example: string;
  cite?: string;
}

export interface WordInfoData {
  headword: string,
  partOfSpeech: string,
  audioSourceUrl?: string,
  transcription: string,
  wordFamilies: WordFamily[],
  infoTitle: string;
  level: string;
  grammar?: string;
  definition: string;
  dictionaryExamples: string[];
  learnerExample: LearnerExample;
}
