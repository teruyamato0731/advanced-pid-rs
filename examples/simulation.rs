use advanced_pid::{pid::Pid, PidController, PidGain};

fn main() {
    // Init PID controller
    let gain = PidGain {
        kp: 1.0,
        ki: 0.8,
        kd: 0.1,
    };
    let mut pid = Pid::new(gain.into());

    let target = 1.0;
    let mut actual = 0.0;
    let dt = 0.1;
    loop {
        // Calculate control output
        let output = pid.update(target, actual, dt);

        // Simulate the system response
        actual += (output - actual) / 4.0;
        println!("{:5.2}", actual);

        // Sleep 100ms
        std::thread::sleep(std::time::Duration::from_secs_f32(dt));
    }
}
