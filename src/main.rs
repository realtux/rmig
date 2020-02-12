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

#[macro_use]
extern crate mysql;
extern crate serde;
extern crate serde_json;

use std::env;

mod config;
mod drivers;
mod init;
mod status;
mod structs;

fn main() {
    println!("rmig 0.0.1 by tux");

    //let migrations = drivers::mysql::do_query("select * from zzzzzbmigmigrations".to_string());

    //for migration in migrations {
    //    println!("name: {}", migration.name);
    //}

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        menu();
        return;
    }

    let command: &str = &args[1];

    let mut force = false;
    let mut transaction = false;
    let mut bail = false;

    for arg in &args {
        match command {
            "init" => match arg.as_str() {
                "-f" => force = true,
                _ => {}
            },
            "migrate" => match arg.as_str() {
                "-t" => transaction = true,
                "-b" => bail = true,
                _ => {}
            },
            _ => {}
        }
    }

    let flags = structs::Flags {
        force,
        transaction,
        bail
    };

    match command {
        "init" => init::handle(flags),
        "status" => status::handle(flags),
        _ => menu()
    }
}

fn menu() {
    println!("usage: rmig command");
    println!("    init");
    println!("        create the initial rmig structure and config");
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
