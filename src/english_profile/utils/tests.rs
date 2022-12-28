use fluid::prelude::*;

use super::*;

#[theory]
#[case(
  "https://ep.org/english/words/detail/1",
  "https://no-ep.org/wordlists/evp",
  "https://ep.org/english/words/detail/1"
)]
#[case(
  "/english/words/detail/2",
  "https://ep.org/wordlists/evp",
  "https://ep.org/english/words/detail/2"
)]
#[case(
  "/",
  "https://ep.org/wordlists/evp",
  "https://ep.org/"
)]
fn ensure_absolute_url_tests(url: &str, base_url: &str, expected: &str) {
  let result = ensure_absolute_url(url, base_url);
  assert_eq!(Some(expected.to_string()), result);
}
