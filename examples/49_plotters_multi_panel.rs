// FROM HERE
// https://plotters-rs.github.io/book/basic/multipanel.html

use plotters::style::IntoFont;
use plotters::prelude::CandleStick;
// TODO switch to std::time
// use std::time::Duration;
use chrono::Duration;
use chrono::Local;
use chrono::Date;
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

#[allow(deprecated)]
fn parse_time(t: &str) -> Date<Local> {
    #[allow(deprecated)]
    Local.datetime_from_str(&format!("{} 0:0", t), "%Y-%m-%d %H:%M").unwrap().date()
}

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

    let chart_0 = upper_areas.get(0);

    chart_0.expect("REASON").fill(&GREY_100).unwrap();

    // get value out of option
    let drawing_area_1 = match chart_0 {
        Some(x) => x,
        None => {
            return Ok(());
        }
    };

    //start
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
    //end

    // chart_2
    let chart_2 = upper_areas.get(1);
    chart_2.expect("REASON").fill(&GREY_300).unwrap();

    // get value out of option
    let drawing_area_2 = match chart_2 {
        Some(x) => x,
        None => {
            return Ok(());
        }
    };

    //https://plotters-rs.github.io/book/basic/multipanel.html
    let mut lower_chart_2 = ChartBuilder::on(drawing_area_2)
        .margin(50)
        // .margin_bottom(50)
        .set_label_area_size(LabelAreaPosition::Top, 30)
        .set_label_area_size(LabelAreaPosition::Left, 30)
        .set_label_area_size(LabelAreaPosition::Right, 30)
        .set_label_area_size(LabelAreaPosition::Bottom, 30)
        .caption("Second Distribution", ("sans-serif", 40))
        .build_cartesian_2d(0.0..10.0, -1.0..1.0)
        .unwrap();

    lower_chart_2.configure_mesh().draw().unwrap();

    lower_chart_2
        .draw_series(
            (0..100)
                .map(|x| (x as f64) / 10.0)
                .map(|x| {
                    let color = if x.cos() > 0.0 {
                        RED.mix(0.3).filled()
                    } else {
                        GREEN.mix(0.3).filled()
                    };
                    Rectangle::new(
                        [
                            (x, 0.0),
                            (x + 0.1, x.cos()),
                        ],
                        color
                    )
                })
        )
        .unwrap();

    // chart_3 start

    let chart_3 = upper_areas.get(2);
    chart_3.expect("REASON").fill(&GREY_300).unwrap();

    // get value out of option
    let drawing_area_3 = match chart_3 {
        Some(x) => x,
        None => {
            return Ok(());
        }
    };

    let data = get_data();

    let (to_date, from_date) = (
        parse_time(data[0].0) + Duration::days(1),
        parse_time(data[29].0) - Duration::days(1),
    );

    let mut chart_3 = ChartBuilder::on(&drawing_area_3)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .caption("MSFT Stock Price Second", ("sans-serif", 50.0).into_font())
        .build_cartesian_2d(from_date..to_date, 110f32..135f32)?;

    chart_3.configure_mesh().light_line_style(WHITE).draw()?;

    chart_3.draw_series(
        data
            .iter()
            .map(|x| {
                CandleStick::new(
                    parse_time(x.0),
                    x.1,
                    x.2,
                    x.3,
                    x.4,
                    GREEN.filled(),
                    RED.filled(),
                    15
                )
            })
    )?;

    // chart_4
    let areas_4 = upper_areas.get(3);
    areas_4.expect("REASON").fill(&GREY_300).unwrap();


    // 
    // get value out of option
    let drawing_area_4 = match areas_4 {
        Some(x) => x,
        None => {
            return Ok(());
        }
    };


    //https://plotters-rs.github.io/book/basic/multipanel.html
    let mut chart_4 = ChartBuilder::on(drawing_area_4)
        .margin(50)
        // .margin_bottom(50)
        .set_label_area_size(LabelAreaPosition::Top, 30)
        .set_label_area_size(LabelAreaPosition::Left, 30)
        .set_label_area_size(LabelAreaPosition::Right, 30)
        .set_label_area_size(LabelAreaPosition::Bottom, 30)
        .caption("Four Distribution", ("sans-serif", 40))
        .build_cartesian_2d(0.0..10.0, -1.0..1.0)
        .unwrap();

    chart_4.configure_mesh().draw().unwrap();

    chart_4
        .draw_series(
            LineSeries::new(
                (0..100).map(|x| (x as f64) / 10.0).map(|x| (x, x.sin())),
                &BLACK
            )
        )
        .unwrap();

    // chart_3.draw_series(
    //     LineSeries::new(
    //     data
    //     .iter()
    //     .map(|x|{ }),&BLUE))
    //     .unwrap();

    // chart_3.draw_series(
    //     LineSeries::new(
    //         (0..100).map(|x| (x as f64) / 5.0)
    //         .map(|x| (x, x.cos())),
    //         &RED
    //    )
    //)

    Ok(())
}

fn get_data() -> Vec<(&'static str, f32, f32, f32, f32)> {
    vec![
        ("2019-04-25", 130.06, 131.37, 128.83, 129.15),
        ("2019-04-24", 125.79, 125.85, 124.52, 125.01),
        ("2019-04-23", 124.1, 125.58, 123.83, 125.44),
        ("2019-04-22", 122.62, 124.0, 122.57, 123.76),
        ("2019-04-18", 122.19, 123.52, 121.3018, 123.37),
        ("2019-04-17", 121.24, 121.85, 120.54, 121.77),
        ("2019-04-16", 121.64, 121.65, 120.1, 120.77),
        ("2019-04-15", 120.94, 121.58, 120.57, 121.05),
        ("2019-04-12", 120.64, 120.98, 120.37, 120.95),
        ("2019-04-11", 120.54, 120.85, 119.92, 120.33),
        ("2019-04-10", 119.76, 120.35, 119.54, 120.19),
        ("2019-04-09", 118.63, 119.54, 118.58, 119.28),
        ("2019-04-08", 119.81, 120.02, 118.64, 119.93),
        ("2019-04-05", 119.39, 120.23, 119.37, 119.89),
        ("2019-04-04", 120.1, 120.23, 118.38, 119.36),
        ("2019-04-03", 119.86, 120.43, 119.15, 119.97),
        ("2019-04-02", 119.06, 119.48, 118.52, 119.19),
        ("2019-04-01", 118.95, 119.1085, 118.1, 119.02),
        ("2019-03-29", 118.07, 118.32, 116.96, 117.94),
        ("2019-03-28", 117.44, 117.58, 116.13, 116.93),
        ("2019-03-27", 117.875, 118.21, 115.5215, 116.77),
        ("2019-03-26", 118.62, 118.705, 116.85, 117.91),
        ("2019-03-25", 116.56, 118.01, 116.3224, 117.66),
        ("2019-03-22", 119.5, 119.59, 117.04, 117.05),
        ("2019-03-21", 117.135, 120.82, 117.09, 120.22),
        ("2019-03-20", 117.39, 118.75, 116.71, 117.52),
        ("2019-03-19", 118.09, 118.44, 116.99, 117.65),
        ("2019-03-18", 116.17, 117.61, 116.05, 117.57),
        ("2019-03-15", 115.34, 117.25, 114.59, 115.91),
        ("2019-03-14", 114.54, 115.2, 114.33, 114.59)
    ]
}

// cargo run --example
// cargo run --example 49_plotters_multi_panel
