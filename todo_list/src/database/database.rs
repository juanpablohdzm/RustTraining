use rusqlite::{Connection};
use crate::database::Item;

pub struct DataBase {
    connection: Connection,
    cached_items: Option<Vec<Item>>,
}

impl DataBase {
    pub fn new(db_path: &str) -> Result<Self, rusqlite::Error> {
        let connection  = DataBase::connect(db_path)?;
        let db = DataBase {
            connection,
            cached_items: None,
        };
        Ok(db)
    }

    fn connect(path: &str) -> Result<Connection,rusqlite::Error> {
        let conn = Connection::open(path)?;
        conn.execute("
            CREATE TABLE IF NOT EXISTS items (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL
            )", [])?;
        Ok(conn)
    }
    
    pub fn add_item(&mut self, item: Item) -> Result<(), rusqlite::Error> {
        let mut stmt = self.connection.prepare("INSERT INTO items (name) VALUES (?)")?;
        stmt.execute(rusqlite::params![item.get_name()])?;
        self.cached_items = None;
        Ok(())
    }
    
    pub fn remove_item(&mut self, id: i32) -> Result<(), rusqlite::Error> {
        let mut stmt = self.connection.prepare("DELETE FROM items WHERE id = ?")?;
        stmt.execute(rusqlite::params![id])?;
        
        self.cached_items = None;
        Ok(())
    }
    
    pub fn get_item_by_index(&mut self, index: usize) -> Option<&Item> {
        let items = self.get_items();
        if let Some(items) = items {
            return items.get(index);
        }
        None
    }
    
    pub fn get_items_length(&self) -> usize {
        match &self.cached_items {
            Some(items) => items.len(),
            None => {
                let mut stmt = self.connection.prepare("SELECT COUNT(*) FROM items").unwrap();
                let count: usize = stmt.query_row([], |row| row.get(0)).unwrap();
                count
            }
        }
    }
    
    pub fn get_items(&mut self) -> Option<&Vec<Item>> {
        
        if self.cached_items.is_none() {
            // Cache the items if not already cached
            let mut stmt = self.connection.prepare("SELECT * FROM items").expect("Failed to select all from items");
            let item_iter = stmt.query_map([], |row| {
                Ok(Item::new(row.get(0)?, row.get(1)?))
            }).expect("Failed to map items");
    
            let mut items = Vec::new();
            for item in item_iter {
                items.push(item.expect("Failed to map item"));
            }
            
            self.cached_items = Some(items.clone());
        }

        self.cached_items.as_ref()
    }
}