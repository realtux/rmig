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

use std::str;
use std::fs::File;
use std::io::Read;
use std::io::{stdout, Write};
use crossterm::execute;
use crossterm::style::{Color, Print, ResetColor, SetForegroundColor};

use crate::status;
use crate::enums::MigrationOperation;
use crate::structs::Flags;
use crate::drivers::interface;

pub fn handle(flags: Flags, op: MigrationOperation) {
    let mut migrations = status::migration_status();

    let mut migrations_ran = 0;

    // for rollback, reverse the order
    if let MigrationOperation::Rollback = op {
        migrations.reverse();
    }

    for migration in migrations {
        if let MigrationOperation::Migrate = op {
            if migration.ran {
                continue;
            }
        }

        if let MigrationOperation::Rollback = op {
            if !migration.ran {
                continue;
            }
        }

        migrations_ran += 1;

        // handles -f flag
        if flags.transaction {
            interface::query("start transaction".to_string());
        }

        let op_word = match op {
            MigrationOperation::Migrate => "applying: ",
            MigrationOperation::Rollback => "removing: ",
        };

        let _ = execute!(
            stdout(),
            SetForegroundColor(Color::Yellow),
            Print(op_word),
            ResetColor,
            Print(&migration.name),
            Print("\n")
        );

        let mut handle = File::open(format!("migrations/{}", &migration.name))
            .expect("problem reading migration file");

        let mut contents = String::new();

        handle.read_to_string(&mut contents)
            .expect("problem reading migration file");

        let lines: Vec<&str> = contents.split('\n').collect();

        // remove all lines not related to the up
        let mut found = false;
        let mut queries = String::new();

        for line in lines {
            if let MigrationOperation::Migrate = op {
                if line.starts_with("up:") {
                    found = true;
                    continue;
                }

                if line.starts_with("down:") {
                    break;
                }
            }

            if let MigrationOperation::Rollback = op {
                if line.starts_with("down:") {
                    found = true;
                    continue;
                }
            }

            if !found {
                continue;
            }

            queries.push_str(line);
            queries.push('\n');
        }

        // this will help find query boundaries in case the user doesn't use
        // a newline on every line
        queries.push('\n');

        let queries = queries.as_bytes();

        // process out discrete queries that need to be ran
        let mut queries_to_execute: Vec<String> = Vec::new();
        let mut temp: Vec<u8> = Vec::new();

        let mut delimiter = b';';
        let mut delimiter_check = true;

        let mut idx = 0;
        while idx < queries.len() {
            // at the beginning of each line, check for delimiter directive
            if delimiter_check {
                delimiter_check = false;

                let test = str::from_utf8(&queries[idx..]).unwrap();

                if test.to_lowercase().starts_with("delimiter ") {
                    delimiter = queries[idx+10];

                    // pushed past this line
                    idx += 11;
                    continue;
                }
            }

            if queries[idx] == b'\n' {
                delimiter_check = true;
            }

            // last iteration, bail
            if idx == queries.len() - 1 {
                temp.push(queries[idx]);

                let temp_string = str::from_utf8(&temp).unwrap();

                if !temp_string.trim().is_empty() {
                    queries_to_execute.push(temp_string.trim().to_string());
                }

                break;
            }

            let next_char = queries[idx+1];

            // check for ending query condition
            if idx < queries.len() - 1 && queries[idx] == delimiter && next_char == b'\n' {
                temp.push(b';');

                let temp_string = str::from_utf8(&temp).unwrap();

                queries_to_execute.push(temp_string.trim().to_string());

                temp.clear();
                idx += 1;
                continue;
            }

            temp.push(queries[idx]);

            idx += 1;
        }

        for query in queries_to_execute {
            interface::query(query);
        }

        let _ = execute!(
            stdout(),
            SetForegroundColor(Color::Green),
            Print("finished: "),
            ResetColor,
            Print(&migration.name),
            Print("\n")
        );

        // handles -f flag
        if flags.transaction {
            interface::query("commit".to_string());
        }

        match op {
            MigrationOperation::Migrate =>
                interface::add_migration(migration.name),
            MigrationOperation::Rollback =>
                interface::remove_migration(migration.name),
        };

        // for rollback, at the moment, stop after the first one
        if let MigrationOperation::Rollback = op {
            break;
        }
    }

    if migrations_ran == 0 {
        match op {
            MigrationOperation::Migrate =>
                println!("all migrations are already applied"),
            MigrationOperation::Rollback =>
                println!("all migrations are already removed"),
        };
    }
}
