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

use crate::config;
use crate::drivers::mysql;
use crate::structs::Config;
use crate::structs::Migration;

// process a normal query
pub fn query(query: String) {
    let config = get_config();

    if config.platform == "mysql" {
        mysql::query(query);
        return
    }
}

// get a list of local migrations compared with remote migrations
pub fn get_migration_list() -> Vec<Migration> {
    let config = get_config();

    if config.platform == "mysql" {
        return mysql::get_migration_list();
    }

    Vec::new()
}

// insert a new migration by name
pub fn add_migration(name: String) {
    let config = get_config();

    if config.platform == "mysql" {
        return mysql::query(format!("insert into rmig values ('{}')", name));
    }
}

// delete an existing migration by name
pub fn remove_migration(name: String) {
    let config = get_config();

    if config.platform == "mysql" {
        return mysql::query(format!("delete from rmig where name = '{}'", name));
    }
}

fn get_config() -> Config {
    // check for config available
    if !config::exists() {
        println!("you must generate a config file first");
        println!("use `rmig init` to do this");
        process::exit(1);
    }

    let config = config::load().unwrap();

    // verify platform is valid
    let valid_platforms = vec!["mysql"];

    if !valid_platforms.contains(&config.platform.as_str()) {
        println!("platform `{}` not supported by rmig", config.platform);
        println!("currently supported platforms: mysql");
        process::exit(1);
    }

    config
}
