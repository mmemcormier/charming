use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone, Copy)]
#[serde(rename_all(serialize = "snake_case", deserialize = "PascalCase"))]
pub enum CoordinateSystem {
    Cartesian2d,
    Polar,
    Single,
    Geo,
    Calendar,
    Parallel,
}
