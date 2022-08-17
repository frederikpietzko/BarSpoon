use log::info;
use rusqlite::Connection;
pub struct Migration {
    pub id: i32,
    pub name: String,
    pub applied_at: String,
    pub sql: String,
}

impl Migration {
    fn apply(&self, conn: &Connection) -> Result<(), rusqlite::Error> {
        info!("Applying migration: {}", self.name);
        conn.execute(&self.sql, [])?;
        Ok(())
    }
}

pub struct MigrationManager<'a> {
    connection: &'a Connection,
    migrations: &'a Vec<Migration>,
}

impl<'a> MigrationManager<'a> {
    pub fn new(connection: &'a Connection, migrations: &'a Vec<Migration>) -> Self {
        Self {
            connection,
            migrations,
        }
    }

    pub fn migrate(&self) -> Result<(), rusqlite::Error> {
        self.connection.execute(
            "CREATE TABLE IF NOT EXISTS T_MIGRATIONS (
        id INTEGER PRIMARY KEY,
        name TEXT NOT NULL,
        applied_at TEXT NOT NULL,
        sql TEXT NOT NULL
      )",
            [],
        )?;
        let mut get_migrations = self
            .connection
            .prepare("SELECT id, name, applied_at, sql FROM T_MIGRATIONS")?;

        let mut applied_migrations = get_migrations
            .query_map([], |row| {
                Ok(Migration {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    applied_at: row.get(2)?,
                    sql: row.get(3)?,
                })
            })?
            .map(|m| m.expect("error while getting migration"));

        let migrations = self
            .migrations
            .iter()
            .filter(|migration| !applied_migrations.any(|am| am.name == migration.name));

        for migration in migrations {
            migration.apply(&self.connection)?;
        }
        Ok(())
    }
}
