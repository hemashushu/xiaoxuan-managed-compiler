use std::{env, fs, process};

/**
 * Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */
use front_end::lexer;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!(
            "\
usage:

$ cargo run --bin lexer path_to_script_file
e.g.
$ cargo run --bin lexer scripts/01-base-expression.xuan"
        );
        process::exit(1);
    }

    let file_path = &args[1];
    let program = fs::read_to_string(file_path).expect("read file error");
    let result = lexer::tokenize(&program).expect("tokenize error");

    for token in result {
        println!("{:?}", token);
    }
}
