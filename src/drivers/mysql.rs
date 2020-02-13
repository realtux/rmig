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

use crate::structs::Config;
use crate::structs::Migration;

pub fn query(config: Config, query: String) -> Vec<Migration> {
    let conn_url = format!(
        "mysql://{}:{}@{}:{}/{}",
        config.user, config.pass,
        config.host, config.port,
        config.db
    );

    let pool = mysql::Pool::new(conn_url).unwrap();

    let migrations: Vec<Migration> =
        pool.prep_exec(query, ())
            .map(|result| {
                result
                    .map(|x| x.unwrap())
                    .map(|row| {
                        let name = mysql::from_row(row);

                        Migration { name }
                    })
                    .collect()
            })
            .unwrap();

    migrations
}
