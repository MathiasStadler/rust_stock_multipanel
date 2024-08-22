

use chrono::naive::NaiveDateTime;
use chrono::offset::{Local, TimeZone};
use chrono::{Duration, NaiveDate, NaiveTime};
use plotters::prelude::*;


const OUT_FILE_NAME: &str = "plotters-doc-data/14_1_stock.png";


fn parse_time(date: &str) -> NaiveDate {

    // "2019-04-25"
        let _d = NaiveDate::parse_from_str(date, "%Y-%m-%d").unwrap();
    
        // let _d = NaiveDate::from_ymd_opt(_date).unwrap();
        let _time = NaiveTime::from_hms_milli_opt(0,0,0,0).unwrap();
        let _dt = NaiveDateTime::new(_d, _time);
        Local
            //.from_local_datetime(&format!("{} 0:0", _dt))
            .from_local_datetime(&_dt)        //DateTime::parse_from_str
            //NaiveDateTime::parse_from_str
            // .parse_from_str(&format!("{} 0:0", t), "%Y-%m-%d %H:%M")
            // .datetime_from_str(&format!("{} 0:0", t), "%Y-%m-%d %H:%M")
            .unwrap()
            .date_naive()
    }

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
    }

    let root = BitMapBackend::new(OUT_FILE_NAME, (1024, 768)).into_drawing_area();
    root.fill(&WHITE)?;

    let (to_date, from_date) = (
        parse_time(data[0].0.as_str()) + Duration::days(1),
        parse_time(data[29].0.as_str()) - Duration::days(1),
    );

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .caption("MSFT Stock Price", ("sans-serif", 50.0).into_font())
        .build_cartesian_2d(from_date..to_date, 110f32..135f32)?;

    chart.configure_mesh().light_line_style(WHITE).draw()?;

    chart.draw_series(
        data.iter().map(|x| {
            CandleStick::new(parse_time(x.0.as_str()), x.1, x.2, x.3, x.4, GREEN.filled(), RED, 15)
        }),
    )?;

    // To avoid the IO failure being ignored silently, we manually call the present function
    root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
    println!("Result has been saved to {}", OUT_FILE_NAME);

    // println!("{:?}",stock_data);
    Ok(())
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

        return_vec.push((date.clone(), open, high, low, close, volume));

        // println!("pop => {:?}", return_vec.pop());

        // println!(
        //     " {:?}, {:?}, {:?}, {:?},{:?}, {:?}",
        //     date, open, high, low, close, volume
        // );
    }

    return_vec
}

// cargo run --example 13_plotters_multi_panel
