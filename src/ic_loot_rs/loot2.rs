
use ic_cdk::export::{candid::{CandidType, Deserialize}};
use crate::rand::Rand;

//I created this because I did not want to 
//mess with the generator which is being used.

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct Loot2 {
    pub weapons: Vec<String>,
    pub chest: Vec<String>,
    pub head: Vec<String>,
    pub waist: Vec<String>,
    pub foot: Vec<String>,
    pub underwear: Vec<String>,
    pub accessory: Vec<String>,
    pub pants: Vec<String>,
    
    pub prefixes: Vec<String>,
    pub name_prefixes: Vec<String>,
    pub name_suffixes: Vec<String>,
}

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct LootData {
    pub slot: String,
    pub name: String,
    
    pub prefix: String,
    pub name_prefix: String,
    pub name_suffix: String,
    pub special: bool
}


impl Loot2 {

    pub fn get_weapon(&self, token_id: u64) -> LootData {
        self.compute(&self.weapons, 1, token_id, "weapon".to_string())
    }

    pub fn get_chest(&self, token_id: u64) -> LootData {
        self.compute(&self.chest, 222, token_id, "chest".to_string())
    }

    pub fn get_head(&self, token_id: u64) -> LootData {
        self.compute(&self.head, 333, token_id, "head".to_string())
    }

    pub fn get_waist(&self, token_id: u64) -> LootData {
        self.compute(&self.waist, 4444, token_id, "waist".to_string())
    }

    pub fn get_foot(&self, token_id: u64) -> LootData {
        self.compute(&self.foot, 55555, token_id, "foot".to_string())
    }

    pub fn get_underwear(&self, token_id: u64) -> LootData {
        self.compute(&self.underwear, 666666, token_id, "underwear".to_string())
    }

    pub fn get_accessory(&self, token_id: u64) -> LootData {
        self.compute(&self.accessory, 7777777, token_id, "accessory".to_string())
    }

    pub fn get_pants(&self, token_id: u64) -> LootData {
        self.compute(&self.pants, 88888888, token_id, "pants".to_string())
    }

    pub fn get_prefix(&self, rand: u64) -> String {
        return self.prefixes[rand as usize % &self.prefixes.len()].clone();
    }

    pub fn get_name_prefix(&self, rand: u64) -> String {
        return self.name_prefixes[rand as usize % &self.name_prefixes.len()].clone();
    }

    pub fn get_name_suffix(&self, rand: u64) -> String {
        return self.name_suffixes[rand as usize % &self.name_suffixes.len()].clone();
    }

    pub fn compute(&self, items: &Vec<String>, offset: u64, token_id: u64, kind: String) -> LootData {
        let rand = Rand::new(token_id + offset).rand();
        let item_index = rand as usize % items.len();

        let mut data = LootData::default();

        data.slot = kind.clone();
        data.name = items[item_index].clone();

        let greatness = rand % 21;

        if greatness > 14 {
            data.prefix = self.get_prefix(rand);
        } 
        if greatness > 19 {
            if greatness == 19 {
                data.name_prefix = self.get_name_prefix(rand);
                data.name_suffix = self.get_name_suffix(rand);
                data.special = false;
            } else {
                data.special = true;
                data.name_prefix = self.get_name_prefix(rand);
                data.name_suffix = self.get_name_suffix(rand);
            }
        } 
        return data;
    }

    pub fn get_properties(&self, token_id: u64) -> Vec<LootData> {
        return vec![
            self.get_weapon(token_id),
            self.get_chest(token_id),
            self.get_head(token_id),
            self.get_waist(token_id),
            self.get_pants(token_id),
            self.get_underwear(token_id),
            self.get_accessory(token_id),
            self.get_foot(token_id),
        ]
    }
}