//! This file is autogenerated by rs_generator.py.<br>Code in this file uses wynncraft item data from [wynnbuilder's github repo](https://raw.githubusercontent.com/hppeng-wynn/hppeng-wynn.github.io/dev/data/2.0.4.3/items.json)
use super::{AtreeItems,super::{AtreeItemData,ArcherAtreeEnums}};
pub const ATREE_DATA: &'static[&'static AtreeItemData<ArcherAtreeEnums>] = &[&AtreeItemData{name:"Arrow Bomb",parents:&[],deps:&[],blockers:&[],props:&[450,16777242],effects:&[&[0,16777217,33554477,50331651,251658253,0,16777216,268435462,160,0,0,0,20,0,1,16777217,352321537,1]],data:&[0,67108864],enum_id:AtreeItems::ArrowBomb},
&AtreeItemData{name:"Bow Proficiency",parents:&[0],deps:&[],blockers:&[],props:&[],effects:&[&[1,50331648,67108872,268435462,5,0,0,0,0,0]],data:&[1,67108865,134218727],enum_id:AtreeItems::BowProficiency},
&AtreeItemData{name:"Cheaper Arrow Bomb",parents:&[1],deps:&[],blockers:&[],props:&[],effects:&[&[1,50331638,50331651]],data:&[2,67108866,134217728],enum_id:AtreeItems::CheaperArrowBomb},
&AtreeItemData{name:"Heart Shatter",parents:&[1],deps:&[],blockers:&[],props:&[],effects:&[&[1,50331651,67108880,268435462,100,0,0,0,0,0],&[1,50331651,67108865,352321537,268435457]],data:&[3,67108867,134217728],enum_id:AtreeItems::HeartShatter},
&AtreeItemData{name:"Double Shots",parents:&[5],deps:&[],blockers:&[6],props:&[],effects:&[&[1,50331648,67108872,268435462,4294967266,0,0,0,0,0],&[1,50331648,67108865,352321537,134217730]],data:&[4,67108868,134218727,150994945,167772160],enum_id:AtreeItems::DoubleShots},
&AtreeItemData{name:"Escape",parents:&[3],deps:&[],blockers:&[],props:&[0,16777216],effects:&[&[0,16777218,33554452,50331650,251658240]],data:&[5,67108869],enum_id:AtreeItems::Escape},
&AtreeItemData{name:"Power Shots",parents:&[5],deps:&[],blockers:&[4],props:&[],effects:&[],data:&[6,67108870,134218727,150994946,167772160],enum_id:AtreeItems::PowerShots},
&AtreeItemData{name:"Arrow Storm",parents:&[4,8],deps:&[],blockers:&[],props:&[16777232],effects:&[&[0,16777219,33554467,50331649,251658254,16777221,268435462,30,0,10,0,0,0,16777222,352321537,83886088,16777217,352321537,100663297]],data:&[7,67108871],enum_id:AtreeItems::ArrowStorm},
&AtreeItemData{name:"Cheaper Escape I",parents:&[7,9],deps:&[],blockers:&[],props:&[],effects:&[&[1,50331643,50331650]],data:&[8,67108872,134217733],enum_id:AtreeItems::CheaperEscapeI},
&AtreeItemData{name:"Arrow Shield",parents:&[6,8],deps:&[],blockers:&[],props:&[33554434,50331708,67109014],effects:&[&[0,16777220,33554462,50331652,251658253,0,16777223,268435462,90,0,0,0,0,10,1,16777217,352321537,117440512]],data:&[9,67108873],enum_id:AtreeItems::ArrowShield},
&AtreeItemData{name:"Windy Feet",parents:&[7],deps:&[],blockers:&[],props:&[8,50331768],effects:&[],data:&[10,67108874,134217733],enum_id:AtreeItems::WindyFeet},
&AtreeItemData{name:"Air Mastery",parents:&[7],deps:&[],blockers:&[],props:&[],effects:&[&[2,285212676,0,1308622863,0,2113941508]],data:&[11,67108875,134218726,150994945,167772160],enum_id:AtreeItems::AirMastery},
&AtreeItemData{name:"Thunder Mastery",parents:&[7,13,8],deps:&[],blockers:&[],props:&[],effects:&[&[2,285212676,0,1258291210,0,2063601672]],data:&[12,67108876,134218726,150994945,167772160],enum_id:AtreeItems::ThunderMastery},
&AtreeItemData{name:"Fire Mastery",parents:&[12,9,8],deps:&[],blockers:&[],props:&[],effects:&[&[2,285212676,0,1291845647,0,2097164293]],data:&[13,67108877,134218726,150994946,167772160],enum_id:AtreeItems::FireMastery},
&AtreeItemData{name:"Water Mastery",parents:&[9],deps:&[],blockers:&[],props:&[],effects:&[&[2,285212676,0,1275068431,0,2080382980]],data:&[14,67108878,134218726,150994946,167772160],enum_id:AtreeItems::WaterMastery},
&AtreeItemData{name:"Earth Mastery",parents:&[12,8,13],deps:&[],blockers:&[],props:&[],effects:&[&[2,285212676,0,1241514004,0,2046828548]],data:&[15,67108879,134218726,150994947,167772160],enum_id:AtreeItems::EarthMastery},
&AtreeItemData{name:"Nimble String",parents:&[11,17],deps:&[7],blockers:&[20],props:&[],effects:&[&[1,50331649,67108869,268435462,4294967291,0,4294967291,0,0,0],&[1,50331649,67108870,352321537,83886086]],data:&[16,67108880,134217735],enum_id:AtreeItems::NimbleString},
&AtreeItemData{name:"Arrow Rain",parents:&[16,12],deps:&[9],blockers:&[],props:&[],effects:&[&[1,50331652,67108881,268435462,150,0,0,0,0,100]],data:&[17,67108881,134217737],enum_id:AtreeItems::ArrowRain},
&AtreeItemData{name:"Bryophyte Roots",parents:&[19,15],deps:&[7],blockers:&[],props:&[2,50331653],effects:&[&[1,33554432,50331649,67108882,268435462,30,10,0,0,0,0],&[1,50331649,67108883,352321537,301989900]],data:&[18,67108882,134217735,150994947,167772161],enum_id:AtreeItems::BryophyteRoots},
&AtreeItemData{name:"Fire Creep",parents:&[20,13,18],deps:&[],blockers:&[],props:&[80,50331654],effects:&[&[1,50331651,67108884,268435462,30,0,0,0,20,0],&[1,50331651,67108885,352321537,335544335]],data:&[19,67108883,134217728],enum_id:AtreeItems::FireCreep},
&AtreeItemData{name:"Phantom Ray",parents:&[14,19],deps:&[7],blockers:&[26,16,64],props:&[16777232],effects:&[&[0,16777221,50331649,251658253,0,16777221,268435462,25,0,0,5,0,0,1,16777217,352321537,83886092],&[1,50331643,50331649]],data:&[20,67108884,134217735],enum_id:AtreeItems::PhantomRay},
&AtreeItemData{name:"Triple Shots",parents:&[16,22],deps:&[4],blockers:&[],props:&[],effects:&[&[1,50331648,67108872,268435462,4294967276,0,0,0,0,0],&[1,50331648,67108865,352321537,134217729]],data:&[21,67108885,134218727,150994945,167772160],enum_id:AtreeItems::TripleShots},
&AtreeItemData{name:"Frenzy",parents:&[21,17],deps:&[],blockers:&[],props:&[],effects:&[&[3,83886081,100663296,117440536,134217798,301989889,67,318767105,3]],data:&[22,67108886,150994945,167772160],enum_id:AtreeItems::Frenzy},
&AtreeItemData{name:"Guardian Angels",parents:&[21,22],deps:&[9],blockers:&[],props:&[16777222,50331708,83886088],effects:&[&[0,16777222,50331652,251658265,0,16777224,268435462,30,0,0,0,0,10,1,16777225,352321537,134217728,1,16777226,352321537,134217728,1,16777218,352321537,167772162,1,16777217,352321537,150994944]],data:&[23,67108887,134217737,150994945,167772163],enum_id:AtreeItems::GuardianAngels},
&AtreeItemData{name:"Basaltic Trap",parents:&[18],deps:&[],blockers:&[],props:&[7,100663298],effects:&[&[0,16777223,50331655,251658249,0,16777227,268435462,210,20,0,0,20,0]],data:&[24,67108888,150994947,167772162],enum_id:AtreeItems::BasalticTrap},
&AtreeItemData{name:"Focus",parents:&[20],deps:&[],blockers:&[],props:&[],effects:&[&[3,83886081,100663296,117440515,301989889,0,318767105,12]],data:&[25,67108889,150994946,167772162],enum_id:AtreeItems::Focus},
&AtreeItemData{name:"Windstorm",parents:&[23,27],deps:&[],blockers:&[20],props:&[],effects:&[&[1,50331649,67108869,268435462,4294967287,0,4294967293,0,0,2],&[1,50331649,67108865,352321537,100663297],&[1,33554432,50331649,67108870,352321537,83886082]],data:&[26,67108890,134217735],enum_id:AtreeItems::Windstorm},
&AtreeItemData{name:"Cheaper Arrow Storm I",parents:&[28,26,24],deps:&[],blockers:&[],props:&[],effects:&[&[1,50331643,50331649]],data:&[27,67108891,134217735],enum_id:AtreeItems::CheaperArrowStormI},
&AtreeItemData{name:"Implosion",parents:&[27,24,29],deps:&[],blockers:&[],props:&[],effects:&[&[1,50331651,67108880,268435462,40,0,0,0,0,0]],data:&[28,67108892,134217728,150994947,167772160],enum_id:AtreeItems::Implosion},
&AtreeItemData{name:"More Shields",parents:&[25,28],deps:&[9],blockers:&[],props:&[33554434],effects:&[&[1,50331652,67108872,268435462,4294967283,0,0,0,0,4294967291]],data:&[29,67108893,134217737],enum_id:AtreeItems::MoreShields},
&AtreeItemData{name:"Patient Hunter",parents:&[27,28,31],deps:&[24],blockers:&[],props:&[134217828],effects:&[&[3,83886081,100663296,117440612,150994945,301989889,0,318767105,1]],data:&[30,67108894,134217752,150994947,167772160],enum_id:AtreeItems::PatientHunter},
&AtreeItemData{name:"Grappling Hook",parents:&[28,29,30],deps:&[],blockers:&[54],props:&[16777246],effects:&[],data:&[31,67108895,134217733,150994947,167772160],enum_id:AtreeItems::GrapplingHook},
&AtreeItemData{name:"More Focus I",parents:&[29],deps:&[25],blockers:&[],props:&[],effects:&[&[3,83886081,100663296,117440514,301989889,0,318767105,0]],data:&[32,67108896,134217753,150994946,167772160],enum_id:AtreeItems::MoreFocusI},
&AtreeItemData{name:"Stormy Feet",parents:&[26],deps:&[10],blockers:&[],props:&[50331708],effects:&[],data:&[33,67108897,134217733,150994945],enum_id:AtreeItems::StormyFeet},
&AtreeItemData{name:"Call of the Hound",parents:&[30],deps:&[9],blockers:&[],props:&[],effects:&[&[0,16777224,50331656,251658251,16777228,268435462,40,0,0,0,0,0,16777218,352321537,201326925]],data:&[34,67108898,134217737,150994947,167772160],enum_id:AtreeItems::CallOfTheHound},
&AtreeItemData{name:"Leap",parents:&[36,26],deps:&[],blockers:&[],props:&[134217730],effects:&[],data:&[35,67108899,150994945,167772164],enum_id:AtreeItems::Leap},
&AtreeItemData{name:"Traveler",parents:&[35,37],deps:&[],blockers:&[],props:&[],effects:&[&[3,83886080,134217828,301989889,56,318767105,1,335544322,0,16777283]],data:&[36,67108900],enum_id:AtreeItems::Traveler},
&AtreeItemData{name:"Bouncing Bomb",parents:&[30,36,38],deps:&[],blockers:&[],props:&[],effects:&[],data:&[37,67108901,134217728,167772160],enum_id:AtreeItems::BouncingBomb},
&AtreeItemData{name:"Ivyroot Mamba",parents:&[37,39],deps:&[18],blockers:&[],props:&[],effects:&[&[0,16777225,50331657,251658254,16777228,268435462,80,20,0,0,0,0,16777229,352321537,201326658,16777219,352321537,218103809]],data:&[38,67108902,134217735,150994947,167772163],enum_id:AtreeItems::IvyrootMamba},
&AtreeItemData{name:"Twain's Arc",parents:&[32,38],deps:&[25],blockers:&[],props:&[16777280,150994946],effects:&[&[0,16777226,50331653,167772160,251658249,0,16777224,268435462,220,0,0,0,0,0,318767104]],data:&[39,67108903,150994946,167772164],enum_id:AtreeItems::TwainsArc},
&AtreeItemData{name:"Rocket Jump",parents:&[38,37],deps:&[0],blockers:&[],props:&[],effects:&[],data:&[40,67108904,134217728],enum_id:AtreeItems::RocketJump},
&AtreeItemData{name:"Scorched Earth",parents:&[39,38],deps:&[19],blockers:&[],props:&[40,50331650],effects:&[&[1,50331651,67108884,268435462,10,0,0,0,5,0]],data:&[41,67108905,150994946,167772160],enum_id:AtreeItems::ScorchedEarth},
&AtreeItemData{name:"More Traps",parents:&[37,36],deps:&[24],blockers:&[],props:&[],effects:&[&[2,285212673,100663298]],data:&[42,67108906,134217752,150994947,167772160],enum_id:AtreeItems::MoreTraps},
&AtreeItemData{name:"Refined Gunpowder",parents:&[35,44],deps:&[],blockers:&[],props:&[],effects:&[&[1,50331651,67108865,268435462,50,0,0,0,0,0]],data:&[43,67108907,134217728],enum_id:AtreeItems::RefinedGunpowder},
&AtreeItemData{name:"Fierce Stomp",parents:&[43,45],deps:&[],blockers:&[],props:&[4],effects:&[&[1,33554432,50331650,67108886,268435462,120,0,0,0,0,0],&[1,33554432,50331650,67108865,352321537,369098753]],data:&[44,67108908,134217733,150994945,167772160],enum_id:AtreeItems::FierceStomp},
&AtreeItemData{name:"Cheaper Arrow Shield I",parents:&[44,46,42],deps:&[],blockers:&[],props:&[],effects:&[&[1,50331643,50331652]],data:&[45,67108909,134217737],enum_id:AtreeItems::CheaperArrowShieldI},
&AtreeItemData{name:"Better Arrow Shield",parents:&[38,45,47],deps:&[9],blockers:&[],props:&[67108914],effects:&[&[1,50331651,67108868,268435462,40,0,0,0,0,0],&[2,285212673,1],&[1,50331652,67108872,268435462,3,0,0,0,0,0]],data:&[46,67108910,134217737],enum_id:AtreeItems::BetterArrowShield},
&AtreeItemData{name:"Shocking Bomb",parents:&[39,46],deps:&[0],blockers:&[],props:&[167772160],effects:&[&[1,50331651,67108865,268435462,20,0,20,0,0,0]],data:&[47,67108911,134217728,150994946,167772164],enum_id:AtreeItems::ShockingBomb},
&AtreeItemData{name:"Better Leap",parents:&[43,44],deps:&[35],blockers:&[],props:&[],effects:&[&[2,285212673,150994943]],data:&[48,67108912,134217763,150994945,167772160],enum_id:AtreeItems::BetterLeap},
&AtreeItemData{name:"Homing Shots",parents:&[45,46],deps:&[],blockers:&[],props:&[],effects:&[],data:&[49,67108913,134218727,150994946,167772162],enum_id:AtreeItems::HomingShots},
&AtreeItemData{name:"Mana Trap",parents:&[44,45,51],deps:&[],blockers:&[],props:&[16777232,184549426,201326602],effects:&[&[1,33554442,50331651]],data:&[50,67108914,134217752,150994947,167772166],enum_id:AtreeItems::ManaTrap},
&AtreeItemData{name:"Cheaper Arrow Storm II",parents:&[52,50],deps:&[],blockers:&[],props:&[],effects:&[&[1,50331643,50331649]],data:&[51,67108915,134217735],enum_id:AtreeItems::CheaperArrowStormIi},
&AtreeItemData{name:"Initiator",parents:&[51,46,47],deps:&[25],blockers:&[],props:&[],effects:&[&[2,201326592,285212674,52,587202620]],data:&[52,67108916,150994946],enum_id:AtreeItems::Initiator},
&AtreeItemData{name:"Better Guardian Angels",parents:&[54,43],deps:&[23],blockers:&[],props:&[16777218],effects:&[&[1,50331652,67108873,352321537,134217732]],data:&[53,67108917,134217737,150994945,167772160],enum_id:AtreeItems::BetterGuardianAngels},
&AtreeItemData{name:"Escape Artist",parents:&[53,44,55],deps:&[],blockers:&[31],props:&[],effects:&[&[1,50331650,67108888,268435462,100,0,50,0,0,0],&[1,50331650,67108865,352321537,402653185]],data:&[54,67108918,134217733],enum_id:AtreeItems::EscapeArtist},
&AtreeItemData{name:"Murder Flock",parents:&[50,51,54],deps:&[24],blockers:&[],props:&[],effects:&[&[0,16777227,50331658,251658254,16777228,268435462,60,0,0,0,0,0,16777230,352321537,201326703,16777219,352321537,234881026]],data:&[55,67108919,134217737,150994947,167772165],enum_id:AtreeItems::MurderFlock},
&AtreeItemData{name:"Decimator",parents:&[52,57,51],deps:&[20],blockers:&[],props:&[],effects:&[&[3,83886081,100663296,117440519,301989889,0,318767105,10]],data:&[56,67108920,134217735,150994946,167772160],enum_id:AtreeItems::Decimator},
&AtreeItemData{name:"Phasing Beam",parents:&[56,52],deps:&[39],blockers:&[],props:&[],effects:&[],data:&[57,67108921,150994946],enum_id:AtreeItems::PhasingBeam},
&AtreeItemData{name:"Recycling",parents:&[53,59],deps:&[],blockers:&[],props:&[],effects:&[],data:&[58,67108922,150994945],enum_id:AtreeItems::Recycling},
&AtreeItemData{name:"Shrapnel Bomb",parents:&[58,54,60],deps:&[],blockers:&[],props:&[],effects:&[&[1,50331651,67108889,268435462,40,0,0,0,20,0]],data:&[59,67108923,134217728,150994945,167772162],enum_id:AtreeItems::ShrapnelBomb},
&AtreeItemData{name:"Cheaper Escape II",parents:&[55,59,61],deps:&[],blockers:&[],props:&[],effects:&[&[1,50331643,50331650]],data:&[60,67108924,134217733],enum_id:AtreeItems::CheaperEscapeIi},
&AtreeItemData{name:"Cheaper Arrow Shield II",parents:&[57,60,56],deps:&[],blockers:&[],props:&[],effects:&[&[1,50331643,50331652]],data:&[61,67108925,134217737],enum_id:AtreeItems::CheaperArrowShieldIi},
&AtreeItemData{name:"Stronger Hook",parents:&[60,61],deps:&[31],blockers:&[],props:&[16777224],effects:&[],data:&[62,67108926,134217733,150994947,167772162],enum_id:AtreeItems::StrongerHook},
&AtreeItemData{name:"Coursing Restraints",parents:&[61],deps:&[47],blockers:&[],props:&[50331654,134217740],effects:&[&[2,201326592,285212674,63,587202575]],data:&[63,67108927,150994946],enum_id:AtreeItems::CoursingRestraints},
&AtreeItemData{name:"Arrow Hurricane",parents:&[58,59],deps:&[],blockers:&[20],props:&[],effects:&[&[1,50331649,67108869,268435462,4294967293,0,4294967295,0,0,4294967295],&[1,50331649,67108865,352321537,100663298]],data:&[64,67108928,134217735,150994945,167772170],enum_id:AtreeItems::ArrowHurricane},
&AtreeItemData{name:"Tangled Traps",parents:&[59,60],deps:&[24],blockers:&[],props:&[218103828],effects:&[&[1,50331655,67108890,268435462,20,0,0,0,0,20],&[1,50331655,67108866,352321537,436207621]],data:&[65,67108929,134217752,150994947,167772160],enum_id:AtreeItems::TangledTraps},
&AtreeItemData{name:"Beast Lore",parents:&[65],deps:&[34],blockers:&[],props:&[],effects:&[&[1,50331656,67108876,268435462,40,0,0,0,0,0],&[1,50331658,67108867,352321537,234881026],&[1,50331657,67108867,352321537,218103809]],data:&[66,67108930,134217762,150994947,167772167],enum_id:AtreeItems::BeastLore},
&AtreeItemData{name:"Crepuscular Ray",parents:&[61],deps:&[7],blockers:&[],props:&[],effects:&[&[0,16777228,50331654,251658254,16777221,268435462,35,0,0,10,0,0,16777218,352321537,83886100,16777231,352321537,33554439]],data:&[67,67108931,150994946,167772170],enum_id:AtreeItems::CrepuscularRay},
&AtreeItemData{name:"Minefield",parents:&[65,66],deps:&[24],blockers:&[],props:&[],effects:&[&[1,33554432,50331655,67108875,268435462,4294967246,0,0,0,0,0],&[2,285212674,16777215,100663302]],data:&[68,67108932,134217752,150994947,167772171],enum_id:AtreeItems::Minefield},
&AtreeItemData{name:"All-Seeing Panoptes",parents:&[64],deps:&[23],blockers:&[],props:&[16777220,83886084],effects:&[&[1,50331652,67108872,268435462,2,0,0,0,5,0]],data:&[69,67108933,134217737,150994945],enum_id:AtreeItems::AllseeingPanoptes},
&AtreeItemData{name:"Stronger Patient Hunter",parents:&[68],deps:&[30],blockers:&[],props:&[134217828],effects:&[&[3,83886081,100663296,117440612,318767105,1]],data:&[70,67108934,134217752,150994947,167772160],enum_id:AtreeItems::StrongerPatientHunter},
&AtreeItemData{name:"Grape Bomb",parents:&[72,68],deps:&[],blockers:&[],props:&[2],effects:&[&[1,50331651,67108891,268435462,60,0,0,0,20,0]],data:&[71,67108935,134217728],enum_id:AtreeItems::GrapeBomb},
&AtreeItemData{name:"More Focus II",parents:&[67,71],deps:&[25],blockers:&[],props:&[],effects:&[&[3,83886081,100663296,117440514,301989889,0,318767105,0]],data:&[72,67108936,134217753,150994946,167772160],enum_id:AtreeItems::MoreFocusIi},
&AtreeItemData{name:"Elusive",parents:&[64],deps:&[],blockers:&[],props:&[],effects:&[],data:&[73,67108937,150994945,167772160],enum_id:AtreeItems::Elusive},
&AtreeItemData{name:"Geyser Stomp",parents:&[64,75],deps:&[44],blockers:&[],props:&[],effects:&[&[1,50331650,67108892,268435462,50,0,0,30,0,0],&[1,50331650,67108865,352321537,469762049],&[2,285212673,1]],data:&[74,67108938,134217733],enum_id:AtreeItems::GeyserStomp},
&AtreeItemData{name:"Snow Storm",parents:&[74,76,70],deps:&[],blockers:&[],props:&[16777466,234881054],effects:&[],data:&[75,67108939],enum_id:AtreeItems::SnowStorm},
&AtreeItemData{name:"Cheaper Arrow Bomb II",parents:&[71,72,75],deps:&[],blockers:&[],props:&[],effects:&[&[1,50331643,50331651]],data:&[76,67108940,134217728],enum_id:AtreeItems::CheaperArrowBombIi}];