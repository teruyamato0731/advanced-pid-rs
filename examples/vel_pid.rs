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
