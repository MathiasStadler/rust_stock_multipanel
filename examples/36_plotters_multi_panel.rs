// FROM HERE
// https://plotters-rs.github.io/book/basic/multipanel.html

use std::path::Path;

use plotters::prelude::full_palette::YELLOW;
// use plotters::prelude::*;
use plotters::backend::BitMapBackend;
use plotters::drawing::IntoDrawingArea;
use plotters::prelude::Palette99;
use plotters::style::AsRelative;
use plotters::style::Palette;
// use plotters::prelude::full_palette::ORANGE;
fn main() {
    let this_file = file!();
    println!("filename: {}", this_file);

    let name_only = Path::new(this_file)
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap();
    println!("filename only: {}", name_only);

    println!("{}", name_only.replace("rs", "png"));

    #[allow(unused_variables)]
    let output_png = name_only.replace("rs", "png");

    let output_png_filename:String = format!("images/{output_png}");

    println!("target filename: {}", output_png_filename);

    let root =
        // BitMapBackend::new("images/36_split-drawing-area.png", (640, 480)).into_drawing_area();
        BitMapBackend::new(&output_png_filename, (640, 480)).into_drawing_area();
        

    // We can split to left panel and right panel 15% left, 85% right
    let (left, right) = root.split_horizontally((15_i32).percent_width());

    left.fill(&YELLOW).unwrap();

    // We can also split upper and lower panel
    // let upper_areas = right.split_evenly((10, 1));
    let (upper_one, upper_two) = right.split_vertically(2);

    upper_one.fill(&Palette99::pick(1)).unwrap();

    upper_one.relative_to_height(30.0);
    let _ = upper_two.titled("This is the title", ("serif", 30));

    upper_two.fill(&Palette99::pick(5)).unwrap();

    //  for (id, area) in upper_areas.into_iter().enumerate() {
    //      area.fill(&Palette99::pick(id)).unwrap();
    //  }

    // for (id, area) in lower_areas.into_iter().enumerate() {
    //     area.fill(&Palette99::pick(id)).unwrap();
    // }

    // upper_1.fill(&GREEN).unwrap();

    // Then we can split the lower area evenly to 3 row 2 col
    // let lower_areas = lower.split_evenly((2, 3));

    // for (id, area) in lower_areas.into_iter().enumerate() {
    //     area.fill(&Palette99::pick(id)).unwrap();
    // }
}

// cargo run --example

// cargo run --example 35_plotters_multi_panel
