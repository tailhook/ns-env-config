//! Structures which describe configuration
//!
//! You don't need any of these unless you want to override configuration or
//! set a default config.
use std::default::Default;
use std::str::FromStr;
use std::time::Duration;

use void::Void;

/// Configuration for resolver
///
/// This is usually parsed from `RUST_NS` env variable, but can also be
/// constructed manually for passing default or overriding environment.
///
/// Note: Config has infallible `FromStr`, we will publish a better opt-in
/// interface for checking errors.
#[derive(PartialEq, Debug)]
pub struct Config {
    /// A fallthrough/default/root resolver
    pub fallthrough: Route,
    _non_exhaustive: (),
}

/// This is a resolver for a single suffix
#[derive(PartialEq, Debug)]
pub enum Route {
    /// A standard library (libc) -based resolver
    Std(Std),
    #[doc(hidden)]
    __Nonexhaustive(Void),
}

/// Configuration for `std` (or libc-based) resolver
#[derive(PartialEq, Debug)]
pub struct Std {
    /// Polling interval for subscriptions
    ///
    /// Default: 1 second
    ///
    /// There is no real subscriptions in libc-based resolver, so we poll
    /// on interval to see updates.
    pub poll_ivl: Duration,
    /// Number of threads to use
    ///
    /// Default: 4
    ///
    /// Since standard library resolver is not asynchronous we run it in
    /// this number of threads.
    pub threads: u32,
    _non_exhaustive: (),
}

impl Default for Std {
    fn default() -> Std {
        Std {
            poll_ivl: Duration::new(1, 0),
            threads: 4,
            _non_exhaustive: (),
        }
    }
}

impl Default for Route {
    fn default() -> Route {
        Route::Std(Std::default())
    }
}

impl Default for Config {
    fn default() -> Config {
        Config {
            fallthrough: Route::Std(Std::default()),
            _non_exhaustive: (),
        }
    }
}

impl Std {
    fn parse_options(input: &str) -> Std {
        let mut r = Std::default();
        for pair in input.split(":") {
            let mut key_val = pair.splitn(2, "=");
            match (key_val.next().unwrap(), key_val.next()) {
                ("threads", Some(nthreads)) => match nthreads.parse() {
                    Ok(val) => r.threads = val,
                    Err(_) => {},
                }
                ("poll_ivl", Some(ivl)) => match ivl.parse() {
                    Ok(val) => r.poll_ivl = Duration::new(val, 0),
                    Err(_) => {},
                }
                _ => {} // ignore unknown options
            }
        }
        return r;
    }
}

impl FromStr for Config {
    type Err = Void;
    fn from_str(val: &str) -> Result<Self, Void> {
        if val.len() == 0 {
            return Ok(Config::default());
        }
        let mut root = Route::default();
        for route in val.split(',') {
            let mut route_params = route.splitn(2, ':');
            let mut rr = route_params.next().unwrap().splitn(2, "=");
            match (rr.next().unwrap(), rr.next()) {
                (_route, Some(_resolver)) => {
                    // unsupported yet
                    continue;
                }
                (resolver, None) => {
                    match resolver {
                        "std" => {
                            root = Route::Std(Std::parse_options(
                                route_params.next().unwrap_or("")));
                        }
                        // ignore unknown
                        _ => {}
                    }
                }
            }
        }
        Ok(Config {
            fallthrough: root,
            _non_exhaustive: (),
        })
    }
}

#[cfg(test)]
mod test {
    use std::default::Default;
    use std::time::Duration;
    use super::{Config, Route, Std};

    #[test]
    fn test_default_std() {
        assert_eq!(Config::default(), "".parse().unwrap());
        assert_eq!(Config::default(), "std".parse().unwrap());
        assert_eq!(Config {
            fallthrough: Route::Std(Std {
                poll_ivl: Duration::new(2, 0),
                .. Std::default()
            }),
            .. Config::default()
        }, "std:poll_ivl=2".parse().unwrap());
        assert_eq!(Config {
            fallthrough: Route::Std(Std {
                threads: 7,
                .. Std::default()
            }),
            .. Config::default()
        }, "std:threads=7".parse().unwrap());
        assert_eq!(Config {
            fallthrough: Route::Std(Std {
                threads: 1,
                poll_ivl: Duration::new(7, 0),
                .. Std::default()
            }),
            .. Config::default()
        }, "std:threads=1:poll_ivl=7".parse().unwrap());
    }
}
