extern crate tokio_core;
extern crate ns_env_config;

use std::env;


fn main() {
    let mut core = tokio_core::reactor::Core::new().expect("init loop");
    let ns = ns_env_config::init(&core.handle()).expect("init dns");
    for name in env::args().skip(1) {
        let value = core.run(ns.resolve_auto(&name, 80));
        println!("{} resolves to {:?}", name, value);
    }
}
