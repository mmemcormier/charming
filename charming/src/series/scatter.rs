use crate::{
    datatype::{DataFrame, DataPoint},
    element::{
        ColorBy, CoordinateSystem, DimensionEncode, Emphasis, ItemStyle, Label, MarkArea, MarkLine,
        Symbol, SymbolSize,
    },
};
use charming_macros::CharmingSetters;
use serde::{Deserialize, Serialize};

#[serde_with::apply(
  Option => #[serde(skip_serializing_if = "Option::is_none")],
  Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Scatter {
    #[serde(rename = "type")]
    #[charming_type = "scatter"]
    type_: String,
    id: Option<String>,
    name: Option<String>,
    color_by: Option<ColorBy>,
    label: Option<Label>,
    dataset_index: Option<f64>,
    coordinate_system: Option<CoordinateSystem>,
    x_axis_index: Option<f64>,
    y_axis_index: Option<f64>,
    symbol: Option<Symbol>,
    symbol_size: Option<SymbolSize>,
    encode: Option<DimensionEncode>,
    mark_line: Option<MarkLine>,
    mark_area: Option<MarkArea>,
    item_style: Option<ItemStyle>,
    emphasis: Option<Emphasis>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    data: DataFrame,
}


pub struct ScatterController<'a> {
    scatter: &'a mut Scatter,
}

// Scatter-specific mutations
// pub struct ScatterMutator<'a>(&'a mut Scatter);

// impl<'a> SeriesVariantMutator for ScatterMutator<'a> {
//     fn set_name<S: Into<String>>(&mut self, name: S) {
//         self.0.name = Some(name.into());
//     }
// }

impl<'a> ScatterController<'a> {
    pub fn new(scatter: &'a mut Scatter) -> Self {
        ScatterController { scatter }
    }
    // Scatter-specific methods
    pub fn with_marker_size<S: Into<SymbolSize>>(&mut self, size: S) -> &mut Self {
        self.scatter.symbol_size = Some(size.into());
        self
    }
    pub fn with_name<S: Into<String>>(&mut self, name: S) -> &mut Self {
        self.scatter.name = Some(name.into());
        self
    }
}
