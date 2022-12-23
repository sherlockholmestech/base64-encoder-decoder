/*    Encodes and decodes Base64.
    Copyright (C) 2022 Sherlock Holmes

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.*/
use std::{env, process::exit};

use dialoguer::{theme::ColorfulTheme, Select, Editor};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut input = String::new();
    let mut output = String::new();
    let modes = &["encode", "decode"];
    // checks for input
    if None == args.get(1) {
        if let Some(rv) = Editor::new().edit("Enter the text that you want to encode/decode").unwrap() {
            input = rv;
        } else {
            println!("Operation failed.");
        }
    } else {
        input =args.get(1).unwrap().to_string();
    }
    if input == "" {
        println!("No text was provided!");
        exit(2);
    }
    //chose mode
    let select_mode = Select::with_theme(&ColorfulTheme::default()).with_prompt("Please select your mode").default(0).items(modes).interact().unwrap();
    match modes[select_mode] {
        "encode" => {
            output = base64::encode(input);
        },
        "decode" => {
            let output_vec = base64::decode(input).unwrap();
            for i in output_vec.iter() {
                output.push(*i as char);
            }
        },
        _ => panic!("How the hell did we arive here"),
    }

    println!("The {}d text is {}", modes[select_mode], output);
}
