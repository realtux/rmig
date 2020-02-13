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

use text_io::read;
use std::io::{stdout,Write};
use std::fs;
use std::process;

use crate::config;

use crate::structs::Flags;
use crate::structs::Config;

pub fn handle(flags: Flags) {
    println!("beginning init process..");

    if !flags.force {
        if config::exists() {
            println!("config already exists, use -f to force");
            process::exit(1);
        }
    }

    print!("host [localhost]: ");
    let _ = stdout().flush();
    let host: String = read!("{}\n");

    print!("port [3306]: ");
    let _ = stdout().flush();
    let port: String = read!("{}\n");
    let port: i32 = match port.parse::<i32>() {
        Ok(p) => p,
        _ => 3306
    };

    print!("database username [root]: ");
    let _ = stdout().flush();
    let user: String = read!("{}\n");

    print!("database password [root]: ");
    let _ = stdout().flush();
    let pass: String = read!("{}\n");

    print!("database name: ");
    let _ = stdout().flush();
    let db: String = read!("{}\n");

    println!("database platform:");
    println!("  [1] mysql");
    println!("");
    print!("choice [1]: ");
    let _ = stdout().flush();

    let platform: String = read!("{}\n");
    let platform: &'static str = match platform.parse::<i32>() {
        Ok(platform) => {
            match platform {
                1 => "mysql",
                _ => "mysql"
            }
        },
        _ => "mysql"
    };
    let platform: String = platform.to_string();

    let mut config = Config { host, port, user, pass, db, platform };

    if config.host == "" {
        config.host = "localhost".to_string();
    }

    if config.user == "" {
        config.user = "root".to_string();
    }

    if config.pass == "" {
        config.pass = "root".to_string();
    }

    let config_string = serde_json::to_string_pretty(&config).unwrap();

    fs::write("config.json", config_string)
        .expect("problem writing config");
}
