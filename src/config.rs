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
