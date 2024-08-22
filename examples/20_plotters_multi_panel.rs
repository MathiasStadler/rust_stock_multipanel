// example convert inside fn String to str

use chrono::ParseError;
// use chrono::naive::NaiveDateTime;
// use chrono::offset::{Local, TimeZone};
use chrono::NaiveDateTime;
use chrono::{Duration, NaiveTime};
// use plotters::prelude::*;
use plotters::backend::BitMapBackend;
use plotters::drawing::IntoDrawingArea;
use plotters::prelude::ChartBuilder;
use plotters::prelude::WHITE;
use plotters::prelude::BLUE;
use plotters::style::IntoFont;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, panel!");

    //get data from csv
    let data: Vec<(String, f32, f32, f32, f32, f32)> = get_data_from_csv();

    // show data
    for data in data.iter() {
        match data.0 {
            _ => {
                println!(
                    "Stock Data {} {} {} {} {} {}",
                    data.0, data.1, data.2, data.3, data.4, data.5
                )
            }
        }
    } // end for

    let root = BitMapBackend::new("images/20_stock_example.png", (1024, 768)).into_drawing_area();
    root.fill(&WHITE)?;

    println!("Start Date {:?}", data[0].0);
    // println!("End Date {:?}", data[-1].0);
    println!("End Date {:?}", data[data.len() - 1].0);

    let (to_date, from_date) = (
        parse_date(&data[0].0).unwrap() + Duration::days(1),
        parse_date(&data[data.len() - 1].0).unwrap() - Duration::days(1),
    );

    // FROM HERE
    // workspace_rust/rust_plotter_histo_multi_line examples/plotters_5_3_x_stock_example.rs
    let y_min = parse_time(&String::from("9:30:0")).unwrap();
    let y_max = parse_time(&String::from("13:00:0")).unwrap();

    println!("count data => {}", data.len());
    println!(" to_date => {:?}", to_date);
    println!("from_data =>{:?}", from_date);

    let _chart = ChartBuilder::on(&root)
        .x_label_area_size(40)
        .y_label_area_size(50)
        .caption("Time", ("sans-serif", 30.0).into_font())
        .build_cartesian_2d(from_date..to_date, y_min..y_max)?;

    chart
        .configure_mesh()
        .light_line_style(&WHITE)
        .y_label_formatter(&|y| format!("{:02}:{:02}", y.num_minutes(), y.num_seconds() % 60))
        .x_label_formatter(&|x| x.naive_local().to_string())
        .draw()?;

    chart.draw_series(data.iter().map(|x| {
        Circle::new(
            (parse_datetime(x.0), parse_time(x.1).unwrap()),
            5,
            BLUE.filled(),
        )
    }))?;

    Ok(())
}

// 2021-01-05 00:00:00
fn parse_date(time_str: &String) -> Result<Duration, ParseError> {
    println!("parse_date_str => {}", time_str);
    println!("parse_date_format => {}", format!("{} 00:00:00", time_str));

    let return_date = match NaiveDateTime::parse_from_str(
        &format!("{} 00:00:00", time_str),
        "%Y-%m-%d %H:%M:%S%.f",
    ) {
        Ok(date) => Ok(date
            .time()
            .signed_duration_since(NaiveTime::from_hms_opt(0, 0, 0).expect("REASON"))),
        Err(e) => {
            println!("{}", e);
            Err(e)
        }
    };

    println!("parse_date return_data => {:?}", return_date);

    return return_date;
}

// 2021-01-05 00:00:00
fn parse_time(time_str: &String) -> Result<Duration, ParseError> {
    println!("parse_time_str => {}", time_str);
    println!("parse_time format => {}", format!("{} 00:00:00", time_str));

    let return_date = match NaiveDateTime::parse_from_str(
        &format!("1999-01-01 {}", time_str),
        "%Y-%m-%d %H:%M:%S%.f",
    ) {
        Ok(date) => Ok(date
            .time()
            .signed_duration_since(NaiveTime::from_hms_opt(0, 0, 0).expect("REASON"))),
        Err(e) => {
            println!("parse_time return_data=> {}", e);
            Err(e)
        }
    };

    println!("return_data => {:?}", return_date);

    return return_date;
}

fn parse_datetime(t: &str) -> Date<Local> {
    Local
        .datetime_from_str(&format!("{} 0:0", t), "%Y-%m-%d %H:%M")
        .unwrap()
        .date()
}

fn get_data_from_csv() -> Vec<(String, f32, f32, f32, f32, f32)> {
    let mut reader = csv::Reader::from_path("stock_data/stock_trex_data.csv").unwrap();

    let mut return_vec: Vec<(String, f32, f32, f32, f32, f32)> = Vec::new();

    for record in reader.deserialize() {
        let (date, open, high, low, close, volume): (String, f32, f32, f32, f32, f32) =
            record.unwrap();

        // let _dt = DataItem::builder()
        //     .open(open)
        //     .high(high)
        //     .low(low)
        //     .close(close)
        //     .volume(volume)
        //     .build()
        //     .unwrap();

        // let sma_7_val = sma_7.next(&dt);

        // String to &str
        //let date_clone:&String = &date.clone();

        // let date_str:&str = &date_clone.as_str();
        // let date_str: &str = date.clone().as_str();

        return_vec.push((date, open, high, low, close, volume));
    }

    return_vec
}

// cargo run --example 20_plotters_multi_panel
