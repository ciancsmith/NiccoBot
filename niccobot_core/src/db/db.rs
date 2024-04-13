use sqlx::{Sqlite, SqlitePool, migrate::MigrateDatabase,
           Error};
use tracing::info;
use std::path::PathBuf;

pub struct DB {
    pub url: String,
    pub tables: Tables,
    pub pool: SqlitePool,
}

impl DB {
    pub async fn new(url: &str, tables: Option<Vec<&str>>) -> DB {

        if !Sqlite::database_exists(url).await.unwrap_or(false) {
            match Sqlite::create_database(url).await {
                Ok(_) => println!("Database created"),
                Err(e) => println!("Error creating database: {}", e),
            }
        } else {
            println!("Database already exists");
        }

        let pool = SqlitePool::connect(url).await.unwrap();
        let url = url.to_string();
        let tables_vec: Vec<String>;

        match tables {
            Some(custom_tables) => {
                info!("Received custom tables for import");
                tables_vec = custom_tables.iter().map(|s| s.to_string()).collect();
            }
            None => {
                info!("Using default tables");
                tables_vec = vec!["champions"]
                    .iter()
                    .map(|&s| s.to_string())
                    .collect();

            }
        }

        let tables = Tables::new(tables_vec);
        info!("New SQLite Database instance created");
        DB { url , tables, pool}
    }

    pub async fn send_migrations(&self) {
        let migrations_dir = std::env::var("MIGRATIONS_DIR").expect("MIGRATIONS_DIR must be set");
        let migrations_path = PathBuf::from(migrations_dir);
        let migrator = sqlx::migrate::Migrator::new(migrations_path).await.unwrap();
        migrator.run(&self.pool).await.map(|_| "Migration Success".to_string());
    }

    async fn is_table_empty(&self, table_name: &String) -> Result<bool, Error> {
        // Construct the query string
        let query = format!("SELECT COUNT(*) FROM {}", table_name);

        // Execute the query
        let count: (i64,) = sqlx::query_as(&query)
            .fetch_one(&self.pool)
            .await?;

        // Check if the count is zero
        Ok(count.0 == 0)
    }

}

pub struct Tables {
    pub tables_vec: Vec<String>,
}

impl Tables {
    pub fn new(tables: Vec<String>) -> Tables {
        Tables { tables_vec: tables }
    }
}

