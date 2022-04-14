use std::env;

use dotenv::dotenv;

pub struct Config {
    pub host: String,
    pub port: u32,
}

impl Config {
    pub fn read_config() -> Config {
        dotenv().ok();
        Config {
            host: env::var("HOST").expect("env HOST not found"),
            port: env::var("PORT")
                .expect("env PORT not found")
                .parse()
                .unwrap(),
        }
    }
}
