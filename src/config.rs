/***
The MIT License (MIT)

Copyright (c) 2020 Brian Seymour

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
THE SOFTWARE.
***/

use std::process;
use std::path::Path;
use std::env;
use std::fs::File;
use std::io::Read;

use crate::structs::Config;

pub fn load() -> Result<Config, &'static str> {
    let exists = Path::new("config.json").exists();

    if exists {
        let mut handle = File::open("config.json")
            .unwrap_or_else(|_| {
                println!("config file not found, use `rmig init` to make one");
                process::exit(1);
            });

        let mut contents = String::new();

        handle.read_to_string(&mut contents)
            .unwrap_or_else(|_| {
                println!("problem parsing config, recommend `rmig init -f` to redo");
                process::exit(1);
            });

        let config: Config = serde_json::from_str(&contents)
            .unwrap_or_else(|_| {
                println!("problem parsing config, most commonly because the json is invalid");
                println!("this can be fixed by either:");
                println!("  a) manually fixing the config.json and making it valid");
                println!("  b) running `rmig init -f` to generate a new config");
                process::exit(1);
            });

        return Ok(config);
    } else {
        let config = Config {
            platform: get_env_string("RMIG_PLATFORM"),
            host: get_env_string("RMIG_HOST"),
            port: get_env_int("RMIG_PORT"),
            user: get_env_string("RMIG_USER"),
            pass: get_env_string("RMIG_PASS"),
            db: get_env_string("RMIG_DB"),
        };

        return Ok(config);
    }
}

pub fn get_env_string(name: &str) -> String {
    match env::var(name) {
        Ok(val) => val,
        Err(_) => "".to_string()
    }
}

pub fn get_env_int(name: &str) -> i32 {
    match env::var(name) {
        Ok(val) => match val.parse::<i32>() {
            Ok(val) => val,
            Err(_) => 0
        },
        Err(_) => 0
    }
}

pub fn exists() -> bool {
    match File::open("config.json") {
        Ok(_) => true,
        _ => false
    }
}
