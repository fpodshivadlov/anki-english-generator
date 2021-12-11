interface WordBaseData {
  baseWord: string;
  level: string;
  wordDetailsUrl: string;
}

export interface WordMetadata extends WordBaseData {
  guideWord: string;
  partOfSpeech: string;
  topic: string;
}

export interface WordFamily {
  partOfSpeech: string;
  words: string[];
}

export interface LearnerExample {
  example: string;
  cite?: string;
}

export interface WordInfoData extends WordBaseData {
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
