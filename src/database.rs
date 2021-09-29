use std::{collections::HashMap, env::Args, iter::Skip, io::Error, path::Path};
use std::fs::{read_to_string, write};

type Key = String;
type Value = String;

const DATABASE_NAME: &str = "database.db";
const INVALID_KEY: &str = "Invalid key input!";
const INVALID_OPERATION: &str = "Cannot not perform the specified operation!";
const INVALID_VALUE: &str = "Invalid value input!";
const IS_CORRUPTED: &str = "Database is corrupted!";
const KEY_NOT_FOUND: &str = "Key not found!";

pub struct Database {
  data: HashMap<Key, Value>
}

impl Database {
  /**
   * Instantiates the DB
   */
  pub fn new() -> Result<Self, Error> {
    let mut map = HashMap::new();

    if  Path::new(DATABASE_NAME).exists() {
      // Read stored data
      let stored_data = read_to_string(DATABASE_NAME).unwrap();

      // Populate map from the retrieved data
      for line in stored_data.lines() {
        let (key, value) = line.split_once('\t').expect(IS_CORRUPTED);
        map.insert(key.to_owned(), value.to_owned());
      }
    }

    // Return database
    return Ok(Self { data: map })
  }

  /**
   * Dynamically performs the operation specified in the args
   */
  pub fn call (&mut self, mut args: Skip<Args>) {
    let operation = args.next().expect(INVALID_OPERATION);
    
    match &*operation {
      "insert" => {
        let key = args.next().expect(INVALID_KEY);
        let value = args.next().expect(INVALID_VALUE);
        self.insert(key, value);
      },
      "select" => {
        let key = args.next().expect(INVALID_KEY);
        self.select(key);
      },
      "list" => {
        self.list();
      },
      "delete" => {
        let key = args.next().expect(INVALID_KEY);
        self.delete(key);
      },
      "flush" => {
        self.flush();
      },
      _ => {
        println!("{}", INVALID_OPERATION)
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
    let value = &self.data.get(&key.to_lowercase()).expect(KEY_NOT_FOUND);
    println!("{}", value)
  }

  /**
   * Returns all DB entries
   */
  pub fn list (&self) {
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
    let _result = write(DATABASE_NAME, new_data);
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn insert_test () {
    // Arrange
    let key = "test";
    let value = "this is the value";
    let mut db = Database::new().unwrap();

    // Act
    db.insert(key.to_string(), value.to_string());

    // Assert
    assert_eq!(db.data.get(key).unwrap(), value);

    db.data.clear() // clean up
  }

  #[test]
  fn update_test () {
    // Arrange
    let key = "test";
    let initial_value = "this is the value";
    let updated_value = "updated value";
    let mut data = <HashMap<Key, Value>>::new();
    data.insert(key.to_string(), initial_value.to_string());
    let mut db = Database { data };

    // Act
    assert_eq!(db.data.get(key).unwrap(), initial_value);
    db.insert(key.to_string(), updated_value.to_string());

    // Assert
    assert_eq!(db.data.get(key).unwrap(), updated_value);

    db.data.clear() // clean up
  }

  #[test]
  fn delete_test () {
    // Arrange
    let key = "test";
    let value = "this is the value";
    let mut data = <HashMap<Key, Value>>::new();
    data.insert(key.to_string(), value.to_string());
    let mut db = Database { data };

    // Act
    db.delete(key.to_string());

    // Assert
    assert_eq!(db.data.get(key), None);

    db.data.clear() // clean up
  }

  #[test]
  fn flush_test () {
    // Arrange
    let key = "test";
    let value = "this is the value";
    let mut data = <HashMap<Key, Value>>::new();
    data.insert(key.to_string(), value.to_string());
    let mut db = Database { data };

    // Act
    db.flush();

    // Assert
    assert!(db.data.is_empty());
  }
}
