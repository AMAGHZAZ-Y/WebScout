#![allow(dead_code, unused)]
use flate2::{write::GzEncoder, Compression};
use std::{
    collections::{BTreeMap, BTreeSet, Bound, HashSet},
    fs,
    io::{read_to_string, Write},
    path::{Path, PathBuf},
};

use webscout::{
    document::Document,
    index::Index,
    query::Query,
    tokenizer::{self, Tokenizer},
    utils::standard_deviation,
};

fn serialize_docs() {
    let dir = fs::read_dir("assets/books").unwrap();
    let bin = fs::read("packs/en.pack").unwrap();
    let mut tokenizer = Tokenizer::from_pack(&bin);
    let mut idx: Index = Index::new();
    for file in dir {
        let path = file.as_ref().unwrap().path().to_owned();
        let name = file.as_ref().unwrap().file_name().into_string().unwrap();
        let mut body = fs::read_to_string(path).unwrap();
        println!("indexing {:}", name);
        let doc: Document = Document::new(&name, &mut body, "en", &mut tokenizer);
        idx.add_document(&doc);
        let content = doc.to_pack();
        fs::write(format!("temp/docs/{}.pack", name), content);
        fs::write(format!("temp/docs/{}.json", name), doc.to_json());
    }
    println!("saving index");
    let mut cmp = GzEncoder::new(Vec::new(), Compression::default());
    cmp.write_all(&idx.serialize());
    let bin = cmp.finish().unwrap();
    fs::write("temp/index/index.f2", bin);
    fs::write("temp/index/index.pack", idx.serialize());
    fs::write("temp/index/index.json", idx.to_json());
}

fn serialize_lemmers() {
    let dir = fs::read_dir("assets/lemmers").unwrap();
    for file in dir {
        let path = file.as_ref().unwrap().path().to_owned();
        let name = file.as_ref().unwrap().file_name().into_string().unwrap();
        let body = fs::read_to_string(path).unwrap();
        let mut tokenizer = Tokenizer::new(&name);
        tokenizer.construct_tokens(&body);
        let json = tokenizer.to_json();
        let pack = tokenizer.to_pack();
        fs::write(format!("packs/{}.pack", name), pack);
        fs::write(format!("temp/lang/{}.json", name), json);
    }
}
fn mean_score(values: &Vec<u32>) -> f32 {
    let mut len: f32 = values.len() as f32;
    let sum: u32 = values.iter().sum();
    sum as f32 / len
}
fn main() {
    // serialize_lemmers();
    // serialize_docs();
    let bin = fs::read("packs/en.pack").unwrap();
    let mut tokenizer = Tokenizer::from_pack(&bin);
    let index_bin = fs::read("temp/index/index.pack").unwrap();
    let mut index = Index::from(&index_bin);
    let mut query = Query::new("97217", &index, &mut tokenizer);
    query.search();
    // let mut book = fs::read_to_string("assets/books/Alcott-1.txt").unwrap();
    // let doc = Document::new("alcott", &mut book, "en", &mut tokenizer);
    // let data: Vec<f32> = vec![351.0, 350.0, 2000.0];
    // println!("{:?}", standard_deviation(&data))
    // let bin = fs::read("packs/en.pack").unwrap();
    // let mut tokenizer = Tokenizer::from_fs(&"en");
    // println!("{:?}", tokenizer.tokens.contains("hom"));
}
