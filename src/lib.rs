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

pub type PidGain = config::Gain;
pub type PidConfig = config::Config;

pub use crate::i_pd::Ipd;
pub use crate::pi_d::PiD;
pub use crate::pid::Pid;
pub use crate::vel_pid::VelPid;

pub trait PidController {
    fn new(config: PidConfig) -> Self;
    fn update(&mut self, set_point: FloatType, actual: FloatType, dt: FloatType) -> FloatType;
    fn reset_config(&mut self, config: PidConfig)
    where
        Self: core::marker::Sized,
    {
        *self = Self::new(config);
    }
}
