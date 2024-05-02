use advanced_pid::{prelude::*, Pid, PidConfig};

fn main() {
    let config = PidConfig::new(1.0, 0.1, 0.1).with_limits(-1.0, 1.0);
    let mut pid = Pid::new(config);

    println!("{:5.2}", pid.update(1.0, 0.0, 1.0));
    println!("{:5.2}", pid.update(1.0, 0.1, 1.0));
    println!("{:5.2}", pid.update(1.0, 0.3, 1.0));
    println!("{:5.2}", pid.update(1.0, 0.6, 1.0));
    println!("{:5.2}", pid.update(1.0, 0.9, 1.0));
    println!("{:5.2}", pid.update(1.0, 1.2, 1.0));
}
