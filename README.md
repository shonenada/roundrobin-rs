# Roundrobin

[![Build Status](https://travis-ci.com/shonenada/roundrobin-rs.svg?branch=master)](https://travis-ci.com/shonenada/roundrobin-rs)
[![Latest version](https://img.shields.io/crates/v/roundrobin.svg)](https://crates.io/crates/roundrobin)
[![License](https://img.shields.io/crates/l/roundrobin.svg)](https://github.com/rust-lang-nursery/lazy-static.rs#license)

A weighted roundrobin implementation in Rustlang.

## Quick Start

[roundrobin-rs](https://crates.io/crates/roundrobin) is available on crates.io.

Add the following dependency to your Cargo.toml:

```
[dependencies]
roundrobin = "0.1.1"
```

## Example

```rust
use roundrobin::wrr::*;

fn main() {
    let url01 = "http://localhost:8081".to_string();
    let url02 = "http://localhost:8082".to_string();
    let server01 = Server::new(url01.clone(), 1);
    let mut rr = WeightedRoundRobinBalancer::new();
    rr.insert_server(server01); // default weight 1
    rr.insert_url(url02.clone(), 2);
    println!("Server: {}", rr.next().unwrap());
}
```

## License

Licensed under MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)
