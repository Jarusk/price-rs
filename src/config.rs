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
}

fn get_app_id() -> Config {
    let mut message = String::new();
    message += "Performing initial config.\n";
    message += "This utility relies upon the use of OpenExchangeRates.org.\n";
    message += "Please register there and return once you have an App ID\n\n";
    message += "Please input your APP ID:";

    let mut app_id = String::new();
    println!("{}", &message);
    match io::stdin().read_line(&mut app_id) {
        Ok(_) => println!("Successfully read APP ID"),
        Err(e) => panic!("Error reading: {}", e),
    };

    Config {
        app_id: app_id.trim().to_owned(),
        base_currency: constants::DEFAULT_BASE_CURRENCY.to_owned(),
        target_currency: constants::DEFAULT_TARGET_CURRENCY.to_owned(),
        tax_rate: constants::DEFAULT_TAX_RATE,
        apply_tax: constants::APPLY_TAX,
    }
}
