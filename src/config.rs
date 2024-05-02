//! The `config` module provides structures for configuring a PID controller.
//!
//! [`Gain`] is a structure that holds the proportional, integral, and derivative gains for a PID controller.
//! [`Config`] is a structure that holds a [`Gain`] and also provides optional limits for the controller output.
//!
//! # Examples
//!
//! ```rust
//! use advanced_pid::config::{Config, Gain};
//!
//! let gain = Gain { kp: 1.0, ki: 0.1, kd: 0.1 };
//! let config = Config::from(gain);
//!
//! let config_with_limits = Config::new(1.0, 0.1, 0.1).with_limits(-1.0, 1.0);
//! ```
use super::FloatType;

/// `Gain` holds the proportional, integral, and derivative gains for a PID controller.
///
/// See also: [`Config`]
#[derive(Debug, Clone, Default)]
pub struct Gain {
    pub kp: FloatType,
    pub ki: FloatType,
    pub kd: FloatType,
}

/// `Config` holds a [`Gain`] and also provides optional limits for the controller output.
#[derive(Debug, Clone)]
pub struct Config {
    pub gain: Gain,
    pub min: FloatType,
    pub max: FloatType,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            gain: Default::default(),
            min: FloatType::NEG_INFINITY,
            max: FloatType::INFINITY,
        }
    }
}

impl Config {
    /// Creates a new `Config` with the specified gains.
    pub fn new(kp: FloatType, ki: FloatType, kd: FloatType) -> Self {
        Self {
            gain: Gain { kp, ki, kd },
            ..Default::default()
        }
    }

    /// Returns a new `Config` with the specified limits.
    pub fn with_limits(self, min: FloatType, max: FloatType) -> Self {
        Self { min, max, ..self }
    }
}

impl From<Gain> for Config {
    /// Converts a `Gain` into a `Config`.
    fn from(gain: Gain) -> Self {
        Self {
            gain,
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gain_default() {
        let gain = Gain::default();
        assert_eq!(gain.kp, 0.0);
        assert_eq!(gain.ki, 0.0);
        assert_eq!(gain.kd, 0.0);
    }

    #[test]
    fn test_config_new() {
        let config = Config::new(1.0, 0.5, 0.1);
        assert_eq!(config.gain.kp, 1.0);
        assert_eq!(config.gain.ki, 0.5);
        assert_eq!(config.gain.kd, 0.1);
        assert_eq!(config.min, FloatType::NEG_INFINITY);
        assert_eq!(config.max, FloatType::INFINITY);
    }

    #[test]
    fn test_config_with_limits() {
        let config = Config::new(1.0, 0.5, 0.1).with_limits(-2.0, 2.0);
        assert_eq!(config.gain.kp, 1.0);
        assert_eq!(config.gain.ki, 0.5);
        assert_eq!(config.gain.kd, 0.1);
        assert_eq!(config.min, -2.0);
        assert_eq!(config.max, 2.0);
    }

    #[test]
    fn test_config_from_gain() {
        let gain = Gain {
            kp: 1.0,
            ki: 0.5,
            kd: 0.1,
        };
        let config = Config::from(gain);
        assert_eq!(config.gain.kp, 1.0);
        assert_eq!(config.gain.ki, 0.5);
        assert_eq!(config.gain.kd, 0.1);
        assert_eq!(config.min, FloatType::NEG_INFINITY);
        assert_eq!(config.max, FloatType::INFINITY);
    }

    #[test]
    fn test_config_default() {
        let config = Config::default();
        assert_eq!(config.gain.kp, 0.0);
        assert_eq!(config.gain.ki, 0.0);
        assert_eq!(config.gain.kd, 0.0);
        assert_eq!(config.min, FloatType::NEG_INFINITY);
        assert_eq!(config.max, FloatType::INFINITY);
    }
}
