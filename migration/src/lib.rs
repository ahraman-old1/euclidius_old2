use sea_orm::{ConnectOptions, Database, DbErr};
use sea_orm_migration::{MigrationTrait, MigratorTrait};

mod v0_1_0;
use v0_1_0::*;

pub struct Migrator;

impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            // 0.1.0
            Box::new(m20240702_000001_create_users_table::Migration),
        ]
    }
}

#[derive(Debug, Clone)]
pub struct DatabaseAddress {
    pub backend: String,
    pub username: String,
    pub password: String,
    pub url: String,
    pub name: String,
}

impl DatabaseAddress {
    pub fn from_env() -> Result<Self, std::env::VarError> {
        Ok(Self {
            backend: std::env::var("DB_BACKEND")?,
            username: std::env::var("DB_USER")?,
            password: std::env::var("DB_PASS")?,
            url: std::env::var("DB_URL")?,
            name: std::env::var("DB_NAME")?,
        })
    }

    pub fn to_string(&self) -> String {
        format!(
            "{}://{}:{}@{}/{}",
            self.backend, self.username, self.password, self.url, self.name
        )
    }
}

pub async fn run(address: DatabaseAddress) -> Result<(), DbErr> {
    let database_url = address.to_string();
    let options = ConnectOptions::new(&database_url);

    let conn = Database::connect(options).await?;
    Migrator::up(&conn, None).await?;

    Ok(())
}
