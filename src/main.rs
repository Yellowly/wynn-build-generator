use atree::AtreeBuild;
// mod items;
// mod wynn_calcs;
use gloo::timers::callback::Timeout;
// use items::{Atrs, Category, Sets, Skill, Tier, Type, WynnItem, NUM_ITEM_IDS, NUM_NON_IDS};
use std::{future::Future, mem::size_of, ops::{Add, AddAssign, Neg, Sub, SubAssign}, rc::Rc, task::Poll};
use web_sys::{console, Event, HtmlInputElement};
use yew::prelude::*;
// use gloo::timers::callback::{Interval, Timeout};

// use crate::WynnBuild;

mod wynn_data;
use wynn_data::{*, builder::WynnBuild, items::*, I12x5, atree::assassin::AtreeItems};

mod best_build_search;
mod website;


// notes
// new URLSearchParams(window.location.search);
// ehp is broken for some reason (8/31/2024)


fn main() {
    // Set this to false to run locally
    let run_website = true;

    if run_website {
        yew::start_app::<website::RootComponent>();
    } else {
        // Uncomment and modify local_run to run locally

        // Local_tests are for my personal testing
        // local_run();
        local_tests();
    }
}

/// Example function for running a build search locally
fn local_run() {
    // Set your weapon here
    let weapon = WynnItem::OAK_WOOD_DAGGER;

    // Initalize items to use in the search
    // [helmet, chestplate, leggings, boots, ring1, ring2, bracelet, necklace]
    let mut items_with_type: [Vec<WynnItem>; 8] = [Vec::new(),Vec::new(),Vec::new(),Vec::new(),Vec::new(),Vec::new(),Vec::new(),Vec::new()];

    for i in 0..8_usize {
        // Initializes vec with every item of a certain type
        items_with_type[i] = items::with_prop_value(Atrs::Type, if i>4{i - 1}else{i} as u32);

        // Filter for certain attributes. Example for filtering level > 90:
        items_with_type[i].retain(|itm| itm.get_ident(Atrs::Lvl).unwrap() > 90);

        // Everything breaks if more than 255 items are used in one search
        // Thus, cap the size
        items_with_type[i].resize(255, WynnItem::NULL);
    }

    // If you want to override with a specific list of items, do something like this here
    // (Comment out if you don't want)
    items_with_type[0] = vec![WynnItem::CUMULONIMBUS];
    items_with_type[1] = vec![WynnItem::SCHADENFREUDE, WynnItem::PETRICHOR];
    items_with_type[7] = vec![WynnItem::DIAMOND_HYDRO_NECKLACE, WynnItem::DIAMOND_FUSION_NECKLACE];

    // Make atree here.
    // Using anything more than super basic spell atree items will probably break things
    // Change the "use atree::assassin::AtreeItems" at the top of this file to archer or whatever for other classes
    let test_atree_items: &[AtreeItems] = &[AtreeItems::SpinAttack,AtreeItems::Dash,AtreeItems::Multihit,AtreeItems::SmokeBomb];
    let test_atree = Rc::new(AtreeBuild::from(test_atree_items));

    // Run the search for highest spell1 damage
    let mut calc_future = best_build_search::BestBuildSearch::make(weapon, items_with_type, test_atree, |bld| bld.calc_spell_dam(0) as i32);

    while !calc_future.calc_best_build(1000000){
        println!("best so far: {}, hashval: {} \nnames {}",calc_future.peek_curr_best().0,calc_future.peek_curr_best().1.generate_hash(),calc_future.peek_curr_best().1.item_names());
    }
}


