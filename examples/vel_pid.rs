use advanced_pid::{vel_pid::VelPid, PidConfig, PidController};

fn main() {
    let config = PidConfig::new(1.0, 0.1, 0.1).with_limits(-1.0, 1.0);
    let mut pid = VelPid::new(config);

    println!("{:5.2}", pid.update(1.0, 0.0, 1.0));
    println!("{:5.2}", pid.update(1.0, 0.1, 1.0));
    println!("{:5.2}", pid.update(1.0, 0.3, 1.0));
}
