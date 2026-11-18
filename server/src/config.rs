use std::env;

#[derive(Debug, Clone)]
pub struct ConfigManager {
    pub database_url: String,
    pub port: u16,
    pub jwt_secret: String,
    pub jwt_max_duration_minutes: u16,
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
        let jwt_secret = env::var("JWT_SECRET")
            .expect("The JWT_SECRET variable must be set in .env file or environment")
            .trim()
            .into();
        let jwt_max_duration_minutes: u16 = env::var("JWT_MAX_DURATION_MINUTES")
            .expect("The JWT_MAX_DURATION_MINUTES variable must be set in .env file or environment")
            .trim()
            .parse()
            .expect("The JWT_MAX_DURATION_MINUTES variable must be a number");

        ConfigManager {
            database_url,
            port,
            jwt_secret,
            jwt_max_duration_minutes,
        }
    }
}
