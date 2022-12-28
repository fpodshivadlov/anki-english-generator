use clap::ValueEnum;

#[derive(Debug, Copy, Clone, PartialEq, Eq, ValueEnum)]
pub enum CerfLevelEnum {
  A1,
  A2,
  B1,
  B2,
  C1,
  C2,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, ValueEnum)]
pub enum TopicEnum {
  Animals,
  ArtsAndMedia,
  BodyAndHealth,
  Clothes,
  Communication,
  Crime,
  DescribingThings,
  Education,
  FoodAndDrink,
  HomesAndBuildings,
  Money,
  NaturalWorld,
  PeopleActions,
  PeopleAppearance,
  PeoplePersonality,
  Politics,
  Relationships,
  Shopping,
  SportsAndGames,
  Technology,
  Travel,
  Work,
}