use std::env;
use std::path::Path;
use text_io::read;
use std::io::{stdout,Write};
use serde::{Deserialize, Serialize};

static FLAG_TRANSACTION: bool = false;
static FLAG_BAIL: bool = false;

#[derive(Serialize, Deserialize)]
struct Config {
    host: String,
    user: String,
    pass: String,
    db: String
}

fn main() {
    println!("rmig 0.0.1");

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        menu();
        return;
    }

    let command: &str = &args[1];

    if command == "init" {
        println!("beginning init process...");

        if Path::new("config.json").exists() {
            println!("config already exists");
            return;
        }

        print!("host [localhost]: ");
        let _ = stdout().flush();
        let host: String = read!("{}\n");

        print!("db user [root]: ");
        let _ = stdout().flush();
        let user: String = read!("{}\n");

        print!("db password [root]: ");
        let _ = stdout().flush();
        let pass: String = read!("{}\n");

        print!("db name: ");
        let _ = stdout().flush();
        let db: String = read!("{}\n");

        let mut config = Config { host, user, pass, db };

        if config.host == "" {
            config.host = "localhost".to_string();
        }

        if config.user == "" {
            config.user = "root".to_string();
        }

        if config.pass == "" {
            config.pass = "root".to_string();
        }

        let config_string = serde_json::to_string(&config).unwrap();

        println!("{}", config_string);
    }
}

fn menu() {
    println!("usage: rmig command");
    println!("    init");
    println!("        create the initial bmig structure and config");
    println!("");
    println!("    status");
    println!("        see the status of all migrations");
    println!("");
    println!("    create [name]");
    println!("        create a new migration");
    println!("");
    println!("    migrate");
    println!("        run all available migrations");
    println!("");
    println!("    rollback");
    println!("        rollback the last migration");
}
