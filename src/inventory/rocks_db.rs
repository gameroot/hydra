//! Manages RocksDB embedded kv store.
extern crate rocksdb;
use self::rocksdb::DB;

pub fn refresh(){
    //! Load data from JSON file and store to Rocks
    //! Clear Rocks
    //! Get JSON inventory
    //! Rewrite Rocks data


    let sentence = "This is a sentence in Rust.";
    let words: Vec<&str> = sentence
        .split_whitespace()
        .collect();
    let words_containing_i: Vec<&str> = words
        .into_iter()
        .filter(|word| word.contains("i"))
        .collect();
    println!("{:?}", words_containing_i);

    let db = DB::open_default(".carbondb").unwrap();
    
    store_key_to_rocks(&db, "0000-0000-0000-0000", "__UNSET__");
}  

fn store_key_to_rocks(db: &rocksdb::DB, k: &str, v: &str) {
    match db.put(k.as_bytes(), v.as_bytes()){
        Ok(_) => println!("set value in db"),
        Err(e) => println!("operational problem encountered: {}", e),
    }

    match db.get(k.as_bytes()) {
        Ok(Some(value)) => println!("retrieved value {}", value.to_utf8().unwrap()),
        Ok(None) => println!("value not found"),
        Err(e) => println!("operational problem encountered: {}", e),
    }
}