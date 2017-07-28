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
use constants;
use config::Config;

//const APP_ID: &str = "56b8dbfc7d0e4425b9d42c53a274e3e2";

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

    pub fn load_or_create(conf: &Config) -> Xchg {
        fetch_rates(conf)
    }
}

fn fetch_rates(conf: &Config) -> Xchg {
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
