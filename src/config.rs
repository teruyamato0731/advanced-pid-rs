use super::FloatType;

#[derive(Debug, Clone, Default)]
pub struct Gain {
    pub kp: FloatType,
    pub ki: FloatType,
    pub kd: FloatType,
}

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
    pub fn new(kp: FloatType, ki: FloatType, kd: FloatType) -> Self {
        Self {
            gain: Gain { kp, ki, kd },
            ..Default::default()
        }
    }

    pub fn with_limits(self, min: FloatType, max: FloatType) -> Self {
        Self { min, max, ..self }
    }
}

impl From<Gain> for Config {
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
