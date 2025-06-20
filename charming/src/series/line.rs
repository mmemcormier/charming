use crate::{
    datatype::{DataFrame, DataPoint},
    element::{
        smoothness::Smoothness, AreaStyle, CoordinateSystem, DimensionEncode, Emphasis, ItemStyle,
        Label, LineStyle, MarkArea, MarkLine, MarkPoint, Sampling, Step, Symbol, SymbolSize,
        Tooltip,
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
pub struct Line {
    #[serde(rename = "type")]
    #[charming_type = "line"]
    type_: String,
    id: Option<String>,
    name: Option<String>,
    coordinate_system: Option<CoordinateSystem>,
    symbol: Option<Symbol>,
    symbol_size: Option<SymbolSize>,
    show_symbol: Option<bool>,
    stack: Option<String>,
    sampling: Option<Sampling>,
    label: Option<Label>,
    line_style: Option<LineStyle>,
    area_style: Option<AreaStyle>,
    item_style: Option<ItemStyle>,
    emphasis: Option<Emphasis>,
    smooth: Option<Smoothness>,
    step: Option<Step>,
    connect_nulls: Option<bool>,
    mark_point: Option<MarkPoint>,
    mark_line: Option<MarkLine>,
    mark_area: Option<MarkArea>,
    dataset_id: Option<String>,
    encode: Option<DimensionEncode>,
    x_axis_index: Option<f64>,
    y_axis_index: Option<f64>,
    tooltip: Option<Tooltip>,
    silent: Option<bool>,
    z: Option<i32>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    data: DataFrame,
}

impl Default for Line {
    fn default() -> Self {
        Self::new()
    }
}

impl Line {
    pub fn new() -> Self {
        Self {
            type_: "line".to_string(),
            id: None,
            name: None,
            coordinate_system: None,
            symbol: None,
            symbol_size: None,
            show_symbol: None,
            stack: None,
            sampling: None,
            label: None,
            line_style: None,
            area_style: None,
            item_style: None,
            emphasis: None,
            smooth: None,
            step: None,
            connect_nulls: None,
            mark_point: None,
            mark_line: None,
            mark_area: None,
            dataset_id: None,
            encode: None,
            x_axis_index: None,
            y_axis_index: None,
            tooltip: None,
            silent: None,
            z: None,
            data: vec![],
        }
    }

    pub fn id<S: Into<String>>(mut self, id: S) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Series name used for displaying in `tooltip` and filtering with `legend`.
    pub fn name<S: Into<String>>(mut self, name: S) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn coordinate_system<C: Into<CoordinateSystem>>(mut self, coordinate_system: C) -> Self {
        self.coordinate_system = Some(coordinate_system.into());
        self
    }

    pub fn symbol<S: Into<Symbol>>(mut self, symbol: S) -> Self {
        self.symbol = Some(symbol.into());
        self
    }

    pub fn symbol_size<F: Into<SymbolSize>>(mut self, symbol_size: F) -> Self {
        self.symbol_size = Some(symbol_size.into());
        self
    }

    pub fn show_symbol(mut self, show_symbol: bool) -> Self {
        self.show_symbol = Some(show_symbol);
        self
    }

    pub fn stack<S: Into<String>>(mut self, stack: S) -> Self {
        self.stack = Some(stack.into());
        self
    }

    pub fn sampling(mut self, sampling: Sampling) -> Self {
        self.sampling = Some(sampling);
        self
    }

    pub fn label<L: Into<Label>>(mut self, label: L) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn line_style<L: Into<LineStyle>>(mut self, line_style: L) -> Self {
        self.line_style = Some(line_style.into());
        self
    }

    pub fn area_style<A: Into<AreaStyle>>(mut self, area_style: A) -> Self {
        self.area_style = Some(area_style.into());
        self
    }

    pub fn item_style<I: Into<ItemStyle>>(mut self, item_style: I) -> Self {
        self.item_style = Some(item_style.into());
        self
    }

    pub fn emphasis<E: Into<Emphasis>>(mut self, emphasis: E) -> Self {
        self.emphasis = Some(emphasis.into());
        self
    }

    /// Smoothness.
    pub fn smooth<S: Into<Smoothness>>(mut self, smooth: S) -> Self {
        self.smooth = Some(smooth.into());
        self
    }

    pub fn step<S: Into<Step>>(mut self, step: S) -> Self {
        self.step = Some(step.into());
        self
    }

    pub fn connect_nulls(mut self, connect_nulls: bool) -> Self {
        self.connect_nulls = Some(connect_nulls);
        self
    }

    pub fn mark_point<M: Into<MarkPoint>>(mut self, mark_point: M) -> Self {
        self.mark_point = Some(mark_point.into());
        self
    }

    pub fn mark_line<M: Into<MarkLine>>(mut self, mark_line: M) -> Self {
        self.mark_line = Some(mark_line.into());
        self
    }

    pub fn mark_area<M: Into<MarkArea>>(mut self, mark_area: M) -> Self {
        self.mark_area = Some(mark_area.into());
        self
    }

    pub fn dataset_id<S: Into<String>>(mut self, dataset_id: S) -> Self {
        self.dataset_id = Some(dataset_id.into());
        self
    }

    pub fn encode<E: Into<DimensionEncode>>(mut self, encode: E) -> Self {
        self.encode = Some(encode.into());
        self
    }

    pub fn x_axis_index<F: Into<f64>>(mut self, x_axis_index: F) -> Self {
        self.x_axis_index = Some(x_axis_index.into());
        self
    }

    pub fn y_axis_index<F: Into<f64>>(mut self, y_axis_index: F) -> Self {
        self.y_axis_index = Some(y_axis_index.into());
        self
    }

    pub fn tooltip(mut self, tooltip: Tooltip) -> Self {
        self.tooltip = Some(tooltip);
        self
    }

    pub fn silent(mut self, silent: bool) -> Self {
        self.silent = Some(silent);
        self
    }

    pub fn z<I: Into<i32>>(mut self, z: I) -> Self {
        self.z = Some(z.into());
        self
    }

    pub fn data<D: Into<DataPoint>>(mut self, data: Vec<D>) -> Self {
        self.data = data.into_iter().map(|d| d.into()).collect();
        self
    }
}

impl Line {
    pub fn get_id(&self) -> Option<String> {
        self.id.clone()
    }

    pub fn set_show_symbol(&mut self, show_symbol: bool) {
        self.show_symbol = Some(show_symbol)
    }
}
