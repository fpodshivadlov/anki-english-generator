use clap::Parser;

mod models;
mod file_utils;
mod english_profile;
//mod ankiCsv;

use models::TopicEnum;
use models::CerfLevelEnum;
use file_utils::fileExists;
use file_utils::loadWordListFileIfSet;
use english_profile::fetchEnglishProfile;

// ToDo: naming convention

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
  #[arg(short = 'a', long)]
  useAmerican: bool,

  #[arg(short = 'l', long, value_enum)]
  filterLevel: Option<Vec<CerfLevelEnum>>,

  #[arg(short = 't', long, value_enum)]
  filterTopic: Option<TopicEnum>,

  #[arg(short = 'w', long)]
  filterWordList: Option<String>,

  #[arg(short = 'o', long)]
  outputFile: Option<String>,
}

fn parse_args() -> Args {
  let args = Args::parse();

  // ToDo: review clone()
  if let Some(val) = args.filterWordList.clone() {
    if !fileExists(val.as_str()) {
      panic!("filterWordList: invalid filename {}", val.as_str());
    }
  }

  args
}

#[tokio::main]
async fn main() {
  let args = parse_args();

  let wordsFilter = loadWordListFileIfSet(&args.filterWordList.as_deref());

  let wordsDetails = fetchEnglishProfile(args.useAmerican, &args.filterLevel, &args.filterTopic, &wordsFilter).await;

  //createAnkiCsvImportFile(wordsDetails, argv.outputFile);
}
