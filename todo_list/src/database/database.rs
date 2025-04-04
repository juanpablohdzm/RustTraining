use rusqlite::{Connection};
use crate::database::Item;

pub struct DataBase {
    db_path: String,
    connection: Connection
}

impl DataBase {
    pub fn new(db_path: &str) -> Result<Self, rusqlite::Error> {
        let connection  = DataBase::connect(db_path)?;
        let db = DataBase {
            db_path: db_path.to_string(),
            connection
        };
        Ok(db)
    }

    fn connect(path: &str) -> Result<Connection,rusqlite::Error> {
        let conn = Connection::open(path)?;
        if let Err(e) = conn.execute("
        CREATE TABLE IF NOT EXISTS items (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL
        )", []) {
            return Err(e);
        }
        Ok(conn)
    }
    
    pub fn add_item(&self, item: &Item) -> Result<(), rusqlite::Error> {
        let mut stmt = self.connection.prepare("INSERT INTO items (id, name) VALUES (?, ?)")?;
        stmt.execute(rusqlite::params![item.get_id(), item.get_name()])?;
        Ok(())
    }
    
    pub fn remove_item(&self, id: i32) -> Result<(), rusqlite::Error> {
        let mut stmt = self.connection.prepare("DELETE FROM items WHERE id = ?")?;
        stmt.execute(rusqlite::params![id])?;
        Ok(())
    }
    
    pub fn get_items(&self) -> Result<Vec<Item>, rusqlite::Error> {
        let mut stmt = self.connection.prepare("SELECT * FROM items")?;
        let item_iter = stmt.query_map([], |row| {
            Ok(Item::new(row.get(0)?, row.get(1)?))
        })?;

        let mut items = Vec::new();
        for item in item_iter {
            items.push(item?);
        }

        Ok(items)
    }
}