use std::{collections::HashMap, env::Args, iter::Skip, io::Error, path::Path};
use std::fs::{read_to_string, write};

type Key = String;
type Value = String;

pub struct Database {
  data: HashMap<Key, Value>
}

impl Database {
  /**
   * Instantiates the DB
   */
  pub fn new() -> Result<Database, Error> {
    let mut map = HashMap::new();
    let db_name = "database.db";

    if  Path::new(db_name).exists() {
      // Read stored data
      let stored_data = read_to_string(db_name).unwrap();

      // Populate map from the retrieved data
      for line in stored_data.lines() {
        let (key, value) = line.split_once('\t').expect("Database is corrupted!");
        map.insert(key.to_owned(), value.to_owned());
      }
    }

    // Return database
    return Ok(Database { data: map })
  }

  /**
   * Dynamically performs the operation specified in the args
   */
  pub fn call (&mut self, mut args: Skip<Args>) {
    let operation = args.next().expect("Invalid operation");
    
    match &*operation {
      "insert" => {
        let key = args.next().expect("Invalid key input");
        let value = args.next().expect("Invalid value input");
        self.insert(key, value);
      },
      "select" => {
        let key = args.next().expect("Invalid key input");
        self.select(key);
      },
      "list" => {
        self.list();
      },
      "delete" => {
        let key = args.next().expect("Invalid key input");
        self.delete(key);
      },
      "flush" => {
        self.flush();
      },
      _ => {
        println!("Cannot not perform the specified operation!")
      },
    }
  }

  /**
   * Inserts a `key` and `value` pair into the DB
   */
  pub fn insert(&mut self, key: Key, value: Value) {
    // Insert new data in the DB
    self.data.insert(key.to_lowercase(), value);
    self.save();
  }

  /**
   * Returns the associated value
   */
  pub fn select (&self, key: Key) {
    let value = &self.data.get(&key.to_lowercase()).expect("Key not found!");
    println!("{}", value)
  }

  /**
   * Returns all DB entries
   */
  pub fn list (&self) {
    println!("All entries");

    for (key, value) in &self.data {
      println!("{}: {}", key, value)
    }
  }

  /**
   * Deletes the `key` and associated `value` from the DB
   */
  pub fn delete (&mut self, key: Key) {
    let _result = self.data.remove(&key);
    self.save();
  }

  /**
   * Flushes the contents of the DB
   */
  pub fn flush (&mut self) {
    self.data.clear();
    self.save();
  }

  /**
   * Persists the data
   */
  fn save (&self) {
    // Build data for storage
    let mut new_data = String::new();

    for (key, value) in &self.data {
      new_data.push_str(&format!("{}\t{}\n", key, value));
    }

    // Store data
    let _result = write("database.db", new_data);
  }
}
