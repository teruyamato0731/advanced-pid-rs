# advanced-pid-rs

[![Crates.io Version](https://img.shields.io/crates/v/advanced-pid)](https://crates.io/crates/advanced-pid)
[![Documentation at docs.rs](https://img.shields.io/docsrs/advanced-pid)](https://docs.rs/advanced-pid)
[![License: MIT](https://img.shields.io/github/license/teruyamato0731/advanced-pid-rs)](https://github.com/teruyamato0731/advanced-pid-rs/blob/main/LICENSE)

## Highlights
- Advanced PID control library available in Rust
- Supports many PID controls
    - Position (normal) PID Control
    - Velocity form PID Control
    - Basing derivative action on PV (PI-D)
    - Basing proportional action on PV (I-PD)

## Quick Start
```bash
cargo run --example simulation
```

## Examples
```rust:examples/simple.rs
use advanced_pid::{pid::Pid, PidController, PidGain};

fn main() {
    let gain = PidGain {
        kp: 1.0,
        ki: 0.1,
        kd: 0.1,
    };
    let mut pid = Pid::new(gain.into());

    println!("{:5.2}", pid.update(1.0, 0.0, 1.0));
    println!("{:5.2}", pid.update(1.0, 0.5, 1.0));
    println!("{:5.2}", pid.update(1.0, 0.8, 1.0));
}
```

```rust:examples/vel_pid.rs
use advanced_pid::{vel_pid::VelPid, PidConfig, PidController};

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
For more information, check out:
- [crates.io](https://crates.io/crates/advanced-pid)
- [docs.rs](https://docs.rs/advanced-pid)
- [Other examples](https://github.com/teruyamato0731/advanced-pid-rs/tree/main/examples)

## License
Copyright (c) 2024 Yoshikawa Teru

This project is released under the MIT License, see [LICENSE](https://github.com/teruyamato0731/advanced-pid-rs/blob/main/LICENSE).
