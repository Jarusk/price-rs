extern crate futures;
extern crate hyper;
extern crate tokio_core;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use serde_json::Error;

use std::io::{self, Write};
use std::collections::BTreeMap;
use futures::{Future, Stream};
use hyper::Client;
use tokio_core::reactor::Core;
use serde_json::Value;

const APP_ID: &str = "56b8dbfc7d0e4425b9d42c53a274e3e2";
const URL_LATEST: &str = "http://openexchangerates.org/api/latest.json";

#[derive(Serialize, Deserialize, Debug)]
struct Xchg<'a> {
    disclaimer: &'a str,
    license: &'a str,
    timestamp: usize,
    base: &'a str,
    rates: BTreeMap<&'a str, f64>,
}

fn main() {
    let mut core = Core::new().unwrap();
    let client = Client::new(&core.handle());

    let uri = (URL_LATEST.to_owned() + "?app_id=" + APP_ID)
        .parse()
        .unwrap();
    let work = client.get(uri).and_then(|res| {
        println!("Response: {}", res.status());

        res.body().concat2().and_then(|body| {
            let v: Xchg = serde_json::from_slice(&body).map_err(|e| {
                io::Error::new(io::ErrorKind::Other, e)
            })?;
            Ok(v)
        })
    });

    let res = core.run(work).unwrap();
    println!("{:?}", res);
}
