// example convert inside fn String to str

// use chrono::naive::NaiveDateTime;
// use chrono::offset::{Local, TimeZone};
// use chrono::{Duration, NaiveDate, NaiveTime};
// use plotters::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, panel!");

    //get data from csv
    let _data: Vec<(String, f32, f32, f32, f32, f32)> = get_data_from_csv();

    // show data
    // for data in data.iter() {
    //     match data.0 {
    //         _ => {
    //             println!(
    //                 "Stock Data {} {} {} {} {} {}",
    //                 data.0, data.1, data.2, data.3, data.4, data.5
    //             )
    //         }
    //     }
    // }

    // let (to_date, from_date) = (
    //     parse_time(data[0].0) + Duration::days(1),
    //     parse_time(data[29].0) - Duration::days(1),
    // );

    // println!("len data => {}", data.len());
    // println!(" to_date => {}", to_date);
    // println!("from_data =>{}", from_date);

    Ok(())
}


//fn get_data_from_csv(
//    mut return_vec: Vec<(String, f32, f32, f32, f32, f32)>,
//) 

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

        // println!("pop => {:?}", return_vec.pop());

        // println!(
        //     " {:?}, {:?}, {:?}, {:?},{:?}, {:?}",
        //     date, open, high, low, close, volume
        // );
    }

    return_vec
}

// cargo run --example 16_plotters_multi_panel
