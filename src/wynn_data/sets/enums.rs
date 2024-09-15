//! This file is autogenerated by rs_generator.py.<br>Code in this file uses wynncraft item data from [wynnbuilder's github repo](https://raw.githubusercontent.com/hppeng-wynn/hppeng-wynn.github.io/dev/data/2.0.4.3/items.json)
use core::fmt;use crate::wynn_data::{WynnEnum,TryIntoWynnEnumError};use crate::enum_from_into;
#[derive(Clone,Default,PartialEq,PartialOrd,Eq,Ord,Copy,Debug)]
pub enum Sets{#[default]None,Boundless,OrnateShadow,Grookwarts,MasterHive,ThunderHive,AirHive,EarthHive,WaterHive,FireHive,SynchCore,Black,RedTeam,Tribal,Champion,Outlaw,Snail,ThanosLegionnaire,Ghostly,Adventurers,AirRelic,Spider,Pigman,Kaerynns,Bandits,Jester,Silverfish,Skiens,Snow,Veekhats,Morph,BlackCatalyst,Leaf,Vexing,Hallowynn2016,Spore,Horse,Nether,ThunderRelic,Visceral,Bony,BlueTeam,Clock,Ultramarine,Cosmic,Saints,Beachside,Villager,Goblin,CorruptedNii,WaterRelic,Elf,Relic,CorruptedUth,FireRelic,Flashfire,EarthRelic,Bear,Slime,Wynnterfest2016}
impl WynnEnum for Sets{const VARIENTS:&'static[Self]=&[Self::None,Self::Boundless,Self::OrnateShadow,Self::Grookwarts,Self::MasterHive,Self::ThunderHive,Self::AirHive,Self::EarthHive,Self::WaterHive,Self::FireHive,Self::SynchCore,Self::Black,Self::RedTeam,Self::Tribal,Self::Champion,Self::Outlaw,Self::Snail,Self::ThanosLegionnaire,Self::Ghostly,Self::Adventurers,Self::AirRelic,Self::Spider,Self::Pigman,Self::Kaerynns,Self::Bandits,Self::Jester,Self::Silverfish,Self::Skiens,Self::Snow,Self::Veekhats,Self::Morph,Self::BlackCatalyst,Self::Leaf,Self::Vexing,Self::Hallowynn2016,Self::Spore,Self::Horse,Self::Nether,Self::ThunderRelic,Self::Visceral,Self::Bony,Self::BlueTeam,Self::Clock,Self::Ultramarine,Self::Cosmic,Self::Saints,Self::Beachside,Self::Villager,Self::Goblin,Self::CorruptedNii,Self::WaterRelic,Self::Elf,Self::Relic,Self::CorruptedUth,Self::FireRelic,Self::Flashfire,Self::EarthRelic,Self::Bear,Self::Slime,Self::Wynnterfest2016];const ENUM_TYPE_ID:u8=10;}impl std::convert::TryFrom<u8> for Sets{type Error=TryIntoWynnEnumError<u8,Self>;fn try_from(n: u8) -> Result<Self,Self::Error> {match n{0 => Ok(Sets::None), 1 => Ok(Sets::Boundless), 2 => Ok(Sets::OrnateShadow), 3 => Ok(Sets::Grookwarts), 4 => Ok(Sets::MasterHive), 5 => Ok(Sets::ThunderHive), 6 => Ok(Sets::AirHive), 7 => Ok(Sets::EarthHive), 8 => Ok(Sets::WaterHive), 9 => Ok(Sets::FireHive), 10 => Ok(Sets::SynchCore), 11 => Ok(Sets::Black), 12 => Ok(Sets::RedTeam), 13 => Ok(Sets::Tribal), 14 => Ok(Sets::Champion), 15 => Ok(Sets::Outlaw), 16 => Ok(Sets::Snail), 17 => Ok(Sets::ThanosLegionnaire), 18 => Ok(Sets::Ghostly), 19 => Ok(Sets::Adventurers), 20 => Ok(Sets::AirRelic), 21 => Ok(Sets::Spider), 22 => Ok(Sets::Pigman), 23 => Ok(Sets::Kaerynns), 24 => Ok(Sets::Bandits), 25 => Ok(Sets::Jester), 26 => Ok(Sets::Silverfish), 27 => Ok(Sets::Skiens), 28 => Ok(Sets::Snow), 29 => Ok(Sets::Veekhats), 30 => Ok(Sets::Morph), 31 => Ok(Sets::BlackCatalyst), 32 => Ok(Sets::Leaf), 33 => Ok(Sets::Vexing), 34 => Ok(Sets::Hallowynn2016), 35 => Ok(Sets::Spore), 36 => Ok(Sets::Horse), 37 => Ok(Sets::Nether), 38 => Ok(Sets::ThunderRelic), 39 => Ok(Sets::Visceral), 40 => Ok(Sets::Bony), 41 => Ok(Sets::BlueTeam), 42 => Ok(Sets::Clock), 43 => Ok(Sets::Ultramarine), 44 => Ok(Sets::Cosmic), 45 => Ok(Sets::Saints), 46 => Ok(Sets::Beachside), 47 => Ok(Sets::Villager), 48 => Ok(Sets::Goblin), 49 => Ok(Sets::CorruptedNii), 50 => Ok(Sets::WaterRelic), 51 => Ok(Sets::Elf), 52 => Ok(Sets::Relic), 53 => Ok(Sets::CorruptedUth), 54 => Ok(Sets::FireRelic), 55 => Ok(Sets::Flashfire), 56 => Ok(Sets::EarthRelic), 57 => Ok(Sets::Bear), 58 => Ok(Sets::Slime), 59 => Ok(Sets::Wynnterfest2016), _ => Err(TryIntoWynnEnumError{from: n, to: Self::default()})}}}
impl fmt::Display for Sets{fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {write!(f,"{}",match self{Sets::None => "None", Sets::Boundless => "Boundless", Sets::OrnateShadow => "Ornate Shadow", Sets::Grookwarts => "Grookwarts", Sets::MasterHive => "Master Hive", Sets::ThunderHive => "Thunder Hive", Sets::AirHive => "Air Hive", Sets::EarthHive => "Earth Hive", Sets::WaterHive => "Water Hive", Sets::FireHive => "Fire Hive", Sets::SynchCore => "Synch Core", Sets::Black => "Black", Sets::RedTeam => "Red Team", Sets::Tribal => "Tribal", Sets::Champion => "Champion", Sets::Outlaw => "Outlaw", Sets::Snail => "Snail", Sets::ThanosLegionnaire => "Thanos Legionnaire", Sets::Ghostly => "Ghostly", Sets::Adventurers => "Adventurer's", Sets::AirRelic => "Air Relic", Sets::Spider => "Spider", Sets::Pigman => "Pigman", Sets::Kaerynns => "Kaerynn's", Sets::Bandits => "Bandit's", Sets::Jester => "Jester", Sets::Silverfish => "Silverfish", Sets::Skiens => "Skien's", Sets::Snow => "Snow", Sets::Veekhats => "Veekhat's", Sets::Morph => "Morph", Sets::BlackCatalyst => "Black Catalyst", Sets::Leaf => "Leaf", Sets::Vexing => "Vexing", Sets::Hallowynn2016 => "Hallowynn 2016", Sets::Spore => "Spore", Sets::Horse => "Horse", Sets::Nether => "Nether", Sets::ThunderRelic => "Thunder Relic", Sets::Visceral => "Visceral", Sets::Bony => "Bony", Sets::BlueTeam => "Blue Team", Sets::Clock => "Clock", Sets::Ultramarine => "Ultramarine", Sets::Cosmic => "Cosmic", Sets::Saints => "Saint's", Sets::Beachside => "Beachside", Sets::Villager => "Villager", Sets::Goblin => "Goblin", Sets::CorruptedNii => "Corrupted Nii", Sets::WaterRelic => "Water Relic", Sets::Elf => "Elf", Sets::Relic => "Relic", Sets::CorruptedUth => "Corrupted Uth", Sets::FireRelic => "Fire Relic", Sets::Flashfire => "Flashfire", Sets::EarthRelic => "Earth Relic", Sets::Bear => "Bear", Sets::Slime => "Slime", Sets::Wynnterfest2016 => "Wynnterfest 2016", })}}enum_from_into!(Sets, u8,u32,u64,i32,i64,usize);