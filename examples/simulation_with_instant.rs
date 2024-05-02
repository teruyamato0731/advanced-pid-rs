use advanced_pid::{prelude::*, PidConfig, VelPid};

use std::time::Instant;

fn main() {
    let mut pid = VelPid::default();
    let config = PidConfig::new(0.8, 0.3, 0.2).with_limits(-1.2, 1.2);
    pid.reset_config(config);

    let target = 1.0;
    let mut actual = 0.0;

    let mut pre = Instant::now();
    loop {
        let now = Instant::now();
        let dt = now - pre;

        if dt > std::time::Duration::from_secs(1) {
            let output = pid.update(target, actual, dt.as_secs_f32());
            actual += (output - actual) / 8.0;
            println!("{:5.2}\t{:5.2}\t{:?}", actual, output, dt);
            pre = now;
        }
    }
}
