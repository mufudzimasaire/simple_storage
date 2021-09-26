use std::collections::HashMap;

type Key = String;
type Value = String;

pub struct Database {
	data: HashMap<Key, Value>
}

impl Database {
	/**
	 * Instantiates the DB
	 */
	pub fn new() -> Result<Database, std::io::Error> {
		let mut map = HashMap::new();
		let db_name = "database.db";

		if  std::path::Path::new(db_name).exists() {
			// Read stored data
			let stored_data = std::fs::read_to_string(db_name).unwrap();

			// Populate map from the retrieved data
			for line in stored_data.lines() {
				let (key, value) = line.split_once('\t').expect("Database is corrupted!");
				map.insert(key.to_owned(), value.to_owned());
			}
		}

		// Return database
		Ok(Database { data: map })
	}

	/**
	 * Inserts a `key` and `value` pair into the DB
	 */
	pub fn insert(&mut self, key: Key, value: Value) {
		// Insert new data in the DB
		self.data.insert(key.to_lowercase(), value);

		// Build data for storage
		let mut new_data = String::new();

		for (key, value) in &self.data {
			new_data.push_str(&format!("{}\t{}\n", key, value));
		}

		// Store data
		let _result = std::fs::write("database.db", new_data);
	}

	/**
	 * Returns all DB entries
	 */
	pub fn read_all (&self) {
		println!("All entries");

		for (key, value) in &self.data {
			println!("{}: {}", key, value)
		}
	}
}
