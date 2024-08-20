// just for demonstration

fn first() {
    println!("Hello, 12_plotters_multi_panel.rs!");
    //explore vec
    let mut return_vec: Vec<(&'static str, f32, f32, f32, f32 )> = Vec::new();

    return_vec.push(("date", 1.0, 2.0, 3.0, 4.0));

    println!("pop => {:?}", return_vec.pop());
}

fn second() {

    // ("2019-04-05", 119.39, 120.23, 119.37, 119.89),
    // ("2019-04-04", 120.1, 120.23, 118.38, 119.36),
    let mut return_vec: Vec<(&'static str, f32, f32, f32, f32)> = Vec::new();

    println!("pop => {:?}", return_vec.pop());


}

fn main() {
    first();
    second();
}
