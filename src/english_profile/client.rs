use reqwest::Client;

use crate::models::CerfLevelEnum;
use crate::models::TopicEnum;

pub async fn fetch_words_html(
  use_american: bool,
  level_filter: &Option<Vec<CerfLevelEnum>>,
  topic_filter: &Option<TopicEnum>) -> String
{
  let mut form = Vec::new();
  form.push(("limit", "0"));

  if let Some(values) = level_filter {
    values.iter().for_each(|level_value| {
      form.push(("filter_custom_Level[]", level_value.get_param_value()));
    });
  }

  if let Some(topic_value) = topic_filter {
    form.push(("filter_custom_Topic", topic_value.get_param_value()));
  }

  let url = match use_american
  {
      true => "https://www.englishprofile.org/american-english",
      false => "https://www.englishprofile.org/wordlists/evp",
  };

  let client = Client::new();
  // ToDo: restore caching
  println!(" fetching URL: {}", url);
  let response = client
      .post(url)
      .form(&form)
      .send()
      .await
      .expect("Unable to load page");

  response.text()
    .await
    .expect("Unable to receive body")
}

pub async fn fetch_word_details(details_url: &str) -> String
{
  let url = details_url;
  println!(" fetching URL: {}", url);

  let client = Client::new();
  // ToDo: restore caching
  let response = client
      .get(url)
      .send()
      .await
      .expect("Unable to load page");

  response.text()
    .await
    .expect("Unable to receive body")
}

pub trait EnglishProfileParam {
  fn get_param_value(&self) -> &str;
}

impl EnglishProfileParam for CerfLevelEnum {
  fn get_param_value(&self) -> &str {
      match self {
        CerfLevelEnum::A1 => "1",
        CerfLevelEnum::A2 => "2",
        CerfLevelEnum::B1 => "3",
        CerfLevelEnum::B2 => "4",
        CerfLevelEnum::C1 => "5",
        CerfLevelEnum::C2 => "6",
      }
  }
}


impl EnglishProfileParam for TopicEnum {
  fn get_param_value(&self) -> &str {
      match self {
        TopicEnum::Animals => "1",
        TopicEnum::ArtsAndMedia => "2",
        TopicEnum::BodyAndHealth => "3",
        TopicEnum::Clothes => "4",
        TopicEnum::Communication => "5",
        TopicEnum::Crime => "6",
        TopicEnum::DescribingThings => "7",
        TopicEnum::Education => "8",
        TopicEnum::FoodAndDrink => "9",
        TopicEnum::HomesAndBuildings => "10",
        TopicEnum::Money => "11",
        TopicEnum::NaturalWorld => "12",
        TopicEnum::PeopleActions => "13",
        TopicEnum::PeopleAppearance => "14",
        TopicEnum::PeoplePersonality => "15",
        TopicEnum::Politics => "16",
        TopicEnum::Relationships => "17",
        TopicEnum::Shopping => "18",
        TopicEnum::SportsAndGames => "19",
        TopicEnum::Technology => "20",
        TopicEnum::Travel => "21",
        TopicEnum::Work => "22",
      }
  }
}