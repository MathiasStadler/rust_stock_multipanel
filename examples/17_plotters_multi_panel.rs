// example convert inside fn String to str

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
     

        return_vec.push((date, open, high, low, close, volume));
    }

    return_vec
}

// cargo run --example 16_plotters_multi_panel
