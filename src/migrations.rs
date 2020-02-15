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
    let migrations = status::migration_status();

    for migration in migrations {
        if migration.ran {
            continue;
        }

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

        // process out discrete queries that need to be ran
        let mut queries_to_execute: Vec<String> = Vec::new();
        let mut temp = String::new();

        let mut delimiter = ';';
        let mut delimiter_check = true;

        for (idx, c) in queries.chars().enumerate() {
            // at the beginning of each line, check for delimiter directive
            if delimiter_check {
                delimiter_check = false;

                let test = &queries[idx..];

                if test.to_lowercase().starts_with("delimiter") {
                    delimiter = queries[idx+10..].chars().next().unwrap();
                }
            }

            if c == '\n' {
                delimiter_check = true;
            }

            // last iteration, bail
            if idx == queries.len() - 1 {
                temp.push(c);
                if !temp.trim().is_empty() {
                    queries_to_execute.push(temp.trim().to_string());
                }
                break;
            }

            let next_char = queries[idx+1..].chars().next().unwrap();

            // check for ending query condition
            if idx < queries.len() - 1 && c == delimiter && next_char == '\n' {
                temp.push(delimiter);
                queries_to_execute.push(temp.trim().to_string());
                temp.clear();
                continue;
            }

            temp.push(c);
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
    }
}
