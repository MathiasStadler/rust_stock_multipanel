// FROM HERE
// https://stackoverflow.com/questions/65937447/how-to-plot-a-series-with-a-date-on-the-x-axis-and-time-on-the-y-axis

#[allow(unused_imports)]
use plotters::style::IntoFont;
// use plotters::prelude::CandleStick;
// TODO switch to std::time
// use std::time::Duration;
#[allow(unused_imports)]
use chrono::Duration;
#[allow(unused_imports)]
use chrono::Local;
// use chrono::Date;
#[allow(unused_imports)]
use chrono::DateTime;
#[allow(unused_imports)]
use chrono::TimeZone;
#[allow(unused_imports)]
use plotters::backend::BitMapBackend;
#[allow(unused_imports)]
use plotters::drawing::IntoDrawingArea;
#[allow(unused_imports)]
use plotters::prelude::full_palette::BLACK;
#[allow(unused_imports)]
use plotters::prelude::full_palette::BLUE;
// use plotters::prelude::full_palette::GREEN;
#[allow(unused_imports)]
use plotters::prelude::full_palette::GREY;
#[allow(unused_imports)]
use plotters::prelude::full_palette::GREY_100;
// use plotters::prelude::full_palette::GREY_200;
// use plotters::prelude::full_palette::GREY_300;
#[allow(unused_imports)]
use plotters::prelude::full_palette::RED;
#[allow(unused_imports)]
use plotters::prelude::full_palette::YELLOW;
#[allow(unused_imports)]
use plotters::prelude::ChartBuilder;
// use plotters::prelude::LabelAreaPosition;
#[allow(unused_imports)]
use plotters::prelude::LineSeries;
// use plotters::prelude::Rectangle;
#[allow(unused_imports)]
use plotters::style::full_palette::ORANGE;
#[allow(unused_imports)]
use plotters::style::AsRelative;
#[allow(unused_imports)]
use plotters::style::Color;
#[allow(unused_imports)]
use plotters::style::WHITE;
#[allow(unused_imports)]
use std::path::Path;

//use chrono::NaiveDateTime;
#[allow(unused_imports)]
use plotters::prelude::*;
#[allow(unused_imports)]
use chrono::ParseError;
#[allow(unused_imports)]
#[allow(deprecated)]
use chrono::Date;
#[allow(unused_imports)]
use chrono::NaiveTime;

#[allow(deprecated)]
fn parse_datetime(t: &str) -> Date<Local> {
    #[allow(deprecated)]
    Local.datetime_from_str(&format!("{} 0:0", t), "%Y-%m-%d %H:%M").unwrap().date()
}

fn parse_time(t: &str) -> Result<Duration, ParseError> {
    return match Local.datetime_from_str(&format!("2020-01-01 0:{}", t), "%Y-%m-%d %H:%M:%S%.f") {
        #[allow(deprecated)]
        Ok(date) => Ok(date.time().signed_duration_since(NaiveTime::from_hms(0, 0, 0))),
        Err(e) => {
            println!("{}", e);
            Err(e)
        }
    };
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

    let data = get_data();
    // let root = BitMapBackend::new("stock-example.png", (1024, 768)).into_drawing_area();
    drawing_area_1.fill(&WHITE)?;

    let (from_date, to_date) = (
        parse_datetime(&data[0].0) - Duration::days(1),
        parse_datetime(&data[data.len() - 1].0) + Duration::days(1),
    );

    let y_min = parse_time("9:30.0").unwrap();
    let y_max = parse_time("13:00.0").unwrap();

    // let mut chart_1 = ChartBuilder::on(&drawing_area_1)
    //     .margin(50)
    //     .set_label_area_size(LabelAreaPosition::Top, 30)
    //     .set_label_area_size(LabelAreaPosition::Left, 30)
    //     .set_label_area_size(LabelAreaPosition::Right, 30)
    //     .set_label_area_size(LabelAreaPosition::Bottom, 30)
    //     .caption("Prime Distribution", ("sans-serif", 40))
    //     .build_cartesian_2d(0.0..10.0, -1.0..1.0)
    //     .unwrap();

    let mut chart_1 = ChartBuilder::on(&root)
        .x_label_area_size(40)
        .y_label_area_size(50)
        .caption("Time", ("sans-serif", 30.0).into_font())
        .build_cartesian_2d(from_date..to_date, y_min..y_max)
        .unwrap();

    

     chart_1.configure_mesh().draw().unwrap();

    // chart_1
    //     .draw_series(
    //         LineSeries::new(
    //             (0..100).map(|x| (x as f64) / 10.0).map(|x| (x, x.sin())),
    //             &BLACK
    //         )
    //     )
    //     .unwrap();

    chart_1.draw_series(
        data
            .iter()
            .map(|x| Circle::new((parse_datetime(x.0), parse_time(x.1).unwrap()), 5, BLUE.filled()))
    )?;


    // chart_1
    //     .draw_series(
    //         LineSeries::new(
    //             (0..100).map(|x| (x as f64) / 5.0).map(|x| (x, x.cos())),
    //             &RED
    //         )
    //     )
    //     .unwrap();

    // chart_1
    //     .draw_series(
    //         LineSeries::new(
    //             (0..100).map(|x| (x as f64) / 10.0).map(|x| (x, 0.2)),
    //             &RED
    //         )
    //     )
    //     .unwrap();

    // chart_1
    //     .configure_series_labels()
    //     .position(SeriesLabelPosition::UpperMiddle)
    //     .background_style(&WHITE)
    //     .border_style(&BLACK)
    //     .draw()?;
    //END

    

    Ok(())
}

fn get_data() -> Vec<(&'static str, &'static str, f32, f32, f32)> {
    return vec![
        ("2019-04-18", "10:11.5", 16.0, 121.3018, 123.37),
        ("2019-04-22", "10:52.2", 15.0, 122.57, 123.76),
        ("2019-04-23", "12:23.5", 14.0, 123.83, 125.44),
        ("2019-04-24", "10:15.0", 13.0, 124.52, 125.01),
        ("2019-04-25", "10:43.9", 12.0, 128.83, 129.15)
    ];
}

// cargo run --example
