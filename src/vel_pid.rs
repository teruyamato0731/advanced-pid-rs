use super::FloatType;
use super::PidConfig;
use super::PidController;

#[derive(Debug, Clone)]
pub struct VelPid {
    config: PidConfig,
    output: FloatType,
    pre_error: FloatType,
    pre_p_term: FloatType,
    d_term_lpf: FloatType,
}

impl Default for VelPid {
    fn default() -> Self {
        Self::new(PidConfig::default())
    }
}

impl PidController for VelPid {
    fn new(config: PidConfig) -> Self {
        Self {
            config,
            output: 0.0,
            pre_error: FloatType::NAN,
            pre_p_term: FloatType::NAN,
            d_term_lpf: 0.0,
        }
    }
    fn update(&mut self, set_point: FloatType, actual: FloatType, dt: FloatType) -> FloatType {
        debug_assert!(dt > 0.0, "dt must be positive");
        let error = set_point - actual;
        let p_term = if self.pre_error.is_nan() {
            0.0
        } else {
            (error - self.pre_error) / dt
        };
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
        self.output = (self.output + du).clamp(self.config.min, self.config.max);
        self.output
    }
}
