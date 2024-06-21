//! This file is autogenerated by rs_generator.py.<br>Code in this file uses wynncraft item data from [wynnbuilder's github repo](https://raw.githubusercontent.com/hppeng-wynn/hppeng-wynn.github.io/dev/data/2.0.4.3/items.json)
pub (super) mod atree_data;use core::fmt;use crate::wynn_data::{WynnEnum,TryIntoWynnEnumError};use crate::enum_from_into;
#[derive(Clone,Default,PartialEq,PartialOrd,Eq,Ord,Copy,Debug)]
pub enum AtreeItems{#[default]Meteor,WandProficiencyI,CheaperMeteor,ShootingStar,WandProficiencyIi,Teleport,Wisdom,Heal,CheaperTeleport,IceSnake,AirMastery,ThunderMastery,WaterMastery,FireMastery,EarthMastery,WindSlash,Thunderstorm,BurningSigil,Sunshower,StrongerMeteor,Windsweeper,Ophanim,ArcaneTransfer,CheaperHeal,Purification,SentientSnake,EyePiercer,Breathless,LargerHeal,LargerManaBank,CheaperIceSnake,Fortitude,CheaperTeleportIi,Pyrokinesis,Blink,HealthierOphanimI,SnakeNest,Seance,TransonicWarp,FluidHealing,OrphionsPulse,ArcaneRestoration,StrongerOphanim,Diffusion,Lightweaver,ArcaneSpeed,LargerManaBankIi,MoreWinded,Psychokinesis,CheaperIceSnakeIi,ExplosiveEntrance,CheaperMeteorIi,ChaosExplosion,TimeDilation,Gust,BetterOphanim,ArcticSnake,StrongerSunshower,ArcanePower,MoreWindedIi,DynamicFaith,HealthierOphanimIi,LargerManaBankIii,Devitalize,Divination,Sunflare,BetterLightweaver,ArcaneOverflow,Timelock,CheaperHealIi,Manastorm,MemoryRecollection}
impl WynnEnum for AtreeItems{const VARIENTS:&'static[Self]=&[Self::Meteor,Self::WandProficiencyI,Self::CheaperMeteor,Self::ShootingStar,Self::WandProficiencyIi,Self::Teleport,Self::Wisdom,Self::Heal,Self::CheaperTeleport,Self::IceSnake,Self::AirMastery,Self::ThunderMastery,Self::WaterMastery,Self::FireMastery,Self::EarthMastery,Self::WindSlash,Self::Thunderstorm,Self::BurningSigil,Self::Sunshower,Self::StrongerMeteor,Self::Windsweeper,Self::Ophanim,Self::ArcaneTransfer,Self::CheaperHeal,Self::Purification,Self::SentientSnake,Self::EyePiercer,Self::Breathless,Self::LargerHeal,Self::LargerManaBank,Self::CheaperIceSnake,Self::Fortitude,Self::CheaperTeleportIi,Self::Pyrokinesis,Self::Blink,Self::HealthierOphanimI,Self::SnakeNest,Self::Seance,Self::TransonicWarp,Self::FluidHealing,Self::OrphionsPulse,Self::ArcaneRestoration,Self::StrongerOphanim,Self::Diffusion,Self::Lightweaver,Self::ArcaneSpeed,Self::LargerManaBankIi,Self::MoreWinded,Self::Psychokinesis,Self::CheaperIceSnakeIi,Self::ExplosiveEntrance,Self::CheaperMeteorIi,Self::ChaosExplosion,Self::TimeDilation,Self::Gust,Self::BetterOphanim,Self::ArcticSnake,Self::StrongerSunshower,Self::ArcanePower,Self::MoreWindedIi,Self::DynamicFaith,Self::HealthierOphanimIi,Self::LargerManaBankIii,Self::Devitalize,Self::Divination,Self::Sunflare,Self::BetterLightweaver,Self::ArcaneOverflow,Self::Timelock,Self::CheaperHealIi,Self::Manastorm,Self::MemoryRecollection];}impl std::convert::TryFrom<u8> for AtreeItems{type Error=TryIntoWynnEnumError<u8,Self>;fn try_from(n: u8) -> Result<Self,Self::Error> {match n{0 => Ok(AtreeItems::Meteor), 1 => Ok(AtreeItems::WandProficiencyI), 2 => Ok(AtreeItems::CheaperMeteor), 3 => Ok(AtreeItems::ShootingStar), 4 => Ok(AtreeItems::WandProficiencyIi), 5 => Ok(AtreeItems::Teleport), 6 => Ok(AtreeItems::Wisdom), 7 => Ok(AtreeItems::Heal), 8 => Ok(AtreeItems::CheaperTeleport), 9 => Ok(AtreeItems::IceSnake), 10 => Ok(AtreeItems::AirMastery), 11 => Ok(AtreeItems::ThunderMastery), 12 => Ok(AtreeItems::WaterMastery), 13 => Ok(AtreeItems::FireMastery), 14 => Ok(AtreeItems::EarthMastery), 15 => Ok(AtreeItems::WindSlash), 16 => Ok(AtreeItems::Thunderstorm), 17 => Ok(AtreeItems::BurningSigil), 18 => Ok(AtreeItems::Sunshower), 19 => Ok(AtreeItems::StrongerMeteor), 20 => Ok(AtreeItems::Windsweeper), 21 => Ok(AtreeItems::Ophanim), 22 => Ok(AtreeItems::ArcaneTransfer), 23 => Ok(AtreeItems::CheaperHeal), 24 => Ok(AtreeItems::Purification), 25 => Ok(AtreeItems::SentientSnake), 26 => Ok(AtreeItems::EyePiercer), 27 => Ok(AtreeItems::Breathless), 28 => Ok(AtreeItems::LargerHeal), 29 => Ok(AtreeItems::LargerManaBank), 30 => Ok(AtreeItems::CheaperIceSnake), 31 => Ok(AtreeItems::Fortitude), 32 => Ok(AtreeItems::CheaperTeleportIi), 33 => Ok(AtreeItems::Pyrokinesis), 34 => Ok(AtreeItems::Blink), 35 => Ok(AtreeItems::HealthierOphanimI), 36 => Ok(AtreeItems::SnakeNest), 37 => Ok(AtreeItems::Seance), 38 => Ok(AtreeItems::TransonicWarp), 39 => Ok(AtreeItems::FluidHealing), 40 => Ok(AtreeItems::OrphionsPulse), 41 => Ok(AtreeItems::ArcaneRestoration), 42 => Ok(AtreeItems::StrongerOphanim), 43 => Ok(AtreeItems::Diffusion), 44 => Ok(AtreeItems::Lightweaver), 45 => Ok(AtreeItems::ArcaneSpeed), 46 => Ok(AtreeItems::LargerManaBankIi), 47 => Ok(AtreeItems::MoreWinded), 48 => Ok(AtreeItems::Psychokinesis), 49 => Ok(AtreeItems::CheaperIceSnakeIi), 50 => Ok(AtreeItems::ExplosiveEntrance), 51 => Ok(AtreeItems::CheaperMeteorIi), 52 => Ok(AtreeItems::ChaosExplosion), 53 => Ok(AtreeItems::TimeDilation), 54 => Ok(AtreeItems::Gust), 55 => Ok(AtreeItems::BetterOphanim), 56 => Ok(AtreeItems::ArcticSnake), 57 => Ok(AtreeItems::StrongerSunshower), 58 => Ok(AtreeItems::ArcanePower), 59 => Ok(AtreeItems::MoreWindedIi), 60 => Ok(AtreeItems::DynamicFaith), 61 => Ok(AtreeItems::HealthierOphanimIi), 62 => Ok(AtreeItems::LargerManaBankIii), 63 => Ok(AtreeItems::Devitalize), 64 => Ok(AtreeItems::Divination), 65 => Ok(AtreeItems::Sunflare), 66 => Ok(AtreeItems::BetterLightweaver), 67 => Ok(AtreeItems::ArcaneOverflow), 68 => Ok(AtreeItems::Timelock), 69 => Ok(AtreeItems::CheaperHealIi), 70 => Ok(AtreeItems::Manastorm), 71 => Ok(AtreeItems::MemoryRecollection), _ => Err(TryIntoWynnEnumError{from: n, to: Self::default()})}}}
impl fmt::Display for AtreeItems{fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {write!(f,"{}",match self{AtreeItems::Meteor => "Meteor", AtreeItems::WandProficiencyI => "WandProficiencyI", AtreeItems::CheaperMeteor => "CheaperMeteor", AtreeItems::ShootingStar => "ShootingStar", AtreeItems::WandProficiencyIi => "WandProficiencyIi", AtreeItems::Teleport => "Teleport", AtreeItems::Wisdom => "Wisdom", AtreeItems::Heal => "Heal", AtreeItems::CheaperTeleport => "CheaperTeleport", AtreeItems::IceSnake => "IceSnake", AtreeItems::AirMastery => "AirMastery", AtreeItems::ThunderMastery => "ThunderMastery", AtreeItems::WaterMastery => "WaterMastery", AtreeItems::FireMastery => "FireMastery", AtreeItems::EarthMastery => "EarthMastery", AtreeItems::WindSlash => "WindSlash", AtreeItems::Thunderstorm => "Thunderstorm", AtreeItems::BurningSigil => "BurningSigil", AtreeItems::Sunshower => "Sunshower", AtreeItems::StrongerMeteor => "StrongerMeteor", AtreeItems::Windsweeper => "Windsweeper", AtreeItems::Ophanim => "Ophanim", AtreeItems::ArcaneTransfer => "ArcaneTransfer", AtreeItems::CheaperHeal => "CheaperHeal", AtreeItems::Purification => "Purification", AtreeItems::SentientSnake => "SentientSnake", AtreeItems::EyePiercer => "EyePiercer", AtreeItems::Breathless => "Breathless", AtreeItems::LargerHeal => "LargerHeal", AtreeItems::LargerManaBank => "LargerManaBank", AtreeItems::CheaperIceSnake => "CheaperIceSnake", AtreeItems::Fortitude => "Fortitude", AtreeItems::CheaperTeleportIi => "CheaperTeleportIi", AtreeItems::Pyrokinesis => "Pyrokinesis", AtreeItems::Blink => "Blink", AtreeItems::HealthierOphanimI => "HealthierOphanimI", AtreeItems::SnakeNest => "SnakeNest", AtreeItems::Seance => "Seance", AtreeItems::TransonicWarp => "TransonicWarp", AtreeItems::FluidHealing => "FluidHealing", AtreeItems::OrphionsPulse => "OrphionsPulse", AtreeItems::ArcaneRestoration => "ArcaneRestoration", AtreeItems::StrongerOphanim => "StrongerOphanim", AtreeItems::Diffusion => "Diffusion", AtreeItems::Lightweaver => "Lightweaver", AtreeItems::ArcaneSpeed => "ArcaneSpeed", AtreeItems::LargerManaBankIi => "LargerManaBankIi", AtreeItems::MoreWinded => "MoreWinded", AtreeItems::Psychokinesis => "Psychokinesis", AtreeItems::CheaperIceSnakeIi => "CheaperIceSnakeIi", AtreeItems::ExplosiveEntrance => "ExplosiveEntrance", AtreeItems::CheaperMeteorIi => "CheaperMeteorIi", AtreeItems::ChaosExplosion => "ChaosExplosion", AtreeItems::TimeDilation => "TimeDilation", AtreeItems::Gust => "Gust", AtreeItems::BetterOphanim => "BetterOphanim", AtreeItems::ArcticSnake => "ArcticSnake", AtreeItems::StrongerSunshower => "StrongerSunshower", AtreeItems::ArcanePower => "ArcanePower", AtreeItems::MoreWindedIi => "MoreWindedIi", AtreeItems::DynamicFaith => "DynamicFaith", AtreeItems::HealthierOphanimIi => "HealthierOphanimIi", AtreeItems::LargerManaBankIii => "LargerManaBankIii", AtreeItems::Devitalize => "Devitalize", AtreeItems::Divination => "Divination", AtreeItems::Sunflare => "Sunflare", AtreeItems::BetterLightweaver => "BetterLightweaver", AtreeItems::ArcaneOverflow => "ArcaneOverflow", AtreeItems::Timelock => "Timelock", AtreeItems::CheaperHealIi => "CheaperHealIi", AtreeItems::Manastorm => "Manastorm", AtreeItems::MemoryRecollection => "MemoryRecollection", })}}enum_from_into!(AtreeItems, u8,u32,u64,i32,i64,usize);
#[derive(Clone,Default,PartialEq,PartialOrd,Eq,Ord,Copy,Debug)]
pub enum Prop{#[default]Aoe,Range,Effects,Duration,Max,Health,NumOrbs,Bank}
impl WynnEnum for Prop{const VARIENTS:&'static[Self]=&[Self::Aoe,Self::Range,Self::Effects,Self::Duration,Self::Max,Self::Health,Self::NumOrbs,Self::Bank];}impl std::convert::TryFrom<u8> for Prop{type Error=TryIntoWynnEnumError<u8,Self>;fn try_from(n: u8) -> Result<Self,Self::Error> {match n{0 => Ok(Prop::Aoe), 1 => Ok(Prop::Range), 2 => Ok(Prop::Effects), 3 => Ok(Prop::Duration), 4 => Ok(Prop::Max), 5 => Ok(Prop::Health), 6 => Ok(Prop::NumOrbs), 7 => Ok(Prop::Bank), _ => Err(TryIntoWynnEnumError{from: n, to: Self::default()})}}}
impl fmt::Display for Prop{fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {write!(f,"{}",match self{Prop::Aoe => "Aoe", Prop::Range => "Range", Prop::Effects => "Effects", Prop::Duration => "Duration", Prop::Max => "Max", Prop::Health => "Health", Prop::NumOrbs => "NumOrbs", Prop::Bank => "Bank", })}}enum_from_into!(Prop, u8,u32,u64,i32,i64,usize);
#[derive(Clone,Default,PartialEq,PartialOrd,Eq,Ord,Copy,Debug)]
pub enum Spell{#[default]Melee,Meteor,Teleport,Heal,IceSnake,BurningSigil,Ophanim,ArcaneTransfer,Lightweaver}
impl WynnEnum for Spell{const VARIENTS:&'static[Self]=&[Self::Melee,Self::Meteor,Self::Teleport,Self::Heal,Self::IceSnake,Self::BurningSigil,Self::Ophanim,Self::ArcaneTransfer,Self::Lightweaver];}impl std::convert::TryFrom<u8> for Spell{type Error=TryIntoWynnEnumError<u8,Self>;fn try_from(n: u8) -> Result<Self,Self::Error> {match n{0 => Ok(Spell::Melee), 1 => Ok(Spell::Meteor), 2 => Ok(Spell::Teleport), 3 => Ok(Spell::Heal), 4 => Ok(Spell::IceSnake), 5 => Ok(Spell::BurningSigil), 6 => Ok(Spell::Ophanim), 7 => Ok(Spell::ArcaneTransfer), 8 => Ok(Spell::Lightweaver), _ => Err(TryIntoWynnEnumError{from: n, to: Self::default()})}}}
impl fmt::Display for Spell{fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {write!(f,"{}",match self{Spell::Melee => "Melee", Spell::Meteor => "Meteor", Spell::Teleport => "Teleport", Spell::Heal => "Heal", Spell::IceSnake => "IceSnake", Spell::BurningSigil => "BurningSigil", Spell::Ophanim => "Ophanim", Spell::ArcaneTransfer => "ArcaneTransfer", Spell::Lightweaver => "Lightweaver", })}}enum_from_into!(Spell, u8,u32,u64,i32,i64,usize);
#[derive(Clone,Default,PartialEq,PartialOrd,Eq,Ord,Copy,Debug)]
pub enum SpellPart{#[default]None,Total,Dps,TotalDps,MeteorDamage,Heal,IceSnakeDamage,TickDamage,TotalBurnDamage,PerOrb,PerMeleeMax,SingleOrb,OrbDps,WindSlash,LightningDamage,SunshowerDamage,BreathlessDamage,SecondAndThirdPulses,TotalHeal,ExplosionDamage}
impl WynnEnum for SpellPart{const VARIENTS:&'static[Self]=&[Self::None,Self::Total,Self::Dps,Self::TotalDps,Self::MeteorDamage,Self::Heal,Self::IceSnakeDamage,Self::TickDamage,Self::TotalBurnDamage,Self::PerOrb,Self::PerMeleeMax,Self::SingleOrb,Self::OrbDps,Self::WindSlash,Self::LightningDamage,Self::SunshowerDamage,Self::BreathlessDamage,Self::SecondAndThirdPulses,Self::TotalHeal,Self::ExplosionDamage];}impl std::convert::TryFrom<u8> for SpellPart{type Error=TryIntoWynnEnumError<u8,Self>;fn try_from(n: u8) -> Result<Self,Self::Error> {match n{0 => Ok(SpellPart::None), 1 => Ok(SpellPart::Total), 2 => Ok(SpellPart::Dps), 3 => Ok(SpellPart::TotalDps), 4 => Ok(SpellPart::MeteorDamage), 5 => Ok(SpellPart::Heal), 6 => Ok(SpellPart::IceSnakeDamage), 7 => Ok(SpellPart::TickDamage), 8 => Ok(SpellPart::TotalBurnDamage), 9 => Ok(SpellPart::PerOrb), 10 => Ok(SpellPart::PerMeleeMax), 11 => Ok(SpellPart::SingleOrb), 12 => Ok(SpellPart::OrbDps), 13 => Ok(SpellPart::WindSlash), 14 => Ok(SpellPart::LightningDamage), 15 => Ok(SpellPart::SunshowerDamage), 16 => Ok(SpellPart::BreathlessDamage), 17 => Ok(SpellPart::SecondAndThirdPulses), 18 => Ok(SpellPart::TotalHeal), 19 => Ok(SpellPart::ExplosionDamage), _ => Err(TryIntoWynnEnumError{from: n, to: Self::default()})}}}
impl fmt::Display for SpellPart{fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {write!(f,"{}",match self{SpellPart::None => "None", SpellPart::Total => "Total", SpellPart::Dps => "Dps", SpellPart::TotalDps => "TotalDps", SpellPart::MeteorDamage => "MeteorDamage", SpellPart::Heal => "Heal", SpellPart::IceSnakeDamage => "IceSnakeDamage", SpellPart::TickDamage => "TickDamage", SpellPart::TotalBurnDamage => "TotalBurnDamage", SpellPart::PerOrb => "PerOrb", SpellPart::PerMeleeMax => "PerMeleeMax", SpellPart::SingleOrb => "SingleOrb", SpellPart::OrbDps => "OrbDps", SpellPart::WindSlash => "WindSlash", SpellPart::LightningDamage => "LightningDamage", SpellPart::SunshowerDamage => "SunshowerDamage", SpellPart::BreathlessDamage => "BreathlessDamage", SpellPart::SecondAndThirdPulses => "SecondAndThirdPulses", SpellPart::TotalHeal => "TotalHeal", SpellPart::ExplosionDamage => "ExplosionDamage", })}}enum_from_into!(SpellPart, u8,u32,u64,i32,i64,usize);