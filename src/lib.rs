//! An advanced PID control library implemented in Rust.
//!
//! This library provides an interface for PID controllers and several implementations.
//! It supports various types of PID controls and allows for customization of PID gains and limits.
//! It is designed to be `no_std` compatible, but can also be used with `std`.
//!
//! ## Features
//! - Standard (Positional) PID Control ([`pid`] module)
//! - Velocity form PID Control ([`vel_pid`] module)
//! - PI-D Control where the Derivative action is based on the Process Variable (PV) ([`pi_d`] module)
//! - I-PD Control where both Proportional and Derivative actions are based on the Process Variable (PV) ([`i_pd`] module)
//! - Customizable PID gains and limits ([`config`] module)
//!
//! ## Usage
//! To use, implement the [`PidController`] trait for your controller.
//! The trait provides a `new` method for creating a new controller, an `update` method for updating the controller,
//! and a `reset_config` method for resetting the controller's configuration.
//!
//! ## Installation
//! To install, run the following Cargo command in your project directory:
//! ```bash
//! cargo add advanced-pid
//! ```
//!
//! Or add the following to your Cargo.toml:
//! ```toml
//! [dependencies]
//! advanced-pid = "0.2.1"
//! ```
//!
//! ## No-std support
//! This library is designed to be `no_std` compatible.
//! To use this library in a `no_std` environment, disable the default features in your `Cargo.toml`:
//! ```toml
//! [dependencies]
//! advanced-pid = { version = "0.2.1", default-features = false }
//! ```
//!
//! ## Floating point precision
//! This library allows switching between `f32` and `f64` floating point types through feature flags.
//! By default, `f32` precision is used.
//! To use `f64` precision, enable the `f64` feature in your `Cargo.toml`:
//! ```toml
//! [dependencies]
//! advanced-pid = { version = "0.2.1", features = ["f64"] }
//! ```
//!
//! ## Examples
//! ```
//! use advanced_pid::{prelude::*, Pid, PidGain};
//!
//! let target = 1.0;
//! let actual = 0.0;
//! let dt = 1.0;
//!
//! let gain = PidGain {
//!     kp: 1.0,
//!     ki: 0.3,
//!     kd: 0.1,
//! };
//! let mut pid = Pid::new(gain.into());
//! println!("{}", pid.update(target, actual, dt));
//! ```
//!
//! For more examples, see the [examples](https://github.com/teruyamato0731/advanced-pid-rs/tree/main/examples).

#![no_std]
#[cfg(feature = "std")]
#[allow(unused_imports)]
#[macro_use]
extern crate std;

#[cfg(not(feature = "f64"))]
type FloatType = f32;
#[cfg(feature = "f64")]
type FloatType = f64;

pub mod prelude;

pub mod config;
pub mod i_pd;
pub mod pi_d;
pub mod pid;
pub mod vel_pid;

/// Type alias for [PID gains](config::Gain).
pub type PidGain = config::Gain;
/// Type alias for [PID configuration](config::Config).
pub type PidConfig = config::Config;

pub use crate::i_pd::Ipd;
pub use crate::pi_d::PiD;
pub use crate::pid::Pid;
pub use crate::vel_pid::VelPid;

/// `PidController` is a trait that provides a standard interface for PID controllers.
///
/// It provides methods for creating a new controller [`Self::new()`], updating the controller [`Self::update()`], and resetting the controller's configuration [`Self::reset_config()`].
pub trait PidController {
    /// Creates a new controller with the specified configuration.
    /// ```
    /// use advanced_pid::{prelude::*, Pid, PidConfig};
    ///
    /// let config = PidConfig::new(1.0, 0.3, 0.1);
    /// let controller = Pid::new(config);
    /// ```
    fn new(config: PidConfig) -> Self;

    /// Updates the controller with the specified set point, actual value, and time delta.
    /// Returns the controller output.
    /// ```
    /// use advanced_pid::{prelude::*, Pid};
    ///
    /// let mut controller = Pid::default();
    /// let output = controller.update(1.0, 0.0, 0.1);
    /// ```
    fn update(&mut self, set_point: FloatType, actual: FloatType, dt: FloatType) -> FloatType;

    /// Resets the controller's configuration to the specified configuration.
    /// ```
    /// use advanced_pid::{prelude::*, Pid, PidConfig};
    ///
    /// let mut controller = Pid::default();
    /// let config = PidConfig::new(1.0, 0.3, 0.1);
    /// controller.reset_config(config);
    /// ```
    fn reset_config(&mut self, config: PidConfig)
    where
        Self: core::marker::Sized,
    {
        *self = Self::new(config);
    }
}
