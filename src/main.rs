extern crate futures;
extern crate hyper;
extern crate tokio_core;

use std::io::{self, Write};
use futures::{Future, Stream};
use hyper::Client;
use tokio_core::reactor::Core;

const APP_ID: &str = "56b8dbfc7d0e4425b9d42c53a274e3e2";
const URL_LATEST: &str = "http://openexchangerates.org/api/latest.json";

fn main() {
    let mut core = Core::new().unwrap();
    let client = Client::new(&core.handle());

    let uri = (URL_LATEST.to_owned()+"?app_id="+APP_ID).parse().unwrap();
    let work = client.get(uri).and_then(|res| {
        println!("Response: {}", res.status());

        res.body().for_each(|chunk| {
            io::stdout()
                .write_all(&chunk)
                .map(|_| ())
                .map_err(From::from)
        })
    });
    core.run(work).unwrap();
}
