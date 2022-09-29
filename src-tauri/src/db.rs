pub mod db {
    use rusqlite::{Connection, Result};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    pub struct PostgresConnection {
        id: i32,
        name: String,
        conn: String,
        username: String,
        password: String,
    }

    pub async fn insert_connection(
        name: &str,
        conn_str: &str,
        username: &str,
        password: &str,
    ) -> Result<(), rusqlite::Error> {
        let conn = Connection::open("saturn.db")?;

        let query =
            "INSERT INTO connections (name, conn, username, password) VALUES(?1, ?2, ?3, ?4)";

        match conn.execute(query, (name, conn_str, username, password)) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub async fn get_connections() -> Result<Vec<PostgresConnection>> {
        let conn = Connection::open("saturn.db")?;
        let query = "SELECT * FROM connections";
        let mut stmt = conn.prepare(query)?;
        let conns = stmt.query_map([], |row| {
            Ok(PostgresConnection {
                id: row.get(0)?,
                name: row.get(1)?,
                conn: row.get(2)?,
                username: row.get(3)?,
                password: row.get(4)?,
            })
        })?;

        let mut result: Vec<PostgresConnection> = Vec::new();
        for row in conns {
            result.push(row.unwrap());
        }
        Ok(result)
    }

    pub fn generate_schema(){
        let conn = Connection::open("saturn.db").unwrap();
        match conn.execute(
            "
        CREATE TABLE IF NOT EXISTS connections (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name VARCHAR NOT NULL, 
            conn VARCHAR NOT NULL,
            username VARCHAR NOT NULL,
            password VARCHAR NOT NULL
        );",
            [],
        ) {
            Ok(_) => println!("Created connections table"),
            Err(e) => panic!("{}", e),
        }

        match conn.execute(
            "
            CREATE TABLE IF NOT EXISTS storedQueries (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name VARCHAR NOT NULL, 
                query VARCHAR NOT NULL
            );",
            [],
        ){
            Ok(_) => println!("Created storedQueries table"),
            Err(e) => panic!("{}", e),
        }
    }
}
