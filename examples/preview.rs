use std::path::PathBuf;
use veusz::data::Data;
use veusz::export::SvgExport;
use veusz::page::{Axis, Graph, Page, Xy};
use veusz::style::line::LineStyle;
use veusz::style::marker::Marker;
use veusz::style::plot::PlotLine;
use veusz::style::ColorName;
use veusz::Veusz;

const PI2: u32 = 629;
const SCALE: f64 = 100.0;

fn main() {
    Veusz::default()
        .with_page(
            Page::default().with(
                Graph::default()
                    .with_axis(Axis::x("x"))
                    .with_axis(Axis::y("y"))
                    .with_xy(
                        Xy::data("x-data", "y-sin")
                            .with_marker(Marker::None)
                            .with_plot_line(
                                PlotLine::default()
                                    .with_color_by_name(ColorName::Blue)
                                    .with_style(LineStyle::DashDotDot)
                                    .with_width(3.0),
                            ),
                    )
                    .with_xy(
                        Xy::data("x-data", "y-cos")
                            .with_marker(Marker::None)
                            .with_plot_line(
                                PlotLine::default()
                                    .with_color_by_name(ColorName::Red)
                                    .with_style(LineStyle::Solid)
                                    .with_width(3.0),
                            ),
                    ),
            ),
        )
        .with_data(Data::new("x-data", (0..PI2).map(|x| x as f64 / SCALE)))
        .with_data(Data::new(
            "y-sin",
            (0..PI2).map(|x| (x as f64 / SCALE).sin()),
        ))
        .with_data(Data::new(
            "y-cos",
            (0..PI2).map(|x| (x as f64 / SCALE).cos()),
        ))
        .with_export(
            SvgExport::target(
                PathBuf::from(file!())
                    .with_extension("svg")
                    .to_str()
                    .unwrap(),
            )
            .with_color(true),
        )
        .open();
}
