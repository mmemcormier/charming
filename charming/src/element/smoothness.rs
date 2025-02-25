use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
// #[serde(tag = "type", content = "value")]
pub enum Smoothness {
    Single(f64),
    Boolean(bool),
}

impl Serialize for Smoothness {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Smoothness::Single(smoothness) => serializer.serialize_f64(*smoothness),
            Smoothness::Boolean(smoothness) => serializer.serialize_bool(*smoothness),
        }
    }
}

// impl Deserialize for Smoothness {
//     fn deserialize<S>(&self, deserializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: serde::Deserializer,
//     {
//         match self {
//             Smoothness::Single(smoothness) => deserializer.deserialize_f64(*smoothness),
//             Smoothness::Boolean(smoothness) => deserializer.deserialize_bool(*smoothness),
//         }
//     }
// }

impl<'de> Deserialize<'de> for Smoothness {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct SmoothnessVisitor;

        impl<'de> serde::de::Visitor<'de> for SmoothnessVisitor {
            type Value = Smoothness;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a float or a boolean")
            }

            fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Smoothness::Single(value))
            }

            fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Smoothness::Boolean(value))
            }
        }

        deserializer.deserialize_any(SmoothnessVisitor)
    }
}

impl From<f64> for Smoothness {
    fn from(value: f64) -> Self {
        Smoothness::Single(value)
    }
}

impl From<f32> for Smoothness {
    fn from(value: f32) -> Self {
        Smoothness::Single(value as f64)
    }
}

impl From<i32> for Smoothness {
    fn from(value: i32) -> Self {
        Smoothness::Single(value as f64)
    }
}

impl From<u32> for Smoothness {
    fn from(value: u32) -> Self {
        Smoothness::Single(value as f64)
    }
}

impl From<bool> for Smoothness {
    fn from(value: bool) -> Self {
        Smoothness::Boolean(value)
    }
}
