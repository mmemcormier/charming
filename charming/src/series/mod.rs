#![allow(clippy::large_enum_variant)]

use crate::element::{smoothness::Smoothness, LineStyle, Symbol};
use serde::{Deserialize, Serialize};
use thiserror;

pub mod bar;
pub mod bar3d;
pub mod boxplot;
pub mod candlestick;
pub mod custom;
pub mod effect_scatter;
pub mod funnel;
pub mod gauge;
pub mod graph;
pub mod heatmap;
pub mod line;
pub mod lines;
pub mod map;
pub mod parallel;
pub mod pictorial_bar;
pub mod pie;
pub mod radar;
pub mod sankey;
pub mod scatter;
pub mod sunburst;
pub mod theme_river;
pub mod tree;
pub mod treemap;

pub use bar::*;
pub use bar3d::*;
pub use boxplot::*;
pub use candlestick::*;
pub use custom::*;
pub use effect_scatter::*;
pub use funnel::*;
pub use gauge::*;
pub use graph::*;
pub use heatmap::*;
pub use line::*;
pub use lines::*;
pub use map::*;
pub use parallel::*;
pub use pictorial_bar::*;
pub use pie::*;
pub use radar::*;
pub use sankey::*;
pub use scatter::*;
pub use sunburst::*;
pub use theme_river::*;
pub use tree::*;
pub use treemap::*;

use crate::datatype::DataPoint;

#[derive(Debug, PartialEq, Clone)]
pub enum Series {
    Bar(bar::Bar),
    Bar3d(bar3d::Bar3d),
    Boxplot(boxplot::Boxplot),
    Candlestick(candlestick::Candlestick),
    Custom(custom::Custom),
    EffectScatter(effect_scatter::EffectScatter),
    Funnel(funnel::Funnel),
    Gauge(gauge::Gauge),
    Graph(graph::Graph),
    Heatmap(heatmap::Heatmap),
    Line(line::Line),
    Map(map::Map),
    Parallel(parallel::Parallel),
    PictorialBar(pictorial_bar::PictorialBar),
    Pie(pie::Pie),
    Radar(radar::Radar),
    Sankey(sankey::Sankey),
    Scatter(scatter::Scatter),
    Sunburst(sunburst::Sunburst),
    ThemeRiver(theme_river::ThemeRiver),
    Tree(tree::Tree),
    Treemap(treemap::Treemap),
}

macro_rules! impl_series_deserialize {
    ($($variant:ident => $type_str:literal),* $(,)?) => {
        impl<'de> serde::Deserialize<'de> for Series {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let value = serde_json::Value::deserialize(deserializer)?;

                let type_str = value
                    .get("type")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| serde::de::Error::missing_field("type"))?;

                match type_str {
                    $(
                        $type_str => Ok(Series::$variant(
                            serde_json::from_value(value).map_err(serde::de::Error::custom)?
                        )),
                    )*
                    _ => Err(serde::de::Error::unknown_variant(
                        type_str,
                        &[$($type_str),*]
                    )),
                }
            }
        }
    };
}

impl_series_deserialize!(
    Bar => "bar",
    Bar3d => "bar3D",
    Boxplot => "boxplot",
    Candlestick => "candlestick",
    Custom => "custom",
    EffectScatter => "effectScatter",
    Funnel => "funnel",
    Gauge => "gauge",
    Graph => "graph",
    Heatmap => "heatmap",
    Line => "line",
    Map => "map",
    Parallel => "parallel",
    PictorialBar => "pictorialBar",
    Pie => "pie",
    Radar => "radar",
    Sankey => "sankey",
    Scatter => "scatter",
    Sunburst => "sunburst",
    ThemeRiver => "themeRiver",
    Tree => "tree",
    Treemap => "treemap",
);

impl Series {
    pub fn with_mutable<F>(&mut self, f: F) -> &mut Self
    where
        F: FnOnce(&mut SeriesController),
    {
        let mut controller = SeriesController { series: self };
        f(&mut controller);
        self
    }
}

pub struct SeriesController<'a> {
    series: &'a mut Series,
}

impl<'a> SeriesController<'a> {
    pub fn new(series: &'a mut Series) -> Self {
        SeriesController { series }
    }
    /// Type-safe access to Line variant
    pub fn as_line_mut(&mut self) -> Result<line::LineController<'_>, SeriesTypeError> {
        match self.series {
            Series::Line(ref mut line) => Ok(line::LineController::new(line)),
            _ => Err(SeriesTypeError::ExpectedLine),
        }
    }

    /// Type-safe access to Scatter variant
    pub fn as_scatter_mut(&mut self) -> Result<scatter::ScatterController<'_>, SeriesTypeError> {
        match self.series {
            Series::Scatter(ref mut scatter) => Ok(scatter::ScatterController::new(scatter)),
            _ => Err(SeriesTypeError::ExpectedScatter),
        }
    }

    //     /// Closure-based modification with automatic variant matching
    //     pub fn with_series<f, r>(&mut self, f: f) -> r
    //     where
    //         f: fnonce(&mut seriescontroller) -> r,
    //     {
    //         match self.series {
    //             series::line(ref mut line) => f(&mut linecontroller { line }),
    //             Series::Scatter(ref mut scatter) => f(&mut ScatterController { scatter }),
    //             _ => (),
    //         }
    //     }
}

// Trait for common operations
// trait SeriesVariantMutator {
//     fn set_name<S: Into<String>>(&mut self, name: S);
//     // Add common operations here
// }

