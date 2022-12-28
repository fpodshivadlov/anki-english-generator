use log::info;
use std::fs;
use std::path::Path;

// import * as fs from 'fs';
// import * as path from "path";
// import { stringify } from 'csv-stringify';

// import { WordInfoData } from './english-profile';

// function getATag(wordDetails: WordInfoData) {
//   return `<a href='${wordDetails.wordDetailsUrl}'>${wordDetails.baseWord}</a>`;
// }

// function escapeRegExp(string: string) {
//   return string.replace(/[.*+?^${}()|[\]\\]/g, '\\$&'); // $& means the whole matched string
// }

// function convertWordDetailsToCsvRowItems(wordDetails: WordInfoData): string[] {
//   let linkAdded = false;
//   const regex = new RegExp(`(\\W|^)(${escapeRegExp(wordDetails.baseWord)})(\\W|$)`, "i");
//   const examples = wordDetails.dictionaryExamples.map((example, index) => {
//     let htmlExample = example;
//     if (example.match(regex)) {
//       linkAdded = true;
//       htmlExample = example.replace(
//         regex,
//         `$1<b>${getATag(wordDetails)}</b>$3`
//       );
//     }
//     if (index > 0) {
//       htmlExample = `[${htmlExample}]`;
//     }
//     return htmlExample;
//   });

  
//   const tags = [ wordDetails.level, wordDetails.partOfSpeech, wordDetails.grammar ]
//     .filter(x => x)
//     .join(',');
//   const front = wordDetails.definition;
//   let back = `${examples.join("<br>")}<br><br>[${wordDetails.transcription}]<br><br>`;;
//   if (wordDetails.wordFamilies.length) {
//     const family = wordDetails.wordFamilies
//       .map((family) => `<b>${family.partOfSpeech}</b> ${family.words.join("")}`)
//       .join("<br>");
//     back = `${back}[${family}]<br><br>`;
//   }

//   if (!linkAdded) {
//     back = `${back}[<b>${getATag(wordDetails)}</b>]<br><br>`;
//   }

//   return [front, back, tags];
// }

// function ensureDirExists(filename: string) {
//   const dir = path.dirname(filename)
//   if (!fs.existsSync(dir)){
//     fs.mkdirSync(dir, { recursive: true });
//   }
// }

fn prepageFileName(filename: String, extension: String) {
  let baseName = path.basename(filename, path.extname(filename));
  return path.join(path.dirname(filename), baseName + extension);
}

fn convertWordDetailsToCsvRowItems(wordDetails: WordInfoData) -> Vec<String> {
  // ToDo: migrate all the logic
  let tags = [ wordDetails.level, wordDetails.partOfSpeech, wordDetails.grammar ]
    .filter(|x| x != None)
    .join(',');
  let front = wordDetails.definition;
  let back = wordDetails.transcription;

  [front, back, tags]
}

pub async fn createAnkiCsvImportFile(words: Vec<WordInfoData>, filename: String) {
  info!("Creating anki file");

  let csvRecords = words.map(|word| {
    convertWordDetailsToCsvRowItems(word)
  });

  let fullFileName = prepageFileName(filename, ".csv");
  let mut wri = csv_async::AsyncWriter::from_writer(
    File::create(file_out).await?
  );

  // wri.write_record(record
  //   .iter()
  //   .enumerate()
  //   .filter(|(i, _)| *i != 1)
  //   .map(|(_, s)| s)
  // stringify(csvRecords, (err, output) => {
  //   ensureDirExists(fullFileName);
  //   fs.writeFileSync(fullFileName, output);
  // });

  info!(format!("Package has been generated: {}", fullFileName));
}
