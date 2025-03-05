use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone, Copy)]
#[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
pub enum TextAlign {
    Auto,
    Left,
    Right,
    Center,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone, Copy)]
#[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
pub enum TextVerticalAlign {
    Auto,
    Top,
    Bottom,
    Middle,
}
