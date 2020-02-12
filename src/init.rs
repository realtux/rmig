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

use std::path::Path;
use text_io::read;
use std::io::{stdout,Write};
use std::fs;

use crate::structs::Flags as Flags;
use crate::structs::Config as Config;

pub fn handle(flags: Flags) {
    println!("beginning init process..");

    if Path::new("config.json").exists() {
        println!("config already exists, use -f to force");
        return;
    }

    print!("host [localhost]: ");
    let _ = stdout().flush();
    let host: String = read!("{}\n");

    print!("database username [root]: ");
    let _ = stdout().flush();
    let user: String = read!("{}\n");

    print!("database password [root]: ");
    let _ = stdout().flush();
    let pass: String = read!("{}\n");

    print!("database name: ");
    let _ = stdout().flush();
    let db: String = read!("{}\n");

    print!("database platform: ");
    print!("  [1] mysql");
    print!("");
    print!("choice: ");

    let platform: String = read!("{}\n");
    let platform: &str = match platform.as_str() {
        "1" => "mysql",
        _ => "mysql"
    };
    let platform: String = platform.to_string();

    let mut config = Config { host, user, pass, db, platform };

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