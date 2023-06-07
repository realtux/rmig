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
extern crate lazy_static;
extern crate chrono;
extern crate mysql;
extern crate serde;
extern crate serde_json;
extern crate crossterm;

use std::env;

mod config;
mod create;
mod drivers;
mod enums;
mod init;
mod migrations;
mod status;
mod structs;

fn main() {
    println!("rmig 0.0.4 by tux");

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        menu();
        return;
    }

    let command: &str = &args[1];

    let mut force = false;
    let mut transaction = false;
    let mut all = false;

    for arg in &args {
        match command {
            "init" => match arg.as_str() {
                "-f" => force = true,
                _ => {}
            },
            "migrate" => match arg.as_str() {
                "-t" => transaction = true,
                _ => {}
            },
            "rollback" => match arg.as_str() {
                "-a" => all = true,
                _ => {}
            },
            _ => {}
        }
    }

    let flags = structs::Flags {
        force,
        transaction,
        all
    };

    match command {
        "init" => init::handle(flags),
        "status" => status::handle(),
        "create" => create::handle(args),
        "migrate" => migrations::handle(flags, enums::MigrationOperation::Migrate),
        "rollback" => migrations::handle(flags, enums::MigrationOperation::Rollback),
        _ => menu()
    };

    println!("");
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
