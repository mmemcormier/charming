#![cfg_attr(docsrs, feature(doc_cfg))]
/*!
Charming is a powerful and versatile chart rendering library for Rust that
leverages the power of [Apache Echarts](https://echarts.apache.org/en/index.html)
to deliver high-quality data visualization. Built with the Rust programming
language, this library aims to provide the Rust ecosystem with an intuitive
and effective way to generate and visualize charts, using a declarative and
user-friendly API.

## Basic Usage

Refer to the documentation of the [`Chart`] struct for how to create a chart
with various components.

Once you create a chart, you can render it into various format. Charming
provides three types of renderers:

- **HTML renderer**: [`HtmlRenderer`] renders a chart into an HTML fragments and
  offloads the actual rendering to user's web browser for an interactive,
  seamless experience. This renderer is useful when you want to render a chart
  on the client side, e.g., in a web application.
- **Image renderer**: [`ImageRenderer`] renders a chart into an image file. This
  renderer makes use of an embed [deno_core](https://github.com/denoland/deno_core)
  engine to execute the JavaScript code of Echarts and generate an image file.
  This renderer is disabled by default, and you need to enable the `ssr`
  (Server-Side Rendering) feature to use it.
  To render raster images like PNG the `ssr-raster` feature must also be enabled.
- **WASM renderer**: `WasmRenderer` renders a chart in a WebAssembly runtime.
  This renderer is disabled by default, and you need to enable the `wasm`
  feature to use it. Note that the `wasm` feature and `ssr` feature are
  mutually exclusive.

Here is an example of drawing a simple pie chart into an SVG file:

```rust
use charming::{
    component::Legend,
    element::ItemStyle,
    series::{Pie, PieRoseType},
    Chart, ImageRenderer
};

let chart = Chart::new()
    .legend(Legend::new().top("bottom"))
    .series(
        Pie::new()
            .name("Nightingale Chart")
            .rose_type(PieRoseType::Radius)
            .radius(vec!["50", "250"])
            .center(vec!["50%", "50%"])
            .item_style(ItemStyle::new().border_radius(8))
            .data(vec![
                (40.0, "rose 1"),
                (38.0, "rose 2"),
                (32.0, "rose 3"),
                (30.0, "rose 4"),
                (28.0, "rose 5"),
                (26.0, "rose 6"),
                (22.0, "rose 7"),
                (18.0, "rose 8"),
            ]),
    );

let mut renderer = ImageRenderer::new(1000, 800);
renderer.save(&chart, "/tmp/nightingale.svg");
```

## Themes

Charming supports a number of themes out of the box. You can use the
[`theme::Theme`] enum to specify a theme for your chart. For instance, the
following code snippet shows how to use the `Westeros` theme:

```rust
use charming::{Chart, ImageRenderer};
use charming::theme::Theme;
use charming::component::Title;

ImageRenderer::new(1000, 800).theme(Theme::Westeros).save(
    &Chart::new().title(Title::new().text("Westeros")),
    "/tmp/westeros.svg",
);
```

Future versions of Charming will support custom themes.
 */
pub mod component;
pub mod datatype;
pub mod element;
pub mod renderer;
pub mod series;
pub mod theme;

pub use renderer::*;

