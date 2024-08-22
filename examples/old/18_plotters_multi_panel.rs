// FROM HERE
// https://users.rust-lang.org/t/simple-function-returning-a-vec/52404

// __- temporary value created here

// example convert inside fn String to str

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, panel!");

    //get data from csv
    //let _data_string: Vec<(String, f32, f32, f32, f32, f32)> = get_data_from_csv_string();

    let mut reader = csv::Reader::from_path("stock_data/stock_trex_data.csv").unwrap();

    let mut data: Vec<(String, f32, f32, f32, f32, f32)> = Vec::new();

    for record in reader.deserialize() {
        let (date, open, high, low, close, volume): (String, f32, f32, f32, f32, f32) =
            record.unwrap();
     

        data.push((date, open, high, low, close, volume));
    }

    // show data
    for data in _data_string.iter() {
        match data.0 {
            _ => {
                println!(
                    "Stock Data(string) {} {} {} {} {} {}",
                    data.0, data.1, data.2, data.3, data.4, data.5
                )
            }
        }
    }

    //
    //get data from csv
    let _data_str: Vec<(String, f32, f32, f32, f32, f32)> = get_data_from_csv_str();

    // show data
    for data in _data_str.iter() {
        match data.0 {
            _ => {
                println!(
                    "Stock Data(str) {} {} {} {} {} {}",
                    data.0, data.1, data.2, data.3, data.4, data.5
                )
            }
        }
    }

    Ok(())
}

//fn get_data_from_csv(
//    mut return_vec: Vec<(String, f32, f32, f32, f32, f32)>,
//)

fn get_data_from_csv_string() -> Vec<(String, f32, f32, f32, f32, f32)> {
    

    return_vec
}

fn get_data_from_csv_str() -> Vec<(&'static str, f32, f32, f32, f32, f32)> {
    let mut reader = csv::Reader::from_path("stock_data/stock_trex_data.csv").unwrap();

    let mut return_vec: Vec<(&'static str, f32, f32, f32, f32, f32)> = Vec::new();

    for record in reader.deserialize() {
        let (date, open, high, low, close, volume): (String, f32, f32, f32, f32, f32) =
            record.unwrap();
     

        return_vec.push(( &date.as_str(), open, high, low, close, volume));
    }

    return_vec
}

// cargo run --example 16_plotters_multi_panel

