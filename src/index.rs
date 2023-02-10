use serde::{Deserialize, Serialize, __private::doc};
use std::{
    any::Any,
    collections::{HashMap, HashSet},
};
use uuid::Uuid;

use crate::document::{self, Document};

#[derive(Serialize, Deserialize, Default)]
pub struct Index {
    pub id: String,
    pub documents: HashSet<String>,
    pub map: HashMap<String, HashMap<String, HashSet<usize>>>,
}
impl Index {
    fn new() -> Index {
        Index {
            id: Uuid::new_v4().to_string(),
            documents: HashSet::new(),
            map: HashMap::new(),
        }
    }
    fn add_document(&mut self, document: &Document) {
        for (token, positions) in &document.index {
            self.map
                .entry(token.to_owned())
                .or_insert(HashMap::from([(
                    document.id.to_owned(),
                    positions.to_owned(),
                )]))
                .entry(document.id.to_owned())
                .or_insert(positions.to_owned());
        }
    }
    fn from(bin: &Vec<u8>) -> Index {
        let index: Index = rmp_serde::decode::from_slice(bin).unwrap();
        return index;
    }
    fn serialize(&self) -> Vec<u8> {
        let bin = rmp_serde::encode::to_vec(self).unwrap();
        return bin;
    }
}