use crate::series::Getters;
use charming_macros::CharmingSetters;
use component::{
    AngleAxis, Aria, Axis, Axis3D, Calendar, DataZoom, GeoMap, Grid, Grid3D, LegendConfig,
    ParallelAxis, ParallelCoordinate, PolarCoordinate, RadarCoordinate, RadiusAxis,
    SaveAsImageType, SingleAxis, Title, Toolbox, VisualMap,
};
use datatype::Dataset;
use element::{process_raw_strings, AnimationTime, AxisPointer, Color, Easing, MarkLine, Tooltip};
use serde::{Deserialize, Serialize};
use serde_with::{formats::PreferOne, serde_as, OneOrMany};
use series::{Series, SeriesController};
/**
The chart representation.

## Anatomy of a Chart

A chart is a collection of different components, each of which is responsible
for rendering a specific part of the chart. Below is a sample chart with a
few components:

```txt
                   Sales Report
  |                                                        # coffee
30|                  x                                     x juice
  |      @           x             @                       @ milk
20|    # @           x@           x@          #
  |    #x@          #x@          #x@          #x
10|    #x@          #x@          #x@          #x@
  |    #x@          #x@          #x@          #x@
 0+-----------------------------------------------------
       Jan          Feb          Mar          Apr
```

The chart above has the following components: **an x axis**, **an y axis**,
**a title** on the top center, and **a legend** on the top right.

The creation of charts in Charming is done in a builder-like fashion. Once you
get a hang of this pattern, you will find that it is very easy to compose a
chart. For instance, the following code snippet shows how to create the chart
above:

```rust
use charming::Chart;
use charming::component::{Axis, Legend, Title};

let chart = Chart::new()
    .title(Title::new().text("Sales Report"))
    .x_axis(Axis::new().data(vec!["Jan", "Feb", "Mar", "Apr"]))
    .y_axis(Axis::new())
    .legend(Legend::new().data(vec!["coffee", "juice", "milk"]));
```

## Components of a Chart

The following sections describe the components of a chart in detail.

### Title

[`Title`] is the title of a chart, including main title and subtitle. A chart
can have multiple titles, which is useful when you want to show multiple sub-
charts in a single chart.

```rust
use charming::Chart;
use charming::component::Title;

let chart = Chart::new()
    .title(Title::new().text("Sales Report"));
```

### Legend

[`Legend`](crate::component::Legend) is the legend of a chart, which is used to show the meaning of the
symbols and colors in the chart. A chart can have multiple legends.

```rust
use charming::Chart;
use charming::component::Legend;

let chart = Chart::new()
    .legend(Legend::new().data(vec!["coffee", "juice", "milk"]));
```

### Grid

[`Grid`] is the background grid in a cartesian coordinate system. A chart can
have multiple grids.

```rust
use charming::Chart;
use charming::component::Grid;

let chart = Chart::new()
    .grid(Grid::new());
```

### X Axis and Y Axis

[`Axis`] is the axis in a cartesian coordinate system.

```rust
use charming::Chart;
use charming::component::Axis;

let chart = Chart::new()
    .x_axis(Axis::new().data(vec!["Jan", "Feb", "Mar", "Apr"]))
    .y_axis(Axis::new());
```

### Polar Coordinate

[`PolarCoordinate`] is the polar coordinate system. Polar coordinate can be used in
scatter and line charts. Every polar coordinate has an [`AngleAxis`] and a
[`RadiusAxis`].

### Radar Coordinate

[`RadarCoordinate`] is the radar coordinate system. Radar coordinate can be in
radar charts.

### Data Zoom

[`DataZoom`] is used for zooming a specific area, which enables user to view
data in different scales. A chart can have multiple data zooms.

### Visual Map

[`VisualMap`] is a visual encoding component. It maps data to visual channels,
such as color, symbol size or symbol shape. A chart can have multiple visual
maps.

### Tooltip

[`Tooltip`] is a floating box that appears when user hovers over a data item.

### AxisPointer

[`AxisPointer`] is a tool for displaying reference line and axis value under
mouse pointer.

### Toolbox

[`Toolbox`] is a feature toolbox that includes data view, save as image, data
zoom, restore, and reset.
 */
#[serde_as]
#[serde_with::apply(
  Option => #[serde(skip_serializing_if = "Option::is_none")],
  Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct Chart {
    title: Vec<Title>,
    animation: Option<bool>,
    animation_duration: Option<AnimationTime>,
    animation_threshold: Option<f64>,
    animation_easing: Option<Easing>,
    animation_delay: Option<AnimationTime>,
    animation_duration_update: Option<AnimationTime>,
    animation_easing_update: Option<Easing>,
    animation_delay_update: Option<AnimationTime>,
    tooltip: Option<Tooltip>,
    legend: Option<LegendConfig>,
    toolbox: Option<Toolbox>,
    grid: Vec<Grid>,
    #[serde(rename = "grid3D")]
    grid3d: Vec<Grid3D>,
    #[serde_as(as = "OneOrMany<_, PreferOne>")]
    x_axis: Vec<Axis>,
    #[serde(rename = "xAxis3D")]
    x_axis3d: Vec<Axis3D>,
    #[serde_as(as = "OneOrMany<_, PreferOne>")]
    y_axis: Vec<Axis>,
    #[serde(rename = "yAxis3D")]
    y_axis3d: Vec<Axis3D>,
    #[serde(rename = "zAxis3D")]
    z_axis3d: Vec<Axis3D>,
    polar: Vec<PolarCoordinate>,
    angle_axis: Vec<AngleAxis>,
    radius_axis: Vec<RadiusAxis>,
    single_axis: Option<SingleAxis>,
    parallel_axis: Vec<ParallelAxis>,
    axis_pointer: Vec<AxisPointer>,
    visual_map: Vec<VisualMap>,
    data_zoom: Vec<DataZoom>,
    parallel: Option<ParallelCoordinate>,
    calendar: Option<Calendar>,
    dataset: Option<Dataset>,
    radar: Vec<RadarCoordinate>,
    #[charming_set_vec]
    color: Vec<Color>,
    background_color: Option<Color>,
    mark_line: Option<MarkLine>,
    aria: Option<Aria>,
    series: Vec<Series>,
    #[serde(skip_serializing)]
    geo_map: Vec<GeoMap>,
}
impl Chart {
    pub fn save_as_image_type(&self) -> Option<&SaveAsImageType> {
        self.toolbox
            .as_ref()
            .and_then(|toolbox| toolbox.save_as_image_type())
    }
}

