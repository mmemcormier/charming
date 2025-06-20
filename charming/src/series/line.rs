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

// Need to implement controller because Line has private fields.
pub struct LineController<'a> {
    line: &'a mut Line,
}

impl<'a> LineController<'a> {
    pub fn new(line: &'a mut Line) -> Self {
        LineController { line }
    }
    // Line-specific methods
    pub fn with_name<S: Into<String>>(&mut self, name: S) -> &mut Self {
        self.line.name = Some(name.into());
        self
    }

    pub fn with_area_style<A: Into<AreaStyle>>(&mut self, style: A) -> &mut Self {
        self.line.area_style = Some(style.into());
        self
    }

    pub fn with_connect_nulls(&mut self, connect: bool) -> &mut Self {
        self.line.connect_nulls = Some(connect);
        self
    }

    pub fn with_coordinate_system<C: Into<CoordinateSystem>>(&mut self, system: C) -> &mut Self {
        self.line.coordinate_system = Some(system.into());
        self
    }

    pub fn with_data<D: Into<DataPoint>>(&mut self, data: Vec<D>) -> &mut Self {
        self.line.data = data.into_iter().map(|d| d.into()).collect();
        self
    }

    pub fn with_dataset_id<S: Into<String>>(&mut self, id: S) -> &mut Self {
        self.line.dataset_id = Some(id.into());
        self
    }

    pub fn with_emphasis<E: Into<Emphasis>>(&mut self, emphasis: E) -> &mut Self {
        self.line.emphasis = Some(emphasis.into());
        self
    }

    pub fn with_encode<E: Into<DimensionEncode>>(&mut self, encode: E) -> &mut Self {
        self.line.encode = Some(encode.into());
        self
    }

    pub fn with_item_style<I: Into<ItemStyle>>(&mut self, style: I) -> &mut Self {
        self.line.item_style = Some(style.into());
        self
    }

    pub fn with_label<L: Into<Label>>(&mut self, label: L) -> &mut Self {
        self.line.label = Some(label.into());
        self
    }

    pub fn with_line_style<L: Into<LineStyle>>(&mut self, style: L) -> &mut Self {
        self.line.line_style = Some(style.into());
        self
    }

    pub fn with_mark_area<M: Into<MarkArea>>(&mut self, area: M) -> &mut Self {
        self.line.mark_area = Some(area.into());
        self
    }

    pub fn with_mark_line<M: Into<MarkLine>>(&mut self, line: M) -> &mut Self {
        self.line.mark_line = Some(line.into());
        self
    }

    pub fn with_mark_point<M: Into<MarkPoint>>(&mut self, point: M) -> &mut Self {
        self.line.mark_point = Some(point.into());
        self
    }

    pub fn with_sampling(&mut self, sampling: Sampling) -> &mut Self {
        self.line.sampling = Some(sampling);
        self
    }

    pub fn with_silent(&mut self, silent: bool) -> &mut Self {
        self.line.silent = Some(silent);
        self
    }

    pub fn with_smooth<S: Into<Smoothness>>(&mut self, smooth: S) -> &mut Self {
        self.line.smooth = Some(smooth.into());
        self
    }

    pub fn with_stack<S: Into<String>>(&mut self, stack: S) -> &mut Self {
        self.line.stack = Some(stack.into());
        self
    }

    pub fn with_step<S: Into<Step>>(&mut self, step: S) -> &mut Self {
        self.line.step = Some(step.into());
        self
    }

    pub fn with_symbol_size<S: Into<SymbolSize>>(&mut self, size: S) -> &mut Self {
        self.line.symbol_size = Some(size.into());
        self
    }

    pub fn with_tooltip(&mut self, tooltip: Tooltip) -> &mut Self {
        self.line.tooltip = Some(tooltip);
        self
    }

    pub fn with_x_axis_index<F: Into<f64>>(&mut self, index: F) -> &mut Self {
        self.line.x_axis_index = Some(index.into());
        self
    }

    pub fn with_y_axis_index<F: Into<f64>>(&mut self, index: F) -> &mut Self {
        self.line.y_axis_index = Some(index.into());
        self
    }

    pub fn with_z<I: Into<i32>>(&mut self, z: I) -> &mut Self {
        self.line.z = Some(z.into());
        self
    }
}

impl Line {
    pub fn get_id(&self) -> Option<String> {
        self.id.clone()
    }

    pub fn get_name(&self) -> Option<String> {
        self.name.clone()
    }
    pub fn set_name<S: Into<String>>(&mut self, name: S) {
        self.name = Some(name.into())
    }

    pub fn get_symbol(&self) -> &Option<Symbol> {
        &self.symbol
    }

    pub fn set_symbol<S: Into<Symbol>>(&mut self, symbol: S) {
        self.symbol = Some(symbol.into())
    }

    pub fn get_show_symbol(&self) -> Option<bool> {
        self.show_symbol.clone()
    }
    pub fn set_show_symbol(&mut self, show_symbol: bool) {
        self.show_symbol = Some(show_symbol)
    }

    pub fn set_smoothness<S: Into<Smoothness>>(&mut self, smoothness: S) {
        self.smooth = Some(smoothness.into())
    }
    pub fn get_smoothness(&self) -> Option<Smoothness> {
        self.smooth.clone()
    }

    pub fn get_linestyle(&self) -> Option<LineStyle> {
        self.line_style.clone()
    }
    pub fn set_linestyle<L: Into<LineStyle>>(&mut self, line_style: L) {
        self.line_style = Some(line_style.into())
    }

    pub fn get_data(&self) -> &Vec<DataPoint> {
        &self.data
    }
    pub fn set_data<D: Into<DataPoint>>(&mut self, new_data: Vec<D>) {
        self.data = new_data.into_iter().map(|d| d.into()).collect()
    }
}
