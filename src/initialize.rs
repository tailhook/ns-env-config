use std::env;

use failure::Error;
use futures_cpupool::CpuPool;
use tokio_core::reactor::Handle;
use ns_std_threaded::ThreadedResolver;


use Router;
use router;
use config::Config;
use ns::HostResolve;
use router::subscribe_ext::SubscribeExt;
use void::unreachable;

/// Init resolver on he specified main loop by parsing environment
///
/// This is equivalent to: `init_default(h, &Config::default())`
pub fn init(h: &Handle) -> Result<Router, Error> {
    init_default(h, &Config::default())
}

/// Init resolver by parsing environment while overriding default
///
/// If no environment variable is set we use `config` as default.
pub fn init_default(h: &Handle, config: &Config) -> Result<Router, Error> {
    if let Ok(val) = env::var("RUST_NS") {
        if val.len() > 0 {
            if let Ok(cfg) = val.parse() {
                return force_config(h, &cfg)
            }
        }
    }
    force_config(h, config)
}

/// Init resolver with specified config, ignoring environment
pub fn force_config(h: &Handle, config: &Config) -> Result<Router, Error> {
    use config::Route;
    let mut cfg = router::Config::new();
    match config.fallthrough {
        Route::Std(ref std) => {
            let pool = CpuPool::new(std.threads as usize);
            cfg.set_fallthrough(ThreadedResolver::use_pool(pool)
                .null_service_resolver()
                .interval_subscriber(std.poll_ivl, &h));
        }
        Route::__Nonexhaustive(void) => unreachable(void),
    }
    let ns = Router::from_config(&cfg.done(), &h);
    Ok(ns)
}
