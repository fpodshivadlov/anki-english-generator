#[derive(Debug, Clone)]
pub struct WordBaseData {
  pub baseWord: String,
  pub level: String,
  pub partOfSpeech: Option<String>,
  pub wordDetailsUrl: String,
}

#[derive(Debug, Clone)]
pub struct WordMetadata {
  pub baseData: WordBaseData,
  pub guideWord: Option<String>,
  pub topic: Option<String>,
}

#[derive(Debug, Clone)]
pub struct WordFamily {
  pub partOfSpeech: String,
  pub words: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct LearnerExample {
  pub example: String,
  pub cite: Option<String>,
}

#[derive(Debug, Clone)]
pub struct WordInfoData {
  pub baseData: WordBaseData,
  pub headword: String,
  pub transcription: String,
  pub wordFamilies: Vec<WordFamily>,
  pub infoTitle: String,
  pub grammar: Option<String>,
  pub definition: String,
  pub dictionaryExamples: Vec<String>,
  pub learnerExample: Option<LearnerExample>,
}
