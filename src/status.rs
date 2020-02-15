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

use std::fs;
use std::io;
use std::io::{stdout, Write};
use crossterm::execute;
use crossterm::style::{Color, Print, ResetColor, SetForegroundColor};

use crate::structs::Migration;
use crate::drivers::interface;

pub fn handle() {
    let local_migrations = migration_status();

    let mut pending_migrations = 0;

    for migration in local_migrations {
        if migration.ran {
            let _ = execute!(
                stdout(),
                SetForegroundColor(Color::Green),
                Print("up - "),
                ResetColor,
                Print(migration.name),
                Print("\n")
            );
        } else {
            pending_migrations += 1;

            let _ = execute!(
                stdout(),
                SetForegroundColor(Color::Red),
                Print("dn - "),
                ResetColor,
                Print(migration.name),
                Print("\n")
            );
        }
    }

    if pending_migrations > 0 {
        println!(
            "you have {} pending migration{}, run `rmig migrate` to apply {}",
            pending_migrations,
            if pending_migrations == 1 { "" } else { "s" },
            if pending_migrations == 1 { "it" } else { "them" }
        );
    }
}

pub fn migration_status() -> Vec<Migration> {
    let migrations = interface::get_migration_list();

    let mut files = fs::read_dir("migrations")
        .unwrap()
        .map(|result| {
            result.map(|file| {
                file.path()
            })
        })
        .collect::<Result<Vec<_>, io::Error>>()
        .unwrap();

    files.sort();

    let mut local_migrations: Vec<Migration> = Vec::new();

    for file in files {
        let name = file.file_name().unwrap().to_str().unwrap();

        if name.starts_with('.') {
            continue;
        }

        let mut ran = false;

        for migration in &migrations {
            if migration.name == name.to_string() {
                ran = true;
            }
        }

        local_migrations.push(Migration {
            name: name.to_string(),
            ran
        });
    }

    local_migrations
}
