use plotters::style::IntoFont;
use plotters::prelude::CandleStick;
// TODO switch to std::time
// use std::time::Duration;
use chrono::Duration;
use chrono::Local;
// use chrono::Date;
#[allow(unused_imports)]
use chrono::DateTime;
use chrono::TimeZone;

use plotters::backend::BitMapBackend;
use plotters::drawing::IntoDrawingArea;
use plotters::prelude::full_palette::BLACK;
#[allow(unused_imports)]
use plotters::prelude::full_palette::BLUE;
use plotters::prelude::full_palette::GREEN;
#[allow(unused_imports)]
use plotters::prelude::full_palette::GREY;
use plotters::prelude::full_palette::GREY_100;
// use plotters::prelude::full_palette::GREY_200;
use plotters::prelude::full_palette::GREY_300;
use plotters::prelude::full_palette::RED;
#[allow(unused_imports)]
use plotters::prelude::full_palette::YELLOW;
use plotters::prelude::ChartBuilder;
use plotters::prelude::LabelAreaPosition;
use plotters::prelude::LineSeries;
use plotters::prelude::Rectangle;
#[allow(unused_imports)]
use plotters::style::full_palette::ORANGE;
use plotters::style::AsRelative;
use plotters::style::Color;
use plotters::style::WHITE;
use std::path::Path;

//use chrono::NaiveDateTime;
#[allow(unused_imports)]
use plotters::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let this_file = file!();
    println!("filename: {}", this_file);

    let name_only = Path::new(this_file)
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap();

    let output_png = name_only.replace("rs", "png");

    let output_png_filename: String = format!("images/{output_png}");

    println!("target filename: {}", output_png_filename);

    let root: plotters::prelude::DrawingArea<
        BitMapBackend<'_>,
        plotters::coord::Shift
    > = BitMapBackend::new(&output_png_filename, (1920, 1080)).into_drawing_area();

    root.fill(&WHITE).unwrap();

    // split panel to left and right
    let (left, right) = root.split_horizontally((15_i32).percent_width());

    // give left side color
    left.fill(&WHITE).unwrap();

    // split upper and lower panel in three parts
    let upper_areas = right.split_evenly((4, 1));

    
    //start 
    let chart_0 = upper_areas.get(0);

    chart_0.expect("REASON").fill(&GREY_100).unwrap();

    // get value out of option
    let drawing_area_1 = match chart_0 {
        Some(x) => x,
        None => {
            return Ok(());
        }
    };

    
    let mut chart_1 = ChartBuilder::on(&drawing_area_1)
        .margin(50)
        .set_label_area_size(LabelAreaPosition::Top, 30)
        .set_label_area_size(LabelAreaPosition::Left, 30)
        .set_label_area_size(LabelAreaPosition::Right, 30)
        .set_label_area_size(LabelAreaPosition::Bottom, 30)
        .caption("Prime Distribution", ("sans-serif", 40))
        .build_cartesian_2d(0.0..10.0, -1.0..1.0)
        .unwrap();
    chart_1.configure_mesh().draw().unwrap();

    chart_1
        .draw_series(
            LineSeries::new(
                (0..100).map(|x| (x as f64) / 10.0).map(|x| (x, x.sin())),
                &BLACK
            )
        )
        .unwrap();

    chart_1
        .draw_series(
            LineSeries::new(
                (0..100).map(|x| (x as f64) / 5.0).map(|x| (x, x.cos())),
                &RED
            )
        )
        .unwrap();

    chart_1
        .draw_series(
            LineSeries::new(
                (0..100).map(|x| (x as f64) / 10.0).map(|x| (x, 0.2)),
                &RED
            )
        )
        .unwrap();

    chart_1
        .configure_series_labels()
        .position(SeriesLabelPosition::UpperMiddle)
        .background_style(&WHITE)
        .border_style(&BLACK)
        .draw()?;
    //END


    Ok(())
}

// cargo run --example
