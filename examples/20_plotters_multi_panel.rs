// example convert inside fn String to str

use chrono::ParseError;
// use chrono::naive::NaiveDateTime;
// use chrono::offset::{Local, TimeZone};
use chrono::NaiveDateTime;
use chrono::{Duration, NaiveTime};
// use plotters::prelude::*;

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

    println!("Start Date {:?}", data[0].0);
    // println!("End Date {:?}", data[-1].0);
    println!("End Date {:?}", data[data.len() - 1].0);

    let (to_date, from_date) = (
        parse_time(&data[0].0).unwrap() + Duration::days(1),
        parse_time(&data[data.len() - 1].0).unwrap() - Duration::days(1),
    );

    // FROM HERE
    // workspace_rust/rust_plotter_histo_multi_line examples/plotters_5_3_x_stock_example.rs
    let y_min = parse_time("9:30.0").unwrap();
    let y_max = parse_time("13:00.0").unwrap();

    println!("count data => {}", data.len());
    println!(" to_date => {:?}", to_date);
    println!("from_data =>{:?}", from_date);

    Ok(())
}



// 2021-01-05 00:00:00
fn parse_time(time_str: &String) -> Result<Duration, ParseError> {
    println!("time_str => {}", time_str);
    println!("time format => {}",format!("{} 00:00:00", time_str ));
   
    let return_date = match NaiveDateTime::parse_from_str(&format!("{} 00:00:00", time_str), "%Y-%m-%d %H:%M:%S%.f" ) {
        Ok(date) => Ok(date
            .time()
            .signed_duration_since(NaiveTime::from_hms_opt(0, 0, 0).expect("REASON"))),
        Err(e) => {
            println!("{}", e);
            Err(e)
        }
    };

    println!("return_data => {:?}",return_date);

    return return_date
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
