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

    println!("{:?}",xchg_still_valid(&res));
}

fn xchg_still_valid(stored: &Xchg) -> bool {
    let now = time::get_time().sec;
    //println!("Current: {}\n Stored: {}\n  Delta: {}", now, stored.timestamp, now - stored.timestamp);
    if now - stored.timestamp < 3600 {
        true
    }else{
        false
    }
}
