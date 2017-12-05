NS Env Config
=============

[API Docs](https://docs.rs/ns-env-config) |
[Usage](https://github.com/tailhook/ns-env-config#usage) |
[Github](https://github.com/tailhook/ns-env-config) |
[Crate](https://crates.io/crates/ns-env-config)

A env_logger-inspired one-stop configuration for name resolution based
on abstract-ns.

**Goals:**

1. Make bootstrapping of small to medium size apps easier
2. Provide a standard way for configuring name resolution in rust ecosystem

Features:

1. Standard libc-based name resolution by default (using threads)
2. Feature-gated other resolvers (async, consul,...)
3. If compiled in, resolvers are enabled by `RUST_NS=resolver_name`
4. Sane defaults for resolver
5. Subscriptions are enabled by default (using polling if not available)
6. Some options configured from env (`RUST_NS=resolver_name:option=value`)
7. [TODO] Suffix-based resolution (`RUST_NS=std,consul=consul-resolver`)

Non-goals:

1. Covering every other way to resolve names
2. Covering all the settings of every resolvers


Usage
=====

Standard Resolver
-----------------

```shell
$ RUST_NS=std ./your-app
```

Since standard library resolver is not asynchronous we run in a thread pool.

Parameters:

`poll_ivl` -- polling interval for subscriptions in seconds, default `1`
`threads` -- number of threads for name resolution, default `4`

Biggest declaration is something like this:

```shell
$ RUST_NS=std:poll_ivl=10:threads=16 ./your-app
```


License
=======

Licensed under either of

* Apache License, Version 2.0,
  (./LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license (./LICENSE-MIT or http://opensource.org/licenses/MIT)
  at your option.

Contribution
------------

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.

