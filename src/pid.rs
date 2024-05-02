use super::FloatType;
use super::PidConfig;
use super::PidController;

#[derive(Debug, Clone)]
pub struct Pid {
    config: PidConfig,
    i_term: FloatType,
    pre_error: FloatType,
}

impl Default for Pid {
    fn default() -> Self {
        Self::new(PidConfig::default())
    }
}

impl PidController for Pid {
    fn new(config: PidConfig) -> Self {
        Self {
            config,
            i_term: 0.0,
            pre_error: FloatType::NAN,
        }
    }
    fn update(&mut self, set_point: FloatType, actual: FloatType, dt: FloatType) -> FloatType {
        let error = set_point - actual;
        self.i_term += error * dt;
        let d_term = if self.pre_error.is_nan() {
            0.0
        } else {
            (error - self.pre_error) / dt
        };
        let output = self.config.gain.kp * error
            + self.config.gain.ki * self.i_term
            + self.config.gain.kd * d_term;
        self.pre_error = error;
        output.clamp(self.config.min, self.config.max)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pid_controller_p() {
        let gain = crate::PidGain {
            kp: 1.0,
            ki: 0.0,
            kd: 0.0,
        };
        let mut pid = Pid::new(gain.into());

        let output = pid.update(1.0, 0.0, 1.0);
        assert_eq!(output, 1.0);
    }

    #[test]
    fn test_pid_controller_i() {
        let gain = crate::PidGain {
            kp: 0.0,
            ki: 1.0,
            kd: 0.0,
        };
        let mut pid = Pid::new(gain.into());

        let output = pid.update(1.0, 0.0, 1.0);
        assert_eq!(output, 1.0);
        let output = pid.update(1.0, 0.0, 1.0);
        assert_eq!(output, 2.0);
        let output = pid.update(1.0, 0.0, 1.0);
        assert_eq!(output, 3.0);
    }

    #[test]
    fn test_pid_controller_d() {
        let gain = crate::PidGain {
            kp: 0.0,
            ki: 0.0,
            kd: 1.0,
        };
        let mut pid = Pid::new(gain.into());

        let output = pid.update(1.0, 0.0, 1.0);
        assert_eq!(output, 0.0);
        let output = pid.update(1.0, 1.0, 1.0);
        assert_eq!(output, -1.0);
    }

    #[test]
    fn test_pid_controller_limits() {
        let config = PidConfig::new(1.0, 0.0, 0.0).with_limits(-0.5, 0.5);
        let mut pid = Pid::new(config);

        let output = pid.update(1.0, 0.0, 1.0);
        assert_eq!(output, 0.5);
        let output = pid.update(-1.0, 0.0, 1.0);
        assert_eq!(output, -0.5);
    }
}
