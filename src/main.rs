use std::{thread::sleep, time::Duration};
use chrono::prelude::*;
use rusqlite::{Connection, Result};

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>,
}

fn main() -> Result<()> {
    let conn = Connection::open_in_memory()?;

    conn.execute(
        "CREATE TABLE person (
            id    INTEGER PRIMARY KEY,
            name  TEXT NOT NULL,
            data  BLOB
        )",
        (), // empty list of parameters.
    )?;
    let me = Person {
        id: 0,
        name: "Steven".to_string(),
        data: None,
    };
    conn.execute(
        "INSERT INTO person (name, data) VALUES (?1, ?2)",
        (&me.name, &me.data),
    )?;
    loop{
        let start: DateTime<Local> = Local::now(); 
        let mut stmt = conn.prepare("SELECT id, name, data FROM person")?;
        let end: DateTime<Local> = Local::now();
        print!("{},", (end - start).num_microseconds().unwrap()); 
        let person_iter = stmt.query_map([], |row| {
            Ok(Person {
                id: row.get(0)?,
                name: row.get(1)?,
                data: row.get(2)?,
            })
        })?;


        for person in person_iter {
            println!("Found person {:?}", person.unwrap());
        }
        sleep(Duration::from_millis(1000));
    }
    Ok(())
}