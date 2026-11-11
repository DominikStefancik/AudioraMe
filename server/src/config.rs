use std::env;

pub struct ConfigManager {
    pub database_url: String,
    pub port: u16,
}

impl ConfigManager {
    pub fn init() -> Self {
        let database_url = env::var("DATABASE_URL")
            .expect("The DATABASE_URL variable must be set in .env file or environment")
            .trim()
            .into();
        let port: u16 = env::var("PORT")
            .expect("The PORT variable must be set in .env file or environment")
            .trim()
            .parse()
            .expect("The PORT variable must be a number");

        ConfigManager { database_url, port }
    }
}
