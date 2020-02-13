use std::process;

use crate::config;
use crate::drivers::mysql;
use crate::structs::Migration;

pub fn query(query: String) -> Vec<Migration> {
    if !config::exists() {
        println!("you must generate a config file first");
        println!("use `rmig init` to do this");
        process::exit(1);
    }

    let config = config::load().unwrap();

    if config.platform == "mysql" {
        return mysql::query(config, query)
    }

    let migrations: Vec<Migration> = Vec::new();

    migrations
}
