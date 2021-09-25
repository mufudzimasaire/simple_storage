use std::collections::HashMap;

type Key = String;
type Value = String;

pub struct Database {
	data: HashMap<Key, Value>
}

impl Database {
	fn new() -> Result<Database, std::io::Error> {
		let mut map = HashMap::new();

		// Read stored data
		let stored_data = std::fs::read_to_string("database.db").unwrap_or_default();
		
		// Populate map with the retrieved data
		for line in stored_data.lines() {
			let (key, value) = line.split_once('\t').expect("Database is corrupted!");
			map.insert(key.to_owned(), value.to_owned());
		}

		// Return populated database
		Ok(Database { data: map })
	}

	pub fn insert(key: Key, value: Value) {
		let mut new_data = String::new();
		let mut db = Database::new().expect("Could not initialize database");

		// Insert new data in the DB
		db.data.insert(key, value);

		// Build data for storage
		for (key, value) in &db.data {
			new_data.push_str(&format!("{}\t{}\n", key, value));
		}
		
		// Store data
		let _result = std::fs::write("database.db", new_data);
	}
}
