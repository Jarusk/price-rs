/*
 * Copyright (C) 2017  Yaraskavitch, Matt <yaraskavitch.matt@gmail.com>
 * Author: Yaraskavitch, Matt <yaraskavitch.matt@gmail.com>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

extern crate serde_json;

use constants;
use std::env;
use std::fs::File;
use std::io;
use std::str::FromStr;


#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    app_id: String,
    base_currency: String,
    target_currency: String,
    apply_tax: bool,
    tax_rate: f32,
}

impl Config {
    pub fn load_or_create() -> Config {
        let dir = match env::home_dir() {
            Some(val) => val,
            None => env::current_dir().unwrap(),
        };

        let conf_file = dir.join(constants::CONFIG_NAME);

        let file = match File::open(&conf_file) {
            Err(_) => File::create(&conf_file).unwrap(),
            Ok(f) => f,
        };

        let mut write_conf = false;

        let conf: Config = match serde_json::from_reader(file) {
            Ok(c) => c,
            _ => {
                write_conf = true;
                get_app_id()
            }
        };

        if write_conf {
            let file = match File::create(conf_file) {
                Ok(f) => f,
                Err(e) => panic!("Could not write conf file: {}", e),
            };
            match serde_json::to_writer(file, &conf) {
                Err(e) => panic!("Could not write conf file: {}", e),
                _ => {}
            }
        }

        return conf;
    }

    pub fn get_app_id(&self) -> &str {
        self.app_id.as_ref()
    }

    pub fn read_args(&mut self) {
        unimplemented!();
    }
}

fn get_app_id() -> Config {
    let mut message = String::new();
    message += "Performing initial config.\n";
    message += "This utility relies upon the use of OpenExchangeRates.org.\n";
    message += "Please register there and return once you have an App ID\n\n";
    message += "\nPlease input your APP ID:";

    let mut app_id = String::new();
    println!("{}", &message);
    match io::stdin().read_line(&mut app_id) {
        Ok(_) => println!("Successfully read APP ID"),
        Err(e) => panic!("Error reading: {}", e),
    };

    let mut base = String::new();
    println!("\nPlease enter the default base currency [USD]:");
    match io::stdin().read_line(&mut base) {
        Ok(_) => {}
        Err(e) => panic!("Error reading: {}", e),
    };
    base = base.trim().to_string();
    if base == "" {
        base = constants::DEFAULT_BASE_CURRENCY.to_string();
    }

    let mut target = String::new();
    println!("\nPlease enter the default target currency [CAD]:");
    match io::stdin().read_line(&mut target) {
        Ok(_) => {}
        Err(e) => panic!("Error reading: {}", e),
    };
    target = target.trim().to_string();
    if target == "" {
        target = constants::DEFAULT_TARGET_CURRENCY.to_string();
    }

    let mut tax_rate = String::new();
    println!("\nPlease enter the default tax rate.\nFor example, if its 13%, enter 1.13 [1.13]:");
    match io::stdin().read_line(&mut tax_rate) {
        Ok(_) => {}
        Err(e) => panic!("Error reading: {}", e),
    };
    let tax_rate = match f32::from_str(&tax_rate) {
        Ok(f) => f,
        Err(_) => constants::DEFAULT_TAX_RATE,
    };

    let mut enable_tax_buffer = String::new();
    let mut enable_tax = constants::APPLY_TAX;

    'valid: loop {
        println!("\nShould tax be applied by default? [Yn]:");
        match io::stdin().read_line(&mut enable_tax_buffer) {
            Ok(_) => {}
            Err(e) => panic!("Error reading: {}", e),
        };
        enable_tax_buffer = enable_tax_buffer.trim().to_string();
        match enable_tax_buffer.as_ref() {
            "N" | "n" => {
                enable_tax = false;
                break 'valid;
            }
            "Y" | "y" | "" => {
                enable_tax = true;
                break 'valid;
            }
            _ => {}
        }
    }

    Config {
        app_id: app_id.trim().to_owned(),
        base_currency: base,
        target_currency: target,
        tax_rate: tax_rate,
        apply_tax: enable_tax,
    }
}