fn local_tests(){
    let id = 250_u32; // first 8 bits represent the ID (unsigned)
    let value = -7999999_i32 as u32 & 0xFFFFFF; // last 24 bits is the value (signed)
    let idval = (id << 24) | value;
    let idval2 = idval as i32;
    println!(
        "id: {}, value: {}",
        idval >> 24,
        ((idval & 0x800000) * 0x1FF | idval & 0xFFFFFF) as i32
    );
    println!("2: id: {}, value: {}",idval >> 24,(idval as i32)<<8>>8);

    let sps = [16,28,39,21,32];
    let sps2 = [-23,-2,-40,-1,-2];
    let sps3 = [23,2,40,1,2];

    let mut compressed_sps: i64 = (sps[0] as i64 & 0x7FF)<<48 | (sps[1] as i64 & 0x7FF)<<36 | (sps[2] as i64 & 0x7FF)<<24 | (sps[3] as i64 & 0x7FF)<<12 | (sps[4] as i64 & 0x7FF);
    let mut compressed_sps2: i64 = (sps2[0] as i64 & 0x7FF)<<48 | (sps2[1] as i64 & 0x7FF)<<36 | (sps2[2] as i64 & 0x7FF)<<24 | (sps2[3] as i64 & 0x7FF)<<12 | (sps2[4] as i64 & 0x7FF);

    let mut compressed_sps3: i64 = (sps[0] as i64 & 0xFFF)<<52 | (sps[1] as i64 & 0xFFF)<<39 | (sps[2] as i64 & 0xFFF)<<26 | (sps[3] as i64 & 0xFFF)<<13 | (sps[4] as i64 & 0xFFF);
    let mut compressed_sps4: i64 = (sps2[0] as i64 & 0xFFF)<<52 | (sps2[1] as i64 & 0xFFF)<<39 | (sps2[2] as i64 & 0xFFF)<<26 | (sps2[3] as i64 & 0xFFF)<<13 | (sps2[4] as i64 & 0xFFF);

    // compressed_sps=(compressed_sps+compressed_sps2); // & 0x7FF7FF7FF7FF7FF
    compressed_sps3=(compressed_sps3+compressed_sps4) & -2252074725150721; //0xFFF7FFBFFDFFEFFF;
// ((compressed_sps>>12)&0x800*-1)&0xFFF
    // println!("{} {} {} {} {}",(compressed, ((compressed_sps>>36)&0xFFF) - 2048, ((compressed_sps>>24)&0xFFF) - 2048, ((compressed_sps>>12)&0xFFF) - 2048, (compressed_sps&0xFFF) - 2048);

    // println!("{} {} {} {} {}",((compressed_sps2>>48) & 0xFFF) - 2048, ((compressed_sps2>>36)&0xFFF) - 2048, ((compressed_sps2>>24)&0xFFF) - 2048, ((compressed_sps2>>12)&0xFFF) - 2048, (compressed_sps2&0xFFF) - 2048);

    let temp = [compressed_sps3>>52,(compressed_sps3>>39)&0xFFF,(compressed_sps3>>26)&0xFFF,(compressed_sps3>>39)&0xFFF,(compressed_sps3>>39)&0xFFF];
    println!("{} {} {} {} {}",((compressed_sps3>>52) & 0x800) * 0x1FFFFFFFFFFFF | (compressed_sps3>>52) & 0xFFF, ((compressed_sps3>>39)&0xFFF), ((compressed_sps3>>26)&0xFFF), ((compressed_sps3>>13)&0xFFF), (compressed_sps3&0xFFF));
    println!("{} {} {} {} {} {}",(compressed_sps3<<52>>52)+(compressed_sps3<<39>>52)+(compressed_sps3<<26>>52)+(compressed_sps3<<13>>52)+(compressed_sps3>>52),compressed_sps3>>52, compressed_sps3<<13>>52, compressed_sps3<<26>>52, compressed_sps3<<39>>52, compressed_sps3<<52>>52);

    // let test = SkillPts::from(sps);
    // let test2 = SkillPts::from(sps2);
    // let test3 = SkillPts::from(sps3);

    // let test4 = -test;
    println!("sp4 cost red {}",make_build!((KERATOCONUS, AZURITE, RINGLETS, CAPRICORN, MOON_POOL_CIRCLET, OLD_KEEPERS_RING, PROWESS, RENDA_LANGIT, HERO),105).unwrap().get_stat(Atrs::SpRaw4));

    // println!("{}",test+test2);
    // println!("{}, {}",test-test3,(test-test3).get_skill(Skill::Str));
    // println!("{}",test4);
    // println!("{:?}",sps);
    println!("{}",-wynn_data::I12x5::from([2047,-23,-43,-12,98]).get_pos());
    println!("{}",wynn_data::I12x5::from([5,-23,43,-12,98]).get_negs());
    println!("{}",wynn_data::I12x5::from([0,0,0,0,0]).is_neg());

    println!("{}",wynn_data::I12x5::from([148,151,-15,151,151]).with_max(150));
    println!("{}",wynn_data::I12x5::from([-1,151,-15,151,-25]).with_min(0));
    println!("ahhh {}",(wynn_data::I12x5::from([-1024, 30, 30, -1024, 30])-wynn_data::I12x5::from([33, 33, 33, 33, 33])).get_negs());



    let mut tesbld = WynnBuild::make_from_names(&"Dune Storm,Dondasch,Horizon,Revenant,Dispersion,Dispersion,Knucklebones,Achromatic Gloom,Oak Wood Spear".split(",").collect::<Vec<&str>>(),106).unwrap();
    println!("made test bld {}",tesbld.item_names());
    println!("test? dam {} {:?}",tesbld.calc_melee_dam(true),tesbld.skills);
    println!("test2w? {}",items::with_name("Spring").unwrap().get_skill_reqs());
    println!("{}",wynn_data::I12x5::from([148,-151,50,151,28]).max(wynn_data::I12x5::from([120,30,-15,24,11])));


    tesbld = WynnBuild::from_test(&[WynnItem::from_idx(2988),WynnItem::from_idx(3632),WynnItem::from_idx(2793),WynnItem::from_idx(983),WynnItem::from_idx(1678),WynnItem::from_idx(2864),WynnItem::from_idx(2641),WynnItem::from_idx(3110),WynnItem::from_idx(1302)],201,I12x5::ZERO).unwrap();
    println!("tesbld1 skills: {:?} {}",tesbld.skills,tesbld.generate_hash());
    // !n + 1 = !(n-1)
    tesbld = WynnBuild::make_with_free_spts(&[WynnItem::from_idx(2988),WynnItem::from_idx(3632),WynnItem::from_idx(2793),WynnItem::from_idx(983),WynnItem::from_idx(1678),WynnItem::from_idx(2864),WynnItem::from_idx(2641),WynnItem::from_idx(3110),WynnItem::from_idx(1302)],201).unwrap();
    println!("tesbld1 skills 2: {:?} {} {}",tesbld.skills, tesbld.skills.iter::<i32>().sum::<i32>(),tesbld.generate_hash());

    // tesbld = WynnBuild::from_test(&[items::with_name("Aphotic").unwrap(),items::with_name("Starglass").unwrap(),items::with_name("Vaward").unwrap(),items::with_name("Memento").unwrap(),items::with_name("Draoi Fair").unwrap(),items::with_name("Yang").unwrap(),items::with_name("Diamond Hydro Bracelet").unwrap(),items::with_name("Contrast").unwrap(),items::with_name("Spring").unwrap()],106,I12x5::ZERO).unwrap();
    // println!("tesbld aphotic 1 (negatives): {:?} {}",tesbld.skills,tesbld.generate_hash());
    // tesbld = WynnBuild::make_with_free_spts(&[items::with_name("Aphotic").unwrap(),items::with_name("Starglass").unwrap(),items::with_name("Vaward").unwrap(),items::with_name("Memento").unwrap(),items::with_name("Draoi Fair").unwrap(),items::with_name("Yang").unwrap(),items::with_name("Diamond Hydro Bracelet").unwrap(),items::with_name("Contrast").unwrap(),items::with_name("Spring").unwrap()],106).unwrap();
    // println!("tesbld aphotic 2 (negatives): {:?} {}",tesbld.skills,tesbld.generate_hash());

    tesbld = make_build!((CONFLAGRATE, KEEPER_OF_SOULS, CINDERCHAIN, ACHILLES, DIAMOND_FIBER_RING, YIN, KNUCKLEBONES, RECALCITRANCE, GUARDIAN),105).unwrap();
    println!("tesbld guardian 3 (negatives): {:?} {} {}",tesbld.skills,tesbld.calc_melee_dam(true),tesbld.generate_hash());

    tesbld = make_build!({Aphotic, Starglass, Vaward, Memento, Draoi Fair, Yang, Diamond Hydro Bracelet, Contrast, Spring},106).unwrap();
    println!("tesbld aphotic 3 (negatives): {:?} {} {}",tesbld.skills,tesbld.calc_melee_dam(true),tesbld.generate_hash());
    tesbld = make_build!((APHOTIC, STARGLASS, VAWARD, MEMENTO, DRAOI_FAIR, YANG, DIAMOND_HYDRO_BRACELET, CONTRAST, SPRING),105).unwrap();
    println!("tesbld aphotic 4 (negatives): {:?} {} {}\n{} {} {} {}",tesbld.skills,tesbld.calc_spell_dam(0),tesbld.generate_hash(),tesbld.get_stat(Atrs::SdRaw),tesbld.get_stat(Atrs::SdPct),tesbld.get_stat(Atrs::WDamPct),tesbld.get_stat(Atrs::FDamPct));

    // tesbld = WynnBuild::from_test(&[items::with_name("Dune Storm").unwrap(),items::with_name("Elysium-Engraved Aegis").unwrap(),items::with_name("Barbarian").unwrap(),items::with_name("Revenant").unwrap(),items::with_name("Dispersion").unwrap(),items::with_name("Dispersion").unwrap(),items::with_name("Knucklebones").unwrap(),items::with_name("Incendiary").unwrap(),items::with_name("Oak Wood Spear").unwrap()],106,I12x5::ZERO).unwrap();
    // println!("tesbld dune storm 1 (negatives): {:?} {}",tesbld.skills,tesbld.generate_hash());
    // tesbld = WynnBuild::make_with_free_spts(&[items::with_name("Dune Storm").unwrap(),items::with_name("Elysium-Engraved Aegis").unwrap(),items::with_name("Barbarian").unwrap(),items::with_name("Revenant").unwrap(),items::with_name("Dispersion").unwrap(),items::with_name("Dispersion").unwrap(),items::with_name("Knucklebones").unwrap(),items::with_name("Incendiary").unwrap(),items::with_name("Oak Wood Spear").unwrap()],106).unwrap();
    // println!("tesbld dune storm 2 (negatives): {:?} {}",tesbld.skills,tesbld.generate_hash());

    // tesbld = WynnBuild::from_names_test(&"Morph-Stardust,Morph-Steel,Morph-Iron,Morph-Gold,Morph-Emerald,Morph-Topaz,Morph-Amethyst,Morph-Ruby,Oak Wood Spear".split(",").collect::<Vec<&str>>(),106).unwrap();
    // println!("tesbld morph 1 (negatives): {:?} {} {}",tesbld.skills,tesbld.calc_melee_dam(true),tesbld.generate_hash());
    // tesbld = WynnBuild::make_from_names(&"Morph-Stardust,Morph-Steel,Morph-Iron,Morph-Gold,Morph-Emerald,Morph-Topaz,Morph-Amethyst,Morph-Ruby,Oak Wood Spear".split(",").collect::<Vec<&str>>(),106).unwrap();
    // println!("tesbld morph 2 (negatives): {:?} {} {}",tesbld.skills,tesbld.calc_melee_dam(true),tesbld.generate_hash());

    // tesbld = WynnBuild::from_names_test(&"Elf Cap,Elf Robe,Elf Pants,Elf Shoes,Oak Wood Spear".split(",").collect::<Vec<&str>>(),106).unwrap();
    // println!("tesbld elph 1: {:?} {} {}",tesbld.skills,tesbld.calc_melee_dam(true),tesbld.generate_hash());
    // tesbld = WynnBuild::make_from_names(&"Elf Cap,Elf Robe,Elf Pants,Elf Shoes,Oak Wood Spear".split(",").collect::<Vec<&str>>(),106).unwrap();
    // println!("tesbld elph 2: {:?} {} {}",tesbld.skills,tesbld.calc_melee_dam(true),tesbld.generate_hash());

    tesbld = make_build!(&"Elf Cap,Elf Robe,Elf Pants,Elf Shoes,Oak Wood Spear".split(",").collect::<Vec<&str>>(),106,I12x5::ZERO).unwrap();
    tesbld = make_build!((ELF_CAP,ELF_ROBE,ELF_PANTS,ELF_SHOES,OAK_WOOD_SPEAR),106).unwrap();
    tesbld = make_build!((ELF_CAP,ELF_ROBE,ELF_PANTS,ELF_SHOES,OAK_WOOD_SPEAR),106).unwrap();

    println!("tesbuild elf {} {} {}",tesbld.skills,tesbld.calc_ehp(),tesbld.calc_melee_dam(false));

    let atreetemp: std::rc::Rc<atree::AtreeBuild> = atree::AtreeBuild::default().into();
    
    let tesitm = WynnItem::OAK_WOOD_SPEAR;
    println!("tesitm hashing {}", WynnItem::from_hash(&tesitm.get_hash()).unwrap().name());

    let temp: &[AtreeItems] = &[AtreeItems::SpinAttack,AtreeItems::Dash,AtreeItems::Multihit,AtreeItems::SmokeBomb];
    let tesatree = AtreeBuild::from(temp);
    println!("tesatree {} {} {} {}, spells: {:?} {:?} {:?} {:?}",tesatree.get_cost(0),tesatree.get_cost(1),tesatree.get_cost(2),tesatree.get_cost(3),tesatree.get_spell_mults(0),tesatree.get_spell_mults(1),tesatree.get_spell_mults(2),tesatree.get_spell_mults(3));
    tesbld = make_build!((CUMULONIMBUS,LIBRA,ALEPH_NULL,STARDEW,PHOTON,SUMMA,SUCCESSION,DIAMOND_FUSION_NECKLACE,NIRVANA),106,I12x5::ZERO,tesatree.into()).unwrap();

    println!("tesbld with atree {} {}",tesbld.get_spell_cost(0),tesbld.spell_per_second(0, false));

    // tesbld = WynnBuild::from_names_test(&"Morph-Stardust,Libra,Aleph Null,Stardew,Prism,Summa,Succession,Diamond Fusion Necklace,Nirvana".split(",").collect::<Vec<&str>>(),106).unwrap();
    // println!("tesbld nirvana 1: {:?} {} {}",tesbld.skills,tesbld.calc_melee_dam(true),tesbld.generate_hash());
    // tesbld = WynnBuild::make_from_names(&"Morph-Stardust,Libra,Aleph Null,Stardew,Prism,Summa,Succession,Diamond Fusion Necklace,Nirvana".split(",").collect::<Vec<&str>>(),106).unwrap();
    // println!("tesbld nirvana 2: {:?} {} {}",tesbld.skills,tesbld.calc_spell_dam(),tesbld.generate_hash());
}