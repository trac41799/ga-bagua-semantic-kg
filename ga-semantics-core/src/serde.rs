use crate::{Blade, Multivector};
use serde::{Deserialize, Serialize};

impl Serialize for Multivector {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.coefficients().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Multivector {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let coeffs = <[f64; 8]>::deserialize(deserializer)?;
        Ok(Multivector::new(coeffs))
    }
}

impl Serialize for Blade {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.index().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Blade {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let idx = usize::deserialize(deserializer)?;
        Blade::from_index(idx).ok_or_else(|| serde::de::Error::custom(format!(
            "invalid blade index: {idx}, expected 0..7"
        )))
    }
}
