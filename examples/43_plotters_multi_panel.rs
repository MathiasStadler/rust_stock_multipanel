// FROM HERE
// https://plotters-rs.github.io/book/basic/multipanel.html

use plotters::prelude::full_palette::BLUE;
#[allow(unused_imports)]
use plotters::prelude::full_palette::GREEN;
#[allow(unused_imports)]
use plotters::prelude::full_palette::RED;
#[allow(unused_imports)]
use plotters::prelude::full_palette::YELLOW;
use plotters::prelude::ChartBuilder;
use plotters::style::full_palette::ORANGE;
use plotters::style::WHITE;
use plotters::prelude::full_palette::GREY;
use plotters::prelude::full_palette::GREY_100;
use plotters::prelude::full_palette::GREY_200;
use plotters::prelude::full_palette::GREY_300;
use std::path::Path;

use plotters::backend::BitMapBackend;
use plotters::drawing::IntoDrawingArea;

use plotters::style::AsRelative;

use plotters::prelude::LabelAreaPosition;

use plotters::style::Color;
use plotters::prelude::Rectangle;

fn main() {
    let this_file = file!();
    println!("filename: {}", this_file);

    let name_only = Path::new(this_file)
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap();

    let output_png = name_only.replace("rs", "png");

    let output_png_filename: String = format!("images/{output_png}");

    println!("target filename: {}", output_png_filename);

    let root: plotters::prelude::DrawingArea<BitMapBackend<'_>, plotters::coord::Shift> =
        BitMapBackend::new(&output_png_filename, (1920, 1080)).into_drawing_area();

        root.fill(&WHITE).unwrap();

    // split panel to left and right
    let (left, right) = root.split_horizontally((15_i32).percent_width());

    // give left side color
    left.fill(&WHITE).unwrap();

    // split upper and lower panel in three parts
    let upper_areas = right.split_evenly((3, 1));

    let chart_0 = upper_areas.get(0);

    chart_0.expect("REASON").fill(&GREY_100).unwrap();

    // get value out of option
    let drawing_area = match chart_0 {
        Some(x) => x,
        None => return (),
    };

    //https://plotters-rs.github.io/book/basic/multipanel.html
    let mut lower_chart = ChartBuilder::on(drawing_area)
        .set_label_area_size(LabelAreaPosition::Left, 30)
        .set_label_area_size(LabelAreaPosition::Right, 30)
        .build_cartesian_2d(0.0..10.0, -1.0..1.0)
        .unwrap();

    lower_chart.configure_mesh().draw().unwrap();

    lower_chart
        .draw_series((0..100).map(|x| x as f64 / 10.0).map(|x| {
            let color = if x.cos() > 0.0 {
                RED.mix(0.3).filled()
            } else {
                GREEN.mix(0.3).filled()
            };
            Rectangle::new([(x, 0.0), (x + 0.1, x.cos())], color)
        }))
        .unwrap();

        // chart_2
    let chart_2 = upper_areas.get(1);
    chart_2.expect("REASON").fill(&GREY_300).unwrap();

    // get value out of option
    let drawing_area_2 = match chart_2 {
        Some(x) => x,
        None => return (),
    };

    //https://plotters-rs.github.io/book/basic/multipanel.html
    let mut lower_chart_2 = ChartBuilder::on(drawing_area_2)
        .set_label_area_size(LabelAreaPosition::Left, 30)
        .set_label_area_size(LabelAreaPosition::Right, 30)
        .build_cartesian_2d(0.0..10.0, -1.0..1.0)
        .unwrap();

    lower_chart_2.configure_mesh().draw().unwrap();

    lower_chart_2
        .draw_series((0..100).map(|x| x as f64 / 10.0).map(|x| {
            let color = if x.cos() > 0.0 {
                RED.mix(0.3).filled()
            } else {
                GREEN.mix(0.3).filled()
            };
            Rectangle::new([(x, 0.0), (x + 0.1, x.cos())], color)
        }))
        .unwrap();



    let chart_3 = upper_areas.get(2);
    chart_3.expect("REASON").fill(&GREY_200).unwrap();
}

// cargo run --example
// cargo run --example 42_plotters_multi_panel
