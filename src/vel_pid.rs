//! The `vel_pid` module provides a velocity form PID controller.
//!
//! `VelPid` is a structure that implements the [`PidController`] trait, which provides methods for creating a new controller and updating the controller.
//!
//! # Examples
//!
//! ```rust
//! use advanced_pid::{prelude::*, PidConfig, VelPid};
//!
//! let config = PidConfig::new(1.0, 0.3, 0.1).with_limits(-1.0, 1.0);
//! let mut pid = VelPid::new(config);
//!
//! let target = 1.0;
//! let actual = 0.0;
//! let dt = 1.0;
//!
//! println!("{}", pid.update(target, actual, dt));
//! ```
use super::FloatType;
use super::PidConfig;
use super::PidController;

/// `VelPid` is a structure that implements the [`PidController`] trait.
#[derive(Debug, Clone)]
pub struct VelPid {
    config: PidConfig,
    output: FloatType,
    pre_error: FloatType,
    pre_p_term: FloatType,
    d_term_lpf: FloatType,
}

impl Default for VelPid {
    /// Creates a new `VelPid` with the default configuration.
    fn default() -> Self {
        Self::new(PidConfig::default())
    }
}

impl PidController for VelPid {
    /// Creates a new `VelPid` with the specified configuration.
    fn new(config: PidConfig) -> Self {
        Self {
            config,
            output: 0.0,
            pre_error: 0.0,
            pre_p_term: FloatType::NAN,
            d_term_lpf: 0.0,
        }
    }

    /// Updates the `VelPid` controller with the specified set point, actual value, and time delta.
    /// Returns the controller output.
    fn update(&mut self, set_point: FloatType, actual: FloatType, dt: FloatType) -> FloatType {
        debug_assert!(dt > 0.0, "dt must be positive");
        let error = set_point - actual;
        let p_term = (error - self.pre_error) / dt;
        let d_term = if self.pre_p_term.is_nan() {
            0.0
        } else {
            (p_term - self.pre_p_term) / dt
        };
        self.d_term_lpf += (d_term - self.d_term_lpf) / 8.0;
        let du = self.config.gain.kp * p_term
            + self.config.gain.ki * error
            + self.config.gain.kd * self.d_term_lpf;
        self.pre_error = error;
        self.pre_p_term = p_term;
        self.output = (self.output + du * dt).clamp(self.config.min, self.config.max);
        self.output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vel_pid_p() {
        let gain = crate::PidGain {
            kp: 1.0,
            ki: 0.0,
            kd: 0.0,
        };
        let mut pid = VelPid::new(gain.into());

        let output = pid.update(1.0, 0.0, 1.0);
        assert_eq!(output, 1.0);
    }

    #[test]
    fn test_vel_pid_i() {
        let gain = crate::PidGain {
            kp: 0.0,
            ki: 1.0,
            kd: 0.0,
        };
        let mut pid = VelPid::new(gain.into());

        let output = pid.update(1.0, 0.0, 1.0);
        assert_eq!(output, 1.0);
        let output = pid.update(1.0, 0.0, 1.0);
        assert_eq!(output, 2.0);
        let output = pid.update(1.0, 0.0, 1.0);
        assert_eq!(output, 3.0);
    }

    #[test]
    fn test_vel_pid_d() {
        let gain = crate::PidGain {
            kp: 0.0,
            ki: 0.0,
            kd: 1.0,
        };
        let mut pid = VelPid::new(gain.into());

        let output = pid.update(1.0, 0.0, 1.0);
        assert_eq!(output, 0.0);
        let output = pid.update(1.0, 5.0, 1.0);
        assert!(output < 0.0, "d_term: {} must be lesser than 0.0", output);
    }

    #[test]
    fn test_vel_pid_limits() {
        let config = PidConfig::new(1.0, 0.0, 0.0).with_limits(-0.5, 0.5);
        let mut pid = VelPid::new(config);

        let output = pid.update(1.0, 0.0, 1.0);
        assert_eq!(output, 0.5);
        let output = pid.update(-1.0, 0.0, 1.0);
        assert_eq!(output, -0.5);
    }
}
