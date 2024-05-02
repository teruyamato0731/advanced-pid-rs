//! The `i_pd` module provides a PID controller where the proportional action is based on the process variable (PV).
//!
//! `Ipd` is a structure that implements the [`PidController`] trait, which provides methods for creating a new controller and updating the controller.
//!
//! # Examples
//!
//! ```rust
//! use advanced_pid::{i_pd::Ipd, PidConfig, PidController};
//!
//! let config = PidConfig::new(1.0, 0.1, 0.1).with_limits(-1.0, 1.0);
//! let mut pid = Ipd::new(config);
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

/// `Ipd` is a structure that implements the [`PidController`] trait.
#[derive(Debug, Clone)]
pub struct Ipd {
    config: PidConfig,
    i_term: FloatType,
    pre_actual: FloatType,
}

impl Default for Ipd {
    /// Creates a new `Ipd` with the default configuration.
    fn default() -> Self {
        Self::new(PidConfig::default())
    }
}

impl PidController for Ipd {
    /// Creates a new `Ipd` with the specified configuration.
    fn new(config: PidConfig) -> Self {
        Self {
            config,
            i_term: 0.0,
            pre_actual: FloatType::NAN,
        }
    }

    /// Updates the `Ipd` controller with the specified set point, actual value, and time delta.
    /// Returns the controller output.
    fn update(&mut self, set_point: FloatType, actual: FloatType, dt: FloatType) -> FloatType {
        let error = set_point - actual;
        self.i_term += error * dt;
        let d_term = if self.pre_actual.is_nan() {
            0.0
        } else {
            (actual - self.pre_actual) / dt
        };
        let output = self.config.gain.ki * self.i_term
            - self.config.gain.kp * actual
            - self.config.gain.kd * d_term;
        self.pre_actual = actual;
        output.clamp(self.config.min, self.config.max)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_i_pd_controller_p() {
        let gain = crate::PidGain {
            kp: 1.0,
            ki: 0.0,
            kd: 0.0,
        };
        let mut pid = Ipd::new(gain.into());

        let output = pid.update(1.0, 0.0, 1.0);
        assert_eq!(output, 0.0);
        let output = pid.update(1.0, 1.0, 1.0);
        assert_eq!(output, -1.0);
    }

    #[test]
    fn test_i_pd_controller_i() {
        let gain = crate::PidGain {
            kp: 0.0,
            ki: 1.0,
            kd: 0.0,
        };
        let mut pid = Ipd::new(gain.into());

        let output = pid.update(1.0, 0.0, 1.0);
        assert_eq!(output, 1.0);
        let output = pid.update(1.0, 0.0, 1.0);
        assert_eq!(output, 2.0);
        let output = pid.update(1.0, 0.0, 1.0);
        assert_eq!(output, 3.0);
    }

    #[test]
    fn test_i_pd_controller_d() {
        let gain = crate::PidGain {
            kp: 0.0,
            ki: 0.0,
            kd: 1.0,
        };
        let mut pid = Ipd::new(gain.into());

        let output = pid.update(0.0, 0.0, 1.0);
        assert_eq!(output, 0.0);
        let output = pid.update(1.0, 0.0, 1.0);
        assert_eq!(output, 0.0);
        let output = pid.update(1.0, 1.0, 1.0);
        assert_eq!(output, -1.0);
    }
}
