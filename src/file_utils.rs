use std;
use std::fs;
use std::path::Path;

pub fn fileExists(filePath: &str) -> bool {
  let exists = Path::new(filePath).exists();
  exists
}

pub fn loadWordListFileIfSet(filePath: &Option<&str>) -> Option<Vec<String>> {
  if filePath.is_none() {
    return None;
  }

  let filePath = filePath.unwrap();
  if !fileExists(&filePath) {
    panic!("file {} is not found", &filePath);
  }

  let contents = fs::read_to_string(filePath)
    .expect("Should have been able to read the file");

  let words = contents
    .split("\n")
    .map(|x| x.trim().to_owned())
    .collect::<Vec<String>>();

  Some(words)
}
