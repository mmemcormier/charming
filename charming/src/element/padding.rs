use serde::{
    de::{SeqAccess, Visitor},
    ser::SerializeSeq,
    Deserialize, Deserializer, Serialize,
};

/// Padding space around content.
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Padding {
    /// Set padding of all sides.
    Single(f64),
    /// Set top and bottom padding to the first value, and left and right
    /// padding to the second value.
    Double(f64, f64),
    /// Set top, bottom, left and right padding separately.
    Quadruple(f64, f64, f64, f64),
}

impl Serialize for Padding {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Padding::Single(padding) => serializer.serialize_f64(*padding),
            Padding::Double(top_bottom, left_right) => {
                let mut s = serializer.serialize_seq(Some(2))?;
                s.serialize_element(top_bottom)?;
                s.serialize_element(left_right)?;
                s.end()
            }
            Padding::Quadruple(top, right, bottom, left) => {
                let mut s = serializer.serialize_seq(Some(4))?;
                s.serialize_element(top)?;
                s.serialize_element(right)?;
                s.serialize_element(bottom)?;
                s.serialize_element(left)?;
                s.end()
            }
        }
    }
}

impl<'de> Deserialize<'de> for Padding {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PaddingVisitor;

        impl<'de> Visitor<'de> for PaddingVisitor {
            type Value = Padding;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a single float, a pair of floats, or four floats")
            }

            fn visit_f64<E>(self, value: f64) -> Result<Padding, E>
            where
                E: serde::de::Error,
            {
                Ok(Padding::Single(value))
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Padding, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let first: f64 = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
                if let Some(second) = seq.next_element()? {
                    if let Some(third) = seq.next_element()? {
                        let fourth: f64 = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::invalid_length(3, &self))?;
                        Ok(Padding::Quadruple(first, second, third, fourth))
                    } else {
                        Ok(Padding::Double(first, second))
                    }
                } else {
                    Ok(Padding::Single(first))
                }
            }
        }

        deserializer.deserialize_any(PaddingVisitor)
    }
}

impl From<f64> for Padding {
    fn from(padding: f64) -> Self {
        Padding::Single(padding)
    }
}

impl From<i64> for Padding {
    fn from(padding: i64) -> Self {
        Padding::Single(padding as f64)
    }
}

impl From<(f64, f64)> for Padding {
    fn from(padding: (f64, f64)) -> Self {
        Padding::Double(padding.0, padding.1)
    }
}

impl From<(i64, i64)> for Padding {
    fn from(padding: (i64, i64)) -> Self {
        Padding::Double(padding.0 as f64, padding.1 as f64)
    }
}

impl From<(f64, f64, f64, f64)> for Padding {
    fn from(padding: (f64, f64, f64, f64)) -> Self {
        Padding::Quadruple(padding.0, padding.1, padding.2, padding.3)
    }
}

impl From<(i64, i64, i64, i64)> for Padding {
    fn from(padding: (i64, i64, i64, i64)) -> Self {
        Padding::Quadruple(
            padding.0 as f64,
            padding.1 as f64,
            padding.2 as f64,
            padding.3 as f64,
        )
    }
}
