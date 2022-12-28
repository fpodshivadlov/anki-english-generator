use fluid::prelude::*;
use std::fs;

use super::*;

#[theory]
#[case("https://www.englishprofile.org/british-english/words/detail/822", true)]
#[case("https://www.englishprofile.org/american-english/words/usdetail/", false)]
fn is_valid_details_url_tests(url: &str, expected_result: bool) {
  let result = is_valid_details_url(url);
  assert_eq!(expected_result, result, "'{}' should be {}", url, expected_result);
}

#[test]
fn parse_list_html_tests() {
  let contents = include_str!("tests_words_list.html");

  let result = parse_list_html(contents);
  assert_eq!(6, result.len());

  let first_result = result.iter().next().unwrap();
  assert_eq!("and", first_result.baseData.baseWord);
  assert_eq!("https://www.englishprofile.org/british-english/words/detail/229", first_result.baseData.wordDetailsUrl);
  assert_eq!(None, first_result.baseData.partOfSpeech);
  assert_eq!("A1", first_result.baseData.level);
  assert_eq!(Some("ALSO".to_string()), first_result.guideWord);
  assert_eq!(Some("communication".to_string()), first_result.topic);

  let sixth_result = result.iter().skip(5).next().unwrap();
  assert_eq!("now and then", sixth_result.baseData.baseWord);
  assert_eq!("https://www.englishprofile.org/british-english/words/detail/3899", sixth_result.baseData.wordDetailsUrl);
  assert_eq!(Some("phrase".to_string()), sixth_result.baseData.partOfSpeech);
  assert_eq!("B2", sixth_result.baseData.level);
  assert_eq!(None, sixth_result.guideWord);
  assert_eq!(None, sixth_result.topic);
}
