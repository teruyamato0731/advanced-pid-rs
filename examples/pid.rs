use advanced_pid::{prelude::*, Pid, PidGain};

fn main() {
    let gain = PidGain {
        kp: 1.0,
        ki: 0.1,
        kd: 0.1,
    };
    let mut pid = Pid::new(gain.into());

    println!("{:5.2}", pid.update(1.0, 0.0, 1.0));
    println!("{:5.2}", pid.update(1.0, 0.1, 1.0));
    println!("{:5.2}", pid.update(1.0, 0.3, 1.0));
    println!("{:5.2}", pid.update(1.0, 0.6, 1.0));
    println!("{:5.2}", pid.update(1.0, 0.9, 1.0));
    println!("{:5.2}", pid.update(1.0, 1.2, 1.0));
}
