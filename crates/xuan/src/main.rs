/**
 * Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */
use std::{env, fs, process};
use front_end::lexer;

fn main() -> std::io::Result<()> {
    let path = env::current_dir()?;
    println!("cwd: {:#?}", path);

    let args: Vec<String> = env::args().collect();
    println!("args: {:#?}", args);

    Ok(())
}
