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
extern crate serde_json;
extern crate time;

#[macro_use]
extern crate serde_derive;


use std::io;
use std::collections::BTreeMap;
use futures::{Future, Stream};
use hyper::Client;
use tokio_core::reactor::Core;

const APP_ID: &str = "56b8dbfc7d0e4425b9d42c53a274e3e2";
const URL_LATEST: &str = "http://openexchangerates.org/api/latest.json";


#[derive(Serialize, Deserialize, Debug)]
struct Xchg {
    disclaimer: String,
    license: String,
    timestamp: i64,
    base: String,
    rates: BTreeMap<String, f64>,
}


fn main() {
    let mut core = Core::new().unwrap();
    let client = Client::new(&core.handle());

    let uri = (URL_LATEST.to_owned() + "?app_id=" + APP_ID)
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

    let res = core.run(work).unwrap();

    println!("{:?}", xchg_still_valid(&res));
}

fn xchg_still_valid(stored: &Xchg) -> bool {
    let now = time::get_time().sec;

    if now - stored.timestamp < 3600 {
        true
    } else {
        false
    }
}
