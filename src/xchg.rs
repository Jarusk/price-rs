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
extern crate futures;
extern crate hyper;
extern crate tokio_core;
extern crate time;
extern crate serde_json;

use self::futures::{Future, Stream};
use self::hyper::Client;
use self::tokio_core::reactor::Core;

use std::collections::BTreeMap;
use std::io;
use std::env;
use std::path::Path;
use std::fs::File;

use constants;
use config::Config;


#[derive(Serialize, Deserialize, Debug)]
pub struct Xchg {
    disclaimer: String,
    license: String,
    timestamp: i64,
    base: String,
    rates: BTreeMap<String, f64>,
}

impl Xchg {
    pub fn still_valid(&self) -> bool {
        let now = time::get_time().sec;
        return now - self.timestamp < 3600;
    }

    pub fn store(&self, p: &Path) -> Result<(), String> {
        let file = match File::create(p) {
            Err(e) => {
                println!("Warning: {:?}", e);
                return Err(e.to_string());
            }
            Ok(f) => f,
        };

        match serde_json::to_writer(file, &self) {
            Err(e) => Err(e.to_string()),
            Ok(_) => Ok(()),
        }
    }

    pub fn load_or_create(conf: &Config) -> Xchg {
        let mut tmp_env = "/tmp".to_string();

        match env::var("TMP") {
            Ok(v) => tmp_env = v,
            Err(_) => {}
        };

        let xchg_json_path = Path::new(&tmp_env).join(constants::XCHG_NAME);

        let file = match File::open(&xchg_json_path) {
            Err(_) => File::create(&xchg_json_path).unwrap(),
            Ok(f) => f,
        };

        let mut refreshed_rates = false;

        let mut rates: Xchg = match serde_json::from_reader(file) {
            Ok(x) => x,
            _ => {
                refreshed_rates = true;
                pull_rates(conf)
            }
        };

        if !rates.still_valid() {
            refreshed_rates = true;
            rates = pull_rates(conf);
        }

        // This line will force the file to be closed
        // above by killing the last reference to it
        let file = 0;

        if refreshed_rates {
            debug_assert!({
                println!("HAD TO PULL", );
                true
            });
            match rates.store(&xchg_json_path) {
                Err(e) => println!("Warning: Could not cache rates file: {:?}", e),
                Ok(_) => {}
            };
        } else {
            debug_assert!({
                println!("USED CACHE", );
                true
            });
        }

        return rates;
    }
}

fn pull_rates(conf: &Config) -> Xchg {
    debug_assert!({
        println!("PULLING");
        true
    });
    let mut core = Core::new().unwrap();
    let client = Client::new(&core.handle());

    let uri = (constants::URL_LATEST.to_owned() + "?app_id=" + conf.get_app_id())
        .parse()
        .unwrap();

    let work = client.get(uri).and_then(|res| {
        res.body().concat2().and_then(|body| {
            let v: Xchg = serde_json::from_slice(&body)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
                .unwrap();

            Ok(v)
        })
    });

    match core.run(work) {
        Err(e) => panic!("Could not fetch Rates file: {}", e),
        Ok(x) => x,
    }
}