#[derive(Debug, thiserror::Error)]
pub enum SeriesTypeError {
    #[error("Expected Line series variant")]
    ExpectedLine,
    #[error("Expected Scatter series variant")]
    ExpectedScatter,
}

// Think about how to get a mutable ref to the enum variants instead of
// implementing wrapper methods.
pub trait Getters {
    fn get_series_id(&self) -> Option<String>;
    fn get_series_name(&self) -> Option<String>;
    fn get_show_symbol(&self) -> Option<bool>;
    fn get_series_smoothness(&self) -> Option<Smoothness>;
    fn get_series_symbol(&self) -> &Option<Symbol>;
    fn get_series_linestyle(&self) -> Option<LineStyle>;
    fn get_series_data(&self) -> Option<&Vec<DataPoint>>;
}

pub trait Setters {
    fn set_show_symbol(&mut self, show_symbol: bool);
    fn set_smoothness<S: Into<Smoothness>>(&mut self, smoothness: S);
    fn set_symbol<S: Into<Symbol>>(&mut self, symbol: S);
    fn set_linestyle<L: Into<LineStyle>>(&mut self, line_style: L);
    fn set_series_data<D: Into<DataPoint>>(&mut self, new_data: Vec<D>);
}

impl Getters for Series {
    fn get_series_id(&self) -> Option<String> {
        match self {
            Series::Line(line) => line.get_id(),
            // Series::Scatter(scatter) => scatter.get_id(),
            _ => None,
        }
    }

    fn get_series_name(&self) -> Option<String> {
        match self {
            Series::Line(line) => line.get_name(),
            _ => None,
        }
    }

    fn get_show_symbol(&self) -> Option<bool> {
        match self {
            Series::Line(line) => line.get_show_symbol(),
            _ => None,
        }
    }

    fn get_series_smoothness(&self) -> Option<Smoothness> {
        match self {
            Series::Line(line) => line.get_smoothness(),
            _ => None,
        }
    }

    fn get_series_symbol(&self) -> &Option<Symbol> {
        match self {
            Series::Line(line) => line.get_symbol(),
            _ => &None,
        }
    }

    fn get_series_linestyle(&self) -> Option<LineStyle> {
        match self {
            Series::Line(line) => line.get_linestyle(),
            _ => None,
        }
    }

    fn get_series_data(&self) -> Option<&Vec<DataPoint>> {
        match self {
            Series::Line(line) => Some(line.get_data()),
            _ => None,
        }
    }
}

impl Setters for Series {
    fn set_series_data<D: Into<DataPoint>>(&mut self, new_data: Vec<D>) {
        match self {
            Series::Line(line) => line.set_data(new_data),
            _ => (),
        }
    }
    fn set_smoothness<S: Into<Smoothness>>(&mut self, smoothness: S) {
        match self {
            Series::Line(line) => line.set_smoothness(smoothness),
            _ => (),
        }
    }
    fn set_symbol<S: Into<Symbol>>(&mut self, symbol: S) {
        match self {
            Series::Line(line) => line.set_symbol(symbol),
            _ => (),
        }
    }
    fn set_linestyle<L: Into<LineStyle>>(&mut self, line_style: L) {
        match self {
            Series::Line(line) => line.set_linestyle(line_style),
            _ => (),
        }
    }
    fn set_show_symbol(&mut self, show_symbol: bool) {
        match self {
            Series::Line(line) => line.set_show_symbol(show_symbol),
            _ => (),
        }
    }
}

impl<'de> Deserialize<'de> for Series {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Deserialize into a struct that contains the "type" field
        let value = serde_json::Value::deserialize(deserializer)?;

        let result = match value.get("type").and_then(serde_json::Value::as_str) {
            Some(type_) => match type_ {
                "bar" => serde_json::from_value(value).map(Series::Bar),
                "line" => serde_json::from_value(value).map(Series::Line),
                // TODO: add remaining series types.
                unknown => Err(serde::de::Error::custom(format!(
                    "unknown series type: {}",
                    unknown
                ))),
            },
            None => Err(serde::de::Error::custom("missing series type")),
        };
        Ok(result.map_err(serde::de::Error::custom)?)
    }
}

macro_rules! impl_series {
    ($($variant:ident),*) => {
        impl Serialize for Series {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::ser::Serializer,
            {
                match self {
                    $(Self::$variant(series) => series.serialize(serializer),)*
                }
            }
        }
        $(
            impl From<$variant> for Series {
                fn from(series: $variant) -> Self {
                    Self::$variant(series)
                }
            }
        )*
    }
}

impl_series!(
    Bar,
    Bar3d,
    Boxplot,
    Custom,
    Candlestick,
    EffectScatter,
    Funnel,
    Gauge,
    Graph,
    Heatmap,
    Line,
    Map,
    Parallel,
    PictorialBar,
    Pie,
    Radar,
    Sankey,
    Scatter,
    Sunburst,
    ThemeRiver,
    Tree,
    Treemap
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip_series_line() {
        let data = vec![vec![0, 1], vec![2, 3]];
        let line = line::Line::new()
            .name("test_line")
            .show_symbol(false)
            .connect_nulls(true)
            .data(data);
        let line_json = serde_json::to_string(&line).unwrap();
        println!("{}", line_json);
        let line_de = serde_json::from_str(&line_json).unwrap();
        assert_eq!(line, line_de);
    }
}
