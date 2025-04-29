<div align="center">

# advanced-pid-rs

**An advanced PID control library implemented in Rust**

[![Crates.io Version](https://img.shields.io/crates/v/advanced-pid)](https://crates.io/crates/advanced-pid)
[![Documentation at docs.rs](https://img.shields.io/docsrs/advanced-pid)](https://docs.rs/advanced-pid)
[![License: MIT](https://img.shields.io/github/license/teruyamato0731/advanced-pid-rs)](https://github.com/teruyamato0731/advanced-pid-rs/blob/main/LICENSE)
[![CI](https://github.com/teruyamato0731/advanced-pid-rs/actions/workflows/ci.yaml/badge.svg)](https://github.com/teruyamato0731/advanced-pid-rs/actions/workflows/ci.yaml)

[**Crates.io**](https://crates.io/crates/advanced-pid)
| [**API Docs**](https://docs.rs/advanced-pid)
| [**Examples**](https://github.com/teruyamato0731/advanced-pid-rs/tree/main/examples)

</div>

## Highlights
- Supports various types of PID controls
    - Position (standard) PID Control
    - Velocity form PID Control
    - Derivative action based on PV (PI-D)
    - Proportional action based on PV (I-PD)
- Customizable PID gains and limits
- `no_std` support
- User-friendly with the PidController trait
- Includes a simulation example
- Allows switching between `f32` and `f64` floating point types through feature flags

## Installation
To install, run the following Cargo command in your project directory:
```bash
cargo add advanced-pid
```

Or add the following to your Cargo.toml:
```toml
[dependencies]
advanced-pid = "0.2.3"
```

## Quick Start
```bash
cargo run --example simulation
```

## Examples

[Example of Standard PID Control](https://github.com/teruyamato0731/advanced-pid-rs/blob/main/examples/simple.rs)

```rust
use advanced_pid::{prelude::*, Pid, PidGain};

fn main() {
    let gain = PidGain {
        kp: 1.0,
        ki: 0.3,
        kd: 0.1,
    };
    let mut pid = Pid::new(gain.into());

    println!("{:5.2}", pid.update(1.0, 0.0, 1.0));
    println!("{:5.2}", pid.update(1.0, 0.5, 1.0));
    println!("{:5.2}", pid.update(1.0, 0.8, 1.0));
}
```

[Example of Velocity Form PID Control](https://github.com/teruyamato0731/advanced-pid-rs/blob/main/examples/vel_pid.rs)

```rust
use advanced_pid::{prelude::*, PidConfig, VelPid};

fn main() {
    let config = PidConfig::new(1.0, 0.1, 0.1).with_limits(-1.0, 1.0);
    let mut pid = VelPid::new(config);

    let target = 1.0;
    let dt = 1.0;

    println!("{:5.2}", pid.update(target, 0.0, dt));
    println!("{:5.2}", pid.update(target, 0.1, dt));
    println!("{:5.2}", pid.update(target, 0.3, dt));
}
```

## More information
For additional information, please visit:
- [Crates.io](https://crates.io/crates/advanced-pid)
- [API Docs](https://docs.rs/advanced-pid)
- [Other examples](https://github.com/teruyamato0731/advanced-pid-rs/tree/main/examples)

## License
Copyright (c) 2024 Yoshikawa Teru

This project is released under the MIT License, see [LICENSE](https://github.com/teruyamato0731/advanced-pid-rs/blob/main/LICENSE).
