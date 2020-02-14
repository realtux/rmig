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
use std::process;

use chrono::Local;

pub fn handle(args: Vec<String>) {
    // check for missing args for the migration
    if args.len() < 3 {
        println!("you must supply a name for your migration\n");
        process::exit(1);
    }

    let date = Local::now();

    let mut filename: String = date.format("%Y%m%d%H%M%S").to_string();

    filename.push_str("-");

    let mut processed = 0;

    // concatenate each arg with a dash
    for arg in &args[2..] {
        filename.push_str(arg);
        processed += 1;

        if processed + 2 != args.len() {
            filename.push_str("-");
        }
    }

    filename.push_str(".sql");

    // desired output is 00000000000000-arg1-arg2-arg3-arg4.sql
    fs::write(format!("migrations/{}", filename), "up:\n\n\ndown:\n\n")
        .expect("problem creating new migration");
}
