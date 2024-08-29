//! This file is autogenerated by rs_generator.py.<br>Code in this file uses wynncraft item data from [wynnbuilder's github repo](https://raw.githubusercontent.com/hppeng-wynn/hppeng-wynn.github.io/dev/data/2.0.4.3/items.json)
pub (super) mod atree_data;use core::fmt;use crate::wynn_data::{WynnEnum,TryIntoWynnEnumError};use crate::enum_from_into;
#[derive(Clone,Default,PartialEq,PartialOrd,Eq,Ord,Copy,Debug)]
pub enum AtreeItems{#[default]Bash,SpearProficiency1,CheaperBash,DoubleBash,Charge,HeavyImpact,Vehement,TougherSkin,Uppercut,CheaperCharge,WarScream,EarthMastery,ThunderMastery,WaterMastery,AirMastery,FireMastery,QuadrupleBash,Fireworks,HalfmoonSwipe,FlybyJab,FlamingUppercut,IronLungs,Generalist,AirShout,MantleOfTheBovemists,BakalsGrasp,SpearProficiency2,CheaperUppercut,Aerodynamics,Provoke,Counter,Manachism,EnragedBlow,FlyingKick,StrongerMantle,SacredSurge,BoilingBlood,Ragnarokkr,PreciseStrikes,CleansingBreeze,StrongerBash,IntoxicatingBlood,Comet,Collide,RejuvenatingSkin,UncontainableCorruption,RadiantDevotee,WhirlwindStrike,MythrilSkin,ArmourBreaker,Riposte,ShieldStrike,SparklingHope,MassiveBash,Tempest,SpiritOfTheRabbit,Massacre,AxeKick,Radiance,CheaperBash2,CheaperWarScreamI,CheaperWarScreamIi,BetterEnragedBlow,Discombobulate,Thunderclap,Cyclone,StrongerSacredSurge,SecondChance,BloodPact,BrinkOfMadness,CheaperUppercut2,Martyr,Haemorrhage}
impl WynnEnum for AtreeItems{const VARIENTS:&'static[Self]=&[Self::Bash,Self::SpearProficiency1,Self::CheaperBash,Self::DoubleBash,Self::Charge,Self::HeavyImpact,Self::Vehement,Self::TougherSkin,Self::Uppercut,Self::CheaperCharge,Self::WarScream,Self::EarthMastery,Self::ThunderMastery,Self::WaterMastery,Self::AirMastery,Self::FireMastery,Self::QuadrupleBash,Self::Fireworks,Self::HalfmoonSwipe,Self::FlybyJab,Self::FlamingUppercut,Self::IronLungs,Self::Generalist,Self::AirShout,Self::MantleOfTheBovemists,Self::BakalsGrasp,Self::SpearProficiency2,Self::CheaperUppercut,Self::Aerodynamics,Self::Provoke,Self::Counter,Self::Manachism,Self::EnragedBlow,Self::FlyingKick,Self::StrongerMantle,Self::SacredSurge,Self::BoilingBlood,Self::Ragnarokkr,Self::PreciseStrikes,Self::CleansingBreeze,Self::StrongerBash,Self::IntoxicatingBlood,Self::Comet,Self::Collide,Self::RejuvenatingSkin,Self::UncontainableCorruption,Self::RadiantDevotee,Self::WhirlwindStrike,Self::MythrilSkin,Self::ArmourBreaker,Self::Riposte,Self::ShieldStrike,Self::SparklingHope,Self::MassiveBash,Self::Tempest,Self::SpiritOfTheRabbit,Self::Massacre,Self::AxeKick,Self::Radiance,Self::CheaperBash2,Self::CheaperWarScreamI,Self::CheaperWarScreamIi,Self::BetterEnragedBlow,Self::Discombobulate,Self::Thunderclap,Self::Cyclone,Self::StrongerSacredSurge,Self::SecondChance,Self::BloodPact,Self::BrinkOfMadness,Self::CheaperUppercut2,Self::Martyr,Self::Haemorrhage];}impl std::convert::TryFrom<u8> for AtreeItems{type Error=TryIntoWynnEnumError<u8,Self>;fn try_from(n: u8) -> Result<Self,Self::Error> {match n{0 => Ok(AtreeItems::Bash), 1 => Ok(AtreeItems::SpearProficiency1), 2 => Ok(AtreeItems::CheaperBash), 3 => Ok(AtreeItems::DoubleBash), 4 => Ok(AtreeItems::Charge), 5 => Ok(AtreeItems::HeavyImpact), 6 => Ok(AtreeItems::Vehement), 7 => Ok(AtreeItems::TougherSkin), 8 => Ok(AtreeItems::Uppercut), 9 => Ok(AtreeItems::CheaperCharge), 10 => Ok(AtreeItems::WarScream), 11 => Ok(AtreeItems::EarthMastery), 12 => Ok(AtreeItems::ThunderMastery), 13 => Ok(AtreeItems::WaterMastery), 14 => Ok(AtreeItems::AirMastery), 15 => Ok(AtreeItems::FireMastery), 16 => Ok(AtreeItems::QuadrupleBash), 17 => Ok(AtreeItems::Fireworks), 18 => Ok(AtreeItems::HalfmoonSwipe), 19 => Ok(AtreeItems::FlybyJab), 20 => Ok(AtreeItems::FlamingUppercut), 21 => Ok(AtreeItems::IronLungs), 22 => Ok(AtreeItems::Generalist), 23 => Ok(AtreeItems::AirShout), 24 => Ok(AtreeItems::MantleOfTheBovemists), 25 => Ok(AtreeItems::BakalsGrasp), 26 => Ok(AtreeItems::SpearProficiency2), 27 => Ok(AtreeItems::CheaperUppercut), 28 => Ok(AtreeItems::Aerodynamics), 29 => Ok(AtreeItems::Provoke), 30 => Ok(AtreeItems::Counter), 31 => Ok(AtreeItems::Manachism), 32 => Ok(AtreeItems::EnragedBlow), 33 => Ok(AtreeItems::FlyingKick), 34 => Ok(AtreeItems::StrongerMantle), 35 => Ok(AtreeItems::SacredSurge), 36 => Ok(AtreeItems::BoilingBlood), 37 => Ok(AtreeItems::Ragnarokkr), 38 => Ok(AtreeItems::PreciseStrikes), 39 => Ok(AtreeItems::CleansingBreeze), 40 => Ok(AtreeItems::StrongerBash), 41 => Ok(AtreeItems::IntoxicatingBlood), 42 => Ok(AtreeItems::Comet), 43 => Ok(AtreeItems::Collide), 44 => Ok(AtreeItems::RejuvenatingSkin), 45 => Ok(AtreeItems::UncontainableCorruption), 46 => Ok(AtreeItems::RadiantDevotee), 47 => Ok(AtreeItems::WhirlwindStrike), 48 => Ok(AtreeItems::MythrilSkin), 49 => Ok(AtreeItems::ArmourBreaker), 50 => Ok(AtreeItems::Riposte), 51 => Ok(AtreeItems::ShieldStrike), 52 => Ok(AtreeItems::SparklingHope), 53 => Ok(AtreeItems::MassiveBash), 54 => Ok(AtreeItems::Tempest), 55 => Ok(AtreeItems::SpiritOfTheRabbit), 56 => Ok(AtreeItems::Massacre), 57 => Ok(AtreeItems::AxeKick), 58 => Ok(AtreeItems::Radiance), 59 => Ok(AtreeItems::CheaperBash2), 60 => Ok(AtreeItems::CheaperWarScreamI), 61 => Ok(AtreeItems::CheaperWarScreamIi), 62 => Ok(AtreeItems::BetterEnragedBlow), 63 => Ok(AtreeItems::Discombobulate), 64 => Ok(AtreeItems::Thunderclap), 65 => Ok(AtreeItems::Cyclone), 66 => Ok(AtreeItems::StrongerSacredSurge), 67 => Ok(AtreeItems::SecondChance), 68 => Ok(AtreeItems::BloodPact), 69 => Ok(AtreeItems::BrinkOfMadness), 70 => Ok(AtreeItems::CheaperUppercut2), 71 => Ok(AtreeItems::Martyr), 72 => Ok(AtreeItems::Haemorrhage), _ => Err(TryIntoWynnEnumError{from: n, to: Self::default()})}}}
impl fmt::Display for AtreeItems{fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {write!(f,"{}",match self{AtreeItems::Bash => "Bash", AtreeItems::SpearProficiency1 => "Spear Proficiency 1", AtreeItems::CheaperBash => "Cheaper Bash", AtreeItems::DoubleBash => "Double Bash", AtreeItems::Charge => "Charge", AtreeItems::HeavyImpact => "Heavy Impact", AtreeItems::Vehement => "Vehement", AtreeItems::TougherSkin => "Tougher Skin", AtreeItems::Uppercut => "Uppercut", AtreeItems::CheaperCharge => "Cheaper Charge", AtreeItems::WarScream => "War Scream", AtreeItems::EarthMastery => "Earth Mastery", AtreeItems::ThunderMastery => "Thunder Mastery", AtreeItems::WaterMastery => "Water Mastery", AtreeItems::AirMastery => "Air Mastery", AtreeItems::FireMastery => "Fire Mastery", AtreeItems::QuadrupleBash => "Quadruple Bash", AtreeItems::Fireworks => "Fireworks", AtreeItems::HalfmoonSwipe => "Half-Moon Swipe", AtreeItems::FlybyJab => "Flyby Jab", AtreeItems::FlamingUppercut => "Flaming Uppercut", AtreeItems::IronLungs => "Iron Lungs", AtreeItems::Generalist => "Generalist", AtreeItems::AirShout => "Air Shout", AtreeItems::MantleOfTheBovemists => "Mantle of the Bovemists", AtreeItems::BakalsGrasp => "Bak'al's Grasp", AtreeItems::SpearProficiency2 => "Spear Proficiency 2", AtreeItems::CheaperUppercut => "Cheaper Uppercut", AtreeItems::Aerodynamics => "Aerodynamics", AtreeItems::Provoke => "Provoke", AtreeItems::Counter => "Counter", AtreeItems::Manachism => "Manachism", AtreeItems::EnragedBlow => "Enraged Blow", AtreeItems::FlyingKick => "Flying Kick", AtreeItems::StrongerMantle => "Stronger Mantle", AtreeItems::SacredSurge => "Sacred Surge", AtreeItems::BoilingBlood => "Boiling Blood", AtreeItems::Ragnarokkr => "Ragnarokkr", AtreeItems::PreciseStrikes => "Precise Strikes", AtreeItems::CleansingBreeze => "Cleansing Breeze", AtreeItems::StrongerBash => "Stronger Bash", AtreeItems::IntoxicatingBlood => "Intoxicating Blood", AtreeItems::Comet => "Comet", AtreeItems::Collide => "Collide", AtreeItems::RejuvenatingSkin => "Rejuvenating Skin", AtreeItems::UncontainableCorruption => "Uncontainable Corruption", AtreeItems::RadiantDevotee => "Radiant Devotee", AtreeItems::WhirlwindStrike => "Whirlwind Strike", AtreeItems::MythrilSkin => "Mythril Skin", AtreeItems::ArmourBreaker => "Armour Breaker", AtreeItems::Riposte => "Riposte", AtreeItems::ShieldStrike => "Shield Strike", AtreeItems::SparklingHope => "Sparkling Hope", AtreeItems::MassiveBash => "Massive Bash", AtreeItems::Tempest => "Tempest", AtreeItems::SpiritOfTheRabbit => "Spirit of the Rabbit", AtreeItems::Massacre => "Massacre", AtreeItems::AxeKick => "Axe Kick", AtreeItems::Radiance => "Radiance", AtreeItems::CheaperBash2 => "Cheaper Bash 2", AtreeItems::CheaperWarScreamI => "Cheaper War Scream I", AtreeItems::CheaperWarScreamIi => "Cheaper War Scream II", AtreeItems::BetterEnragedBlow => "Better Enraged Blow", AtreeItems::Discombobulate => "Discombobulate", AtreeItems::Thunderclap => "Thunderclap", AtreeItems::Cyclone => "Cyclone", AtreeItems::StrongerSacredSurge => "Stronger Sacred Surge", AtreeItems::SecondChance => "Second Chance", AtreeItems::BloodPact => "Blood Pact", AtreeItems::BrinkOfMadness => "Brink of Madness", AtreeItems::CheaperUppercut2 => "Cheaper Uppercut 2", AtreeItems::Martyr => "Martyr", AtreeItems::Haemorrhage => "Haemorrhage", })}}enum_from_into!(AtreeItems, u8,u32,u64,i32,i64,usize);
#[derive(Clone,Default,PartialEq,PartialOrd,Eq,Ord,Copy,Debug)]
pub enum Prop{#[default]Aoe,Range,Hits,MeleeRange,Angle,Duration,DefenseBonus,Tick,AttackRate,Charges,Cooldown,Chance,HealthCost}
impl WynnEnum for Prop{const VARIENTS:&'static[Self]=&[Self::Aoe,Self::Range,Self::Hits,Self::MeleeRange,Self::Angle,Self::Duration,Self::DefenseBonus,Self::Tick,Self::AttackRate,Self::Charges,Self::Cooldown,Self::Chance,Self::HealthCost];}impl std::convert::TryFrom<u8> for Prop{type Error=TryIntoWynnEnumError<u8,Self>;fn try_from(n: u8) -> Result<Self,Self::Error> {match n{0 => Ok(Prop::Aoe), 1 => Ok(Prop::Range), 2 => Ok(Prop::Hits), 3 => Ok(Prop::MeleeRange), 4 => Ok(Prop::Angle), 5 => Ok(Prop::Duration), 6 => Ok(Prop::DefenseBonus), 7 => Ok(Prop::Tick), 8 => Ok(Prop::AttackRate), 9 => Ok(Prop::Charges), 10 => Ok(Prop::Cooldown), 11 => Ok(Prop::Chance), 12 => Ok(Prop::HealthCost), _ => Err(TryIntoWynnEnumError{from: n, to: Self::default()})}}}
impl fmt::Display for Prop{fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {write!(f,"{}",match self{Prop::Aoe => "Aoe", Prop::Range => "Range", Prop::Hits => "Hits", Prop::MeleeRange => "MeleeRange", Prop::Angle => "Angle", Prop::Duration => "Duration", Prop::DefenseBonus => "DefenseBonus", Prop::Tick => "Tick", Prop::AttackRate => "AttackRate", Prop::Charges => "Charges", Prop::Cooldown => "Cooldown", Prop::Chance => "Chance", Prop::HealthCost => "HealthCost", })}}enum_from_into!(Prop, u8,u32,u64,i32,i64,usize);
#[derive(Clone,Default,PartialEq,PartialOrd,Eq,Ord,Copy,Debug)]
pub enum Spell{#[default]Melee,Bash,Charge,Uppercut,WarScream,FlamingUppercut,Counter,SacredSurge,ShieldStrike,SparklingHope}
impl WynnEnum for Spell{const VARIENTS:&'static[Self]=&[Self::Melee,Self::Bash,Self::Charge,Self::Uppercut,Self::WarScream,Self::FlamingUppercut,Self::Counter,Self::SacredSurge,Self::ShieldStrike,Self::SparklingHope];}impl std::convert::TryFrom<u8> for Spell{type Error=TryIntoWynnEnumError<u8,Self>;fn try_from(n: u8) -> Result<Self,Self::Error> {match n{0 => Ok(Spell::Melee), 1 => Ok(Spell::Bash), 2 => Ok(Spell::Charge), 3 => Ok(Spell::Uppercut), 4 => Ok(Spell::WarScream), 5 => Ok(Spell::FlamingUppercut), 6 => Ok(Spell::Counter), 7 => Ok(Spell::SacredSurge), 8 => Ok(Spell::ShieldStrike), 9 => Ok(Spell::SparklingHope), _ => Err(TryIntoWynnEnumError{from: n, to: Self::default()})}}}
impl fmt::Display for Spell{fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {write!(f,"{}",match self{Spell::Melee => "Melee", Spell::Bash => "Bash", Spell::Charge => "Charge", Spell::Uppercut => "Uppercut", Spell::WarScream => "WarScream", Spell::FlamingUppercut => "FlamingUppercut", Spell::Counter => "Counter", Spell::SacredSurge => "SacredSurge", Spell::ShieldStrike => "ShieldStrike", Spell::SparklingHope => "SparklingHope", })}}enum_from_into!(Spell, u8,u32,u64,i32,i64,usize);
#[derive(Clone,Default,PartialEq,PartialOrd,Eq,Ord,Copy,Debug)]
pub enum SpellPart{#[default]None,Total,Dps,TotalDps,SingleHit,Uppercut,WarScream,DamageTick,CounterDamage,SmiteDamage,DamagePerShield,Melee,HeavyImpact,ContactDamage,Fireworks,FlybyJab,AirShout,FlyingKick,FlyingKickMaxDamage,BoilingBlood,Comet,Collide,Tempest,TempestTotalDamage,Cyclone,CycloneTotalDamage}
impl WynnEnum for SpellPart{const VARIENTS:&'static[Self]=&[Self::None,Self::Total,Self::Dps,Self::TotalDps,Self::SingleHit,Self::Uppercut,Self::WarScream,Self::DamageTick,Self::CounterDamage,Self::SmiteDamage,Self::DamagePerShield,Self::Melee,Self::HeavyImpact,Self::ContactDamage,Self::Fireworks,Self::FlybyJab,Self::AirShout,Self::FlyingKick,Self::FlyingKickMaxDamage,Self::BoilingBlood,Self::Comet,Self::Collide,Self::Tempest,Self::TempestTotalDamage,Self::Cyclone,Self::CycloneTotalDamage];}impl std::convert::TryFrom<u8> for SpellPart{type Error=TryIntoWynnEnumError<u8,Self>;fn try_from(n: u8) -> Result<Self,Self::Error> {match n{0 => Ok(SpellPart::None), 1 => Ok(SpellPart::Total), 2 => Ok(SpellPart::Dps), 3 => Ok(SpellPart::TotalDps), 4 => Ok(SpellPart::SingleHit), 5 => Ok(SpellPart::Uppercut), 6 => Ok(SpellPart::WarScream), 7 => Ok(SpellPart::DamageTick), 8 => Ok(SpellPart::CounterDamage), 9 => Ok(SpellPart::SmiteDamage), 10 => Ok(SpellPart::DamagePerShield), 11 => Ok(SpellPart::Melee), 12 => Ok(SpellPart::HeavyImpact), 13 => Ok(SpellPart::ContactDamage), 14 => Ok(SpellPart::Fireworks), 15 => Ok(SpellPart::FlybyJab), 16 => Ok(SpellPart::AirShout), 17 => Ok(SpellPart::FlyingKick), 18 => Ok(SpellPart::FlyingKickMaxDamage), 19 => Ok(SpellPart::BoilingBlood), 20 => Ok(SpellPart::Comet), 21 => Ok(SpellPart::Collide), 22 => Ok(SpellPart::Tempest), 23 => Ok(SpellPart::TempestTotalDamage), 24 => Ok(SpellPart::Cyclone), 25 => Ok(SpellPart::CycloneTotalDamage), _ => Err(TryIntoWynnEnumError{from: n, to: Self::default()})}}}
impl fmt::Display for SpellPart{fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {write!(f,"{}",match self{SpellPart::None => "None", SpellPart::Total => "Total", SpellPart::Dps => "Dps", SpellPart::TotalDps => "TotalDps", SpellPart::SingleHit => "SingleHit", SpellPart::Uppercut => "Uppercut", SpellPart::WarScream => "WarScream", SpellPart::DamageTick => "DamageTick", SpellPart::CounterDamage => "CounterDamage", SpellPart::SmiteDamage => "SmiteDamage", SpellPart::DamagePerShield => "DamagePerShield", SpellPart::Melee => "Melee", SpellPart::HeavyImpact => "HeavyImpact", SpellPart::ContactDamage => "ContactDamage", SpellPart::Fireworks => "Fireworks", SpellPart::FlybyJab => "FlybyJab", SpellPart::AirShout => "AirShout", SpellPart::FlyingKick => "FlyingKick", SpellPart::FlyingKickMaxDamage => "FlyingKickMaxDamage", SpellPart::BoilingBlood => "BoilingBlood", SpellPart::Comet => "Comet", SpellPart::Collide => "Collide", SpellPart::Tempest => "Tempest", SpellPart::TempestTotalDamage => "TempestTotalDamage", SpellPart::Cyclone => "Cyclone", SpellPart::CycloneTotalDamage => "CycloneTotalDamage", })}}enum_from_into!(SpellPart, u8,u32,u64,i32,i64,usize);