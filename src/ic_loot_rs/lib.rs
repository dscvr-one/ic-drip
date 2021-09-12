use ic_cdk::api::call::CallResult;
use ic_cdk::export::{
    candid::{CandidType, Deserialize},
    Principal,
};
use ic_cdk::storage;
use ic_cdk_macros::*;
use serde_bytes::ByteBuf;
use std::collections::BTreeMap;

mod address;
mod loot;
mod loot2;
mod rand;

use crate::address::AddressBook;
use crate::loot::Loot;
use crate::loot2::{Loot2, LootData};

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct TransferNotification {
    pub to: Principal,
    pub token_id: u64,
    pub from: Principal,
    pub amount: u64,
}

#[init]
fn init() {
    ic_cdk::println!("Init {:?}", ic_cdk::api::time());

    let address_book = storage::get_mut::<AddressBook>();
    address_book.total_supply = 8000;

    //needs to be a way to pass this into init
    let owner_id =
        Principal::from_text("2c22g-lboam-nseoa-i5al6-o7k6f-o2fwz-huoua-be63r-oi3k2-wy7uq-zae")
            .unwrap();
    address_book.add_controller(&owner_id);

    init_loot();
}

fn init_loot() -> () {
    let loot = storage::get_mut::<Loot>();

    loot.weapons = vec![
        "Cellphone",
        "Laptop",
        "Purse",
        "Cat",
        "Golf Club",
        "Spoon",
        "Beggar Cup",
        "Wine Glass",
        "Fidget Spinner",
        "Massager",
        "Beer Mug",
        "TV Controller",
        "Pillow",
        "Blanket",
        "Wand",
        "Waifu Pillow",
        "Microphone",
        "Guitar",
        "Trumpet",
        "Banjo",
        "Cane",
        "Yo-yo",
        "Tennis Racket",
        "Record Player",
        "Video Game Console",
        "Umbrella",
        "Surf Board",
        "Whip",
        "Scissors",
        "Key",
        "Pen",
        "Paintbrush",
        "Megaphone",
        "Dog Leash",
        "Book",
        "Pocket Watch",
        "Wrench",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();

    loot.chest = vec![
        "Robe",
        "Shirt",
        "Hoodie",
        "Fishnet Shirt",
        "Plastic Shirt",
        "Polyester Shirt",
        "Hawaiian Shirt",
        "Polo Shirt",
        "Ring Mail",
        "Blazer",
        "Vest",
        "Tank Top",
        "Button Up",
        "Bowling Shirt",
        "Shawl",
        "Cardigan",
        "Wind Breaker",
        "Rain Coat",
        "Sports Bra",
        "Graphic Tee",
        "Body Paint",
        "Tunic",
        "Corset",
        "Sweater",
        "Crop Top",
        "Cardigan",
        "Safari Jacket",
        "Bandeau Top",
        "Bra",
        "Bikini Top",
        "Dress",
        "Puffer Jacket",
        "Dinner Jacket",
        "Smoking Jacket",
        "Waist Coat",
        "Tailcoat",
        "Tuxedo Jacket",
        "Parka",
        "Overcoat",
        "Turtle Neck",
        "Bomber Jacket",
        "Trench Coat",
        "Cape",
        "Kimono",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();

    loot.head = vec![
        "Fedora",
        "Sombrero",
        "Sun Hat",
        "Viking Helm",
        "Silk Hood",
        "Hood",
        "Top Hat",
        "Bucket Hat",
        "Baseball Cap",
        "Beret",
        "Doo Rag",
        "Cowboy",
        "Flat Cap",
        "Porkpie Hat",
        "Panama Hat",
        "Sailors Bonet",
        "Beanie",
        "Headband",
        "Captain Hat",
        "Equestrian Helm",
        "Visor",
        "Turban",
        "Wedding Veil",
        "Feather Hat",
        "Propeller Hat",
        "Cat Ears",
        "Headphones",
        "Helmet",
        "Wizard Hat",
        "Safari Hat",
        "Pith Helmet",
        "Tiara",
        "Flower Crown",
        "Crown",
        "Bandana",
        "Umbrella Hat",
        "Motorcycle Helmet",
        "Peaked Cap",
        "Golf Visor",
        "Bowler Hat",
        "Bow Hat",
        "Dunce Cap",
        "Towel",
        "Jewelry Headpiece",
        "Cheese Head",
        "Yarmulke",
        "Bald Cap",
        "Coonskin Cap",
        "Balaclava",
        "Bonnet",
        "Fez",
        "Space Helmet",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();

    loot.waist = vec![
        "Fanny Pack",
        "Fish Belt",
        "Studded Belt",
        "Belt",
        "Leather Belt",
        "Chrome Belt",
        "Platinum Belt",
        "Belt",
        "Bat Belt",
        "Shoelace Belt",
        "Suspenders",
        "Rope",
        "Jeweled Belt",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();

    loot.foot = vec![
        "Slippers",
        "Hightops",
        "Boat shoes",
        "Studded Boots",
        "Leather Boots",
        "Linen Shoes",
        "Shoes",
        "Dress Shoe",
        "Ballet Shoe",
        "Riding Boots",
        "Slides",
        "Roller Skates",
        "Ice Skates",
        "Socks",
        "Flip Flops",
        "Platform Boots",
        "Stilletos",
        "Pumps",
        "Sandals",
        "Sneakers",
        "Cowboy Boots",
        "Snow Boots",
        "Ski Boots",
        "Fins",
        "High Heel Sandals",
        "Jeweled Sandals",
        "Wedge",
        "Rain Boots",
        "Cleats",
        "Brogues",
        "Derby Shoes",
        "Moccasins",
        "Espadrilles",
        "Loafers",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();

    loot.underwear = vec![
        "Thong",
        "Boxers",
        "Boxer Briefs",
        "Panties",
        "Jockstrap",
        "Tighty-whities",
        "Trunks",
        "Bikini",
        "Midway Briefs",
        "Boy Shorts",
        "String Bikini",
        "G-String",
        "Granny Panties",
        "Diaper",
        "Garter",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();

    loot.accessory = vec![
        "Necklace",
        "Amulet",
        "Pendant",
        "Chain",
        "Choker",
        "Cross Necklace",
        "Ear Buds",
        "Hoops",
        "Neck Tie",
        "Bow Tie",
        "Exclusive Pass",
        "Scarf",
        "Feather Boa",
        "Neck Pillow",
        "Ascot",
        "Hope Diamond",
        "Snake",
        "Stethoscope",
        "Shades",
        "Heart Glasses",
        "Nerd Glasses",
        "Goggles",
        "Ski Goggle",
        "Contact Lenses",
        "Monocle",
        "Green Contacts",
        "3D Glasses",
        "Sunglasses",
        "Blindfold",
        "VR Goggles",
        "Cyclops Glasses",
        "Aviator Sunglasses",
        "Spectacles",
        "Purse",
        "Backpack",
        "Tote Bag",
        "Grocery Bag",
        "Briefcase",
        "Suitcase",
        "Messenger Bag",
        "Wallet",
        "Crossbody Bag",
        "Hat Bag",
        "Golf Bag",
        "Portfolio Bag",
        "Tackle Box",
        "Tennis Bag",
        "Steamer Trunk",
        "Tool Box",
        "Guitar Case",
        "Watch",
        "Smart Watch",
        "Casio Watch",
        "Atomic Watch",
        "Festival Wristband",
        "Engagement Ring",
        "Antique Watch",
        "Wristband",
        "Scrunchie",
        "Bracelet",
        "Kandi Bracelet",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();

    loot.pants = vec![
        "Shorts",
        "Skirt",
        "Jeans",
        "Skort",
        "Sweat Pants",
        "Yoga Pants",
        "Kilt",
        "Track Pants",
        "Cargo Shorts",
        "Skinny Jeans",
        "Cargo Pants",
        "Riding Pants",
        "Cutoff Pants",
        "Baggy Pants",
        "Tights",
        "Dress Pants",
        "Bell Bottoms",
        "Karate Pants",
        "Miniskirt",
        "Tennis Skirt",
        "Chaps",
        "Leggings",
        "Crop Pants",
        "Trousers",
        "Hot Pants",
        "Swim Trunks",
        "Capri Pants",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();

    loot.prefixes = vec![
        "Second Hand",
        "Aged",
        "Worn Out",
        "Brand New",
        "Used",
        "Old",
        "2yr Old",
        "5yr Old",
        "3yr Old",
        "2021",
        "1980s",
        "1990s",
        "1970s",
        "1960s",
        "1950s",
        "1920s",
        "1800s",
        "Medieval",
        "Futuristic",
        "Vintage",
        "Retro",
        "Victorian",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();

    loot.name_prefixes = vec![
        "Gold",
        "Shiny",
        "Psychedelic",
        "Blazing",
        "Cheetah Print",
        "Leopard Print",
        "Rainbow",
        "Camo",
        "Neon",
        "Flashing",
        "Hot Pink",
        "Checkered",
        "Pinstripe",
        "Icy",
        "Silver",
        "Tie Dye",
        "Steampunk",
        "Sparkly",
        "Plaid",
        "Anime",
        "Colorful",
        "Floral",
        "8 Bit",
        "Blood Red",
        "Icy Blue",
        "Forest Green",
        "Bright Yellow",
        "Mystic Purple",
        "Night Black",
        "Cloud White",
        "Glittery",
        "Goth",
        "Emo",
        "Punk",
        "Preppy",
        "Electric Swirl",
        "Fruit Print",
        "Paisley",
        "Crunchy",
        "Aesthetic",
        "Boho",
        "Military",
        "Velour",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();

    loot.name_suffixes = vec![
        "Dirty",
        "Clean",
        "Designer",
        "Synthetic",
        "Wet",
        "Alien",
        "Organic",
        "Vegan",
        "Kawaii",
        "Badass",
        "Cute",
        "Sexy",
        "Beautiful",
        "Ugly",
        "Fire",
        "Cruelty Free",
        "Collectors Edition",
        "Limited Edition",
        "Drippin",
        "Stylin",
        "Custom",
        "Imaginary",
        "Knockoff",
        "Fake",
        "Signed",
        "Cheap",
        "Expensive",
        "Hip",
        "Chic",
        "Trendy",
        "Casual",
        "Formal",
        "Monogramed",
        "Haute Couture",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();

    let loot2 = storage::get_mut::<Loot2>();
    *loot2 = Loot2 {
        weapons: loot.weapons.clone(),
        waist: loot.waist.clone(),
        chest: loot.chest.clone(),
        head: loot.head.clone(),
        foot: loot.foot.clone(),
        underwear: loot.underwear.clone(),
        accessory: loot.accessory.clone(),
        pants: loot.pants.clone(),
        prefixes: loot.prefixes.clone(),
        name_prefixes: loot.name_prefixes.clone(),
        name_suffixes: loot.name_suffixes.clone(),
    }
}

#[query]
fn user_tokens(user: Principal) -> Vec<u64> {
    return storage::get::<AddressBook>().user_tokens(&user);
}

#[query]
fn supply() -> u64 {
    return storage::get::<AddressBook>().total_supply;
}

#[query]
fn remaining() -> u64 {
    return storage::get::<AddressBook>().remaining();
}

#[query]
fn owner_of(token_id: u64) -> Option<Principal> {
    return storage::get::<AddressBook>().owner_of(&token_id);
}

#[update]
fn transfer_to(user: Principal, token_id: u64) -> bool {
    return storage::get_mut::<AddressBook>().transfer_to(user, token_id);
}

#[update]
async fn transfer_with_notify(
    user_id: Principal, token_id: u64
) -> bool {
    let address_book = storage::get_mut::<AddressBook>();
    if address_book.transfer_to(user_id, token_id) {
        match
            ic_cdk::call(user_id, "transfer_notification", ( 
                TransferNotification{
                    to: user_id,
                    from: ic_cdk::caller(),
                    token_id: token_id,
                    amount: 1
                },)
            ).await as CallResult<()> {
                Ok(_) => return true,
                Err(_) => {
                    //gets in rejected state and the next
                    //line is not executed completely
                    //address_book.undo_transfer(user_id, token_id);
                    return false;
                }
            }
    } else {
        return false;
    }
}

#[update]
fn claim() -> Result<u64, String> {
    //return Err("No claims for this NFT type (IC DRIP)".to_string());
    return storage::get_mut::<AddressBook>().claim(ic_cdk::caller());
}

//Allow the original airdrop to always exists for future references
//where sites can use this to know if the person transferred their NFT or not.
#[query]
fn get_airdrops() -> Vec<(u64, bool)> {
    let airdroppers = storage::get_mut::<BTreeMap<Principal, Vec<u64>>>();
    let address_book = storage::get_mut::<AddressBook>();
    match airdroppers.get(&ic_cdk::caller()) {
        Some(tokens) => {
            let mut results: Vec<(u64, bool)> = Vec::new();
            for token in tokens {
                results.push((
                    token.clone(),
                    address_book.is_owner_of(ic_cdk::caller(), token),
                ));
            }
            return results;
        }
        None => Vec::new(),
    }
}

//Save list of airdrops for other platforms to use.
fn update_airdroppers(user: Principal, token_id: u64) -> () {
    let airdroppers = storage::get_mut::<BTreeMap<Principal, Vec<u64>>>();
    match airdroppers.get_mut(&user) {
        Some(tokens) => tokens.push(token_id),
        None => {
            airdroppers.insert(user, vec![token_id]);
        }
    }
}

#[update(guard = "is_controller")]
fn add_airdrops(users: Vec<Principal>) -> bool {
    let address_book = storage::get_mut::<AddressBook>();
    for id in users {
        match address_book.claim(id) {
            Ok(token_id) => update_airdroppers(id, token_id),
            Err(_) => return false,
        }
    }
    return true;
}

#[update(guard = "is_controller")]
fn add_controller(user: Principal) -> bool {
    return storage::get_mut::<AddressBook>().add_controller(&user);
}

#[update(guard = "is_controller")]
fn remove_controller(user: Principal) -> bool {
    return storage::get_mut::<AddressBook>().remove_controller(&user);
}

#[update(guard = "is_controller")]
fn get_controllers() -> Vec<Principal> {
    return storage::get::<AddressBook>().controllers.clone();
}

#[update]
fn get_cycles() -> i64 {
    return ic_cdk::api::call::msg_cycles_available();
}

#[query]
fn name() -> String {
    return "IC_DRIP".to_string();
}

#[query]
fn symbol() -> String {
    return "IC_DRIP".to_string();
}

#[query]
fn get_address_book() -> AddressBook {
    return storage::get::<AddressBook>().clone();
}

type HeaderField = (String, String);

#[derive(Clone, Debug, CandidType, Deserialize)]
struct HttpRequest {
    method: String,
    url: String,
    headers: Vec<(String, String)>,
    body: ByteBuf,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
struct HttpResponse {
    status_code: u16,
    headers: Vec<HeaderField>,
    body: Vec<u8>,
}

#[query]
async fn http_request(req: HttpRequest) -> HttpResponse {
    let parts: Vec<&str> = req.url.split('?').collect();

    let token_param: Vec<&str> = parts[1].split('=').collect();
    let token_id = token_param[1].parse::<u64>().unwrap();

    let address_book = storage::get_mut::<AddressBook>();

    if token_id <= 0 || token_id > address_book.total_supply || !address_book.is_claimed(&token_id)
    {
        return HttpResponse {
            status_code: 404,
            headers: Vec::new(),
            body: Vec::new(),
        };
    }

    let loot = storage::get_mut::<Loot>();

    let seed = address_book.token_seeds.get(&token_id).unwrap();

    let data = loot.generate(token_id.clone() + seed.clone());

    let results = data.as_bytes();

    let mut headers: Vec<HeaderField> = Vec::new();
    headers.push(("content-type".to_string(), "image/svg+xml".to_string()));
    headers.push((
        "cache-control".to_string(),
        "public, max-age=604800, immutable".to_string(),
    ));
    return HttpResponse {
        status_code: 200,
        headers,
        body: results.to_vec(),
    };
}

#[query]
fn data_of(token_id: u64) -> Vec<LootData> {
    let address_book = storage::get::<AddressBook>();
    if token_id <= 0 || token_id > address_book.total_supply || !address_book.is_claimed(&token_id)
    {
        return Vec::new();
    }
    let seed = address_book.token_seeds.get(&token_id).unwrap();
    let loot = storage::get::<Loot2>();
    return loot.get_properties(token_id + seed);
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub enum DataOfQuery {
    Range(u64, u64),
    List(Vec<u64>),
}

#[query]
fn data_of_many(query: DataOfQuery) -> BTreeMap<u64, Vec<LootData>> {
    let address_book = storage::get::<AddressBook>();
    match query {
        DataOfQuery::Range(from, to) => {
            let mut results = BTreeMap::new();
            for i in from..to + 1 {
                if !address_book.is_claimed(&i) {
                    continue;
                }
                let seed = address_book.token_seeds.get(&i).unwrap();
                let loot = storage::get::<Loot2>();
                results.insert(i, loot.get_properties(i + seed));
            }
            return results;
        }
        DataOfQuery::List(items) => {
            let mut results = BTreeMap::new();
            for id in items {
                if !address_book.is_claimed(&id) {
                    continue;
                }
                let seed = address_book.token_seeds.get(&id).unwrap();
                let loot = storage::get::<Loot2>();
                results.insert(id, loot.get_properties(id + seed));
            }
            return results;
        }
    }
}

#[query]
fn get_token_properties(token_id: u64) -> Vec<(String, String)> {
    let address_book = storage::get::<AddressBook>();
    if token_id <= 0 || token_id > address_book.total_supply || !address_book.is_claimed(&token_id)
    {
        return Vec::new();
    }
    let seed = address_book.token_seeds.get(&token_id).unwrap();
    let loot = storage::get::<Loot>();
    return loot.get_properties(token_id + seed);
}

#[query]
fn get_token_properties_range(from: u64, to: u64) -> Vec<Vec<(String, String)>> {
    let address_book = storage::get::<AddressBook>();
    let mut results = Vec::new();
    for i in from..to + 1 {
        if !address_book.is_claimed(&i) {
            continue;
        }
        let seed = address_book.token_seeds.get(&i).unwrap();
        let loot = storage::get::<Loot>();
        results.push(loot.get_properties(i + seed))
    }
    return results;
}

//this is not working correctly.
#[query(name = "__get_candid_interface_tmp_hack")]
fn export_candid() -> String {
    return r#"

//    http://icdrip.io
//    STAGING ENVIRONMENT

//    ██▓ ▄████▄     ▓█████▄  ██▀███   ██▓ ██▓███  
//    ▓██▒▒██▀ ▀█     ▒██▀ ██▌▓██ ▒ ██▒▓██▒▓██░  ██▒
//    ▒██▒▒▓█    ▄    ░██   █▌▓██ ░▄█ ▒▒██▒▓██░ ██▓▒
//    ░██░▒▓▓▄ ▄██▒   ░▓█▄   ▌▒██▀▀█▄  ░██░▒██▄█▓▒ ▒
//    ░██░▒ ▓███▀ ░   ░▒████▓ ░██▓ ▒██▒░██░▒██▒ ░  ░
//    ░▓  ░ ░▒ ▒  ░    ▒▒▓  ▒ ░ ▒▓ ░▒▓░░▓  ▒▓▒░ ░  ░
//     ▒ ░  ░  ▒       ░ ▒  ▒   ░▒ ░ ▒░ ▒ ░░▒ ░     
//     ▒ ░░            ░ ░  ░   ░░   ░  ▒ ░░░       
//     ░  ░ ░            ░       ░      ░           
//        ░            ░                            
 
type HeaderField = record { text; text; };

type HttpRequest = record {
    method: text;
    url: text;
    headers: vec HeaderField;
    body: blob;
};

type HttpResponse = record {
    status_code: nat16;
    headers: vec HeaderField;
    body: blob;
};

type AddressBook = record {
    total_supply: nat64;
    tokens: vec record { nat64; principal};
    controllers: vec principal;
    claim_index: nat64;
    token_seeds: vec record { nat64; nat64};
};

type ClaimResult = variant {
    Ok : nat64;
    Err: text;
};

type TransferNotification = record {
    to: principal;
    from: principal;
    token_id: nat64;
    amount: nat64;
  };

type LootData = record {
    slot: text;
    name: text;

    prefix: text;
    name_prefix: text;
    name_suffix: text;
    special: bool;
};

type DataOfQuery = variant {
    Range: record {nat64; nat64};
    List: vec nat64;
};
  
service : {
    http_request: (request: HttpRequest) -> (HttpResponse) query;

    get_address_book: () -> (AddressBook) query;

    get_token_properties: (nat64) -> (vec record { text; text}) query;
    get_token_properties_range: (nat64, nat64) -> (vec vec record { text; text}) query;

    data_of: (nat64) -> (vec LootData) query;
    data_of_many: (DataOfQuery) -> (vec record {nat64; vec LootData;}) query;

    get_cycles: () -> (int64);
    get_airdrops: () -> (vec record { nat64; bool }) query;
    add_airdrops: (vec principal) -> (bool);
    name: () -> (text) query;
    symbol: () -> (text) query;
    user_tokens: (principal) -> (vec nat64) query;
    owner_of: (nat64) -> (opt principal) query;
    transfer_to: (principal, nat64) -> (bool);
    transfer_with_notify: (principal, nat64) -> (bool);
    claim: () -> (ClaimResult);
    remaining: () -> (nat64);

    get_controllers: () -> (vec principal) query;
    add_controller: (principal) -> (bool);
    remove_controller: (principal) -> (bool);
    supply: () -> (nat64);
}
    "#
    .to_string();
}

#[derive(CandidType, Deserialize)]
struct StableStorage {
    address_book: AddressBook,
    airdroppers: BTreeMap<Principal, Vec<u64>>,
}

#[pre_upgrade]
fn pre_upgrade() {
    let stable = StableStorage {
        address_book: storage::get::<AddressBook>().clone(),
        airdroppers: storage::get::<BTreeMap<Principal, Vec<u64>>>().clone(),
    };

    match storage::stable_save((stable,)) {
        Ok(_) => (),
        Err(candid_err) => {
            ic_cdk::trap(&format!(
                "An error occurred when saving to stable memory (pre_upgrade): {}",
                candid_err
            ));
        }
    };
}

#[post_upgrade]
fn post_upgrade() {
    init();
    if let Ok((storage,)) = storage::stable_restore::<(StableStorage,)>() {
        let address_book = storage::get_mut::<AddressBook>();
        *address_book = storage.address_book;
        let airdroppers = storage::get_mut::<BTreeMap<Principal, Vec<u64>>>();
        *airdroppers = storage.airdroppers;
    }
}

fn is_controller() -> Result<(), String> {
    if storage::get::<AddressBook>().is_controller(&ic_cdk::caller()) {
        Ok(())
    } else {
        Err("Only the controller can call this method.".to_string())
    }
}
