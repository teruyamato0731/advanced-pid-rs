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
