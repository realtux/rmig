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

use crate::config;
use crate::structs::Migration;

use postgres::{Client, NoTls};
use std::io::{stdout, Write};
use crossterm::execute;
use crossterm::style::{Color, Print, ResetColor, SetForegroundColor};
use std::sync::Mutex;

lazy_static! {
    static ref CONNECTION: Mutex<postgres::Client> = {
        let config = config::load().unwrap();

        let conn_url = format!("host={} user={}", config.host, config.user);

        let mut conn = Client::connect(&conn_url, NoTls).unwrap();

        // backwards compatibility for bmig users
        let _ = conn.query("rename table zzzzzbmigmigrations to rmig", &[]);

        // add the rmig table in case it doesn't exist
        let _ = conn.query("create table if not exists \
            rmig ( \
                name varchar(255) not null, \
                primary key(name) \
            )engine=innodb default charset=utf8", &[]);

        Mutex::new(conn)
    };
}

pub fn query(query: String) {
    let conn = &mut CONNECTION.lock().unwrap();

    let result = conn.query(query.as_str(), &[]);

    match result {
        Err(e) => {
            let _ = execute!(
                stdout(),
                SetForegroundColor(Color::Red),
                Print("error: "),
                ResetColor,
                Print(e.to_string()),
                Print("\n")
            );
        },
        _ => {
            let _ = execute!(
                stdout(),
                SetForegroundColor(Color::Red),
                Print("unknown error"),
                ResetColor,
                Print("\n")
            );
        }
    };
}

pub fn get_migration_list() -> Vec<Migration> {
    let conn = &mut CONNECTION.lock().unwrap();

    let mut migrations: Vec<Migration> = Vec::new();

    for row in conn.query("select * from rmig", &[]).unwrap() {
        migrations.push(Migration {
            name: row.get(0),
            ran: true
        });
    }

    migrations
}
