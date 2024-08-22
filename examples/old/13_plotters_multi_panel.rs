use ta::DataItem;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, panel!");

    //get data from csv
    let stock_data: Vec<(String, f64, f64, f64, f64, f64)> = get_data_from_csv();

    // show data
    for data in stock_data.iter() {
        match data.0 {
            // TODO ^ Try deleting the & and matching just "Ferris"
            _ => println!(
                "Stock Data {} {} {} {} {} {}",
                data.0, data.1, data.2, data.3, data.4, data.5
            ),
        }
    }

    // println!("{:?}",stock_data);
    Ok(())
}

fn get_data_from_csv() -> Vec<(String, f64, f64, f64, f64, f64)> {
    let mut reader = csv::Reader::from_path("stock_data/stock_trex_data.csv").unwrap();

    let mut return_vec: Vec<(String, f64, f64, f64, f64, f64)> = Vec::new();

    for record in reader.deserialize() {
        let (date, open, high, low, close, volume): (String, f64, f64, f64, f64, f64) =
            record.unwrap();

        let dt = DataItem::builder()
            .open(open)
            .high(high)
            .low(low)
            .close(close)
            .volume(volume)
            .build()
            .unwrap();

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
