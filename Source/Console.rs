// Copyright (c) TribuFu 2015-2020. All Rights Reserved

#![allow(non_snake_case)]

use std::io::{self, Read};

fn main() {
    println!("Hello World");

    loop {
        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(n) => {
                print!("{}", input);
            }
            Err(error) => panic!("{}", error),
        }
    }
}