impl Chart {
    pub fn get_color_ref(&self) -> &Vec<Color> {
        &self.color
    }

    pub fn get_color_mut(&mut self) -> &mut Vec<Color> {
        &mut self.color
    }

    pub fn get_all_ids(&self) -> Vec<String> {
        self.series
            .iter()
            .map(|s| s.get_series_id().unwrap())
            .collect()
    }

    // !!! Could this just match on series and return the underlying series type?
    // getters and setters would only need to be implemented on the underlying type
    // and the need for wrapper methods would be avoided.
    pub fn get_series_mut(&mut self, id: &String) -> Option<&mut Series> {
        if let Some(index) = self
            .series
            .iter()
            .position(|s| s.get_series_id() == Some(id.clone()))
        {
            Some(&mut self.series[index])
        } else {
            None
        }
    }

    pub fn get_all_series_ref(&self) -> &Vec<Series> {
        &self.series
    }

    pub fn get_all_series_mut(&mut self) -> &mut Vec<Series> {
        &mut self.series
    }

    pub fn reset_x_axis(&mut self) {
        self.x_axis = vec![]
    }

    pub fn reset_y_axis(&mut self) {
        self.y_axis = vec![]
    }
}

impl Chart {
    pub fn with_mutable<F>(&mut self, f: F) -> &mut Self
    where
        F: FnOnce(&mut ChartController),
    {
        let mut controller = ChartController { chart: self };
        f(&mut controller);
        self
    }
}

pub struct ChartController<'a> {
    chart: &'a mut Chart,
}

impl<'a> ChartController<'a> {
    pub fn reset_x_axis(&mut self) -> &mut Self {
        self.chart.x_axis = vec![];
        self
    }
    pub fn with_x_axis(&mut self, axis: Axis) -> &mut Self {
        self.chart.x_axis.push(axis);
        self
    }
    pub fn reset_y_axis(&mut self) -> &mut Self {
        self.chart.y_axis = vec![];
        self
    }
    pub fn with_y_axis(&mut self, axis: Axis) -> &mut Self {
        self.chart.y_axis.push(axis);
        self
    }
    pub fn reset_series(&mut self) -> &mut Self {
        self.chart.series = vec![];
        self
    }
    pub fn with_series<S: Into<Series>>(&mut self, series: S) -> &mut Self {
        self.chart.series.push(series.into());
        self
    }
    pub fn series_mut_by_index(&mut self, index: usize) -> SeriesController<'_> {
        let series = self
            .chart
            .series
            .get_mut(index)
            .expect("Index out of bounds for series vector.");
        SeriesController::new(series)
    }
}

impl std::fmt::Display for Chart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            process_raw_strings(&serde_json::to_string_pretty(self).unwrap())
        )
    }
}

#[derive(Debug)]
pub enum EchartsError {
    HtmlRenderingError(String),
    ImageRenderingError(String),
    JsRuntimeError(String),
    WasmError(String),
}

impl std::error::Error for EchartsError {}
impl std::fmt::Display for EchartsError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::HtmlRenderingError(msg) => write!(f, "HTML rendering error: {msg}"),
            Self::ImageRenderingError(msg) => write!(f, "Image rendering error: {msg}"),
            Self::JsRuntimeError(msg) => write!(f, "JavaScript runtime error: {msg}"),
            Self::WasmError(msg) => write!(f, "WebAssembly runtime error: {msg}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // use claims::{assert_ok, assert_ok_eq};
    use component::Legend;
    use series::line::Line;
    // use serde_assert::{Deserializer, Serializer, Token};

    #[test]
    fn test_chart_round_trip() {
        // let chart = Chart::new().legend(Legend::new().show(true));
        let chart = Chart::new()
            .legend(Legend::new().show(true))
            .series(Series::Line(Line::new().data(vec![vec![0, 1], vec![2, 3]])));
        println!("Chart:\n {}", chart);
        let chart_json = serde_json::to_string(&chart).unwrap();
        println!("{}", chart_json);
        let chart_de = serde_json::from_str(&chart_json).unwrap();
        assert_eq!(chart, chart_de);
    }
    // #[test]
    // fn test_chart_serialization() {
    //     let value = Chart::new().legend(Legend::new().show(true));
    //     let serializer = Serializer::builder().build();
    //     let tokens = assert_ok!(value.serialize(&serializer));
    //     assert_eq!(
    //         tokens,
    //         [
    //             Token::Struct {
    //                 name: "Chart",
    //                 len: 1
    //             },
    //             Token::Struct {
    //                 name: "Legend",
    //                 len: 1
    //             },
    //             Token::StructEnd,
    //         ]
    //     );
    // }
}
