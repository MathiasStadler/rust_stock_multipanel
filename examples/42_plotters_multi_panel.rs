// FROM HERE
// https://plotters-rs.github.io/book/basic/multipanel.html

use plotters::prelude::full_palette::BLUE;
#[allow(unused_imports)]
use plotters::prelude::full_palette::GREEN;
#[allow(unused_imports)]
use plotters::prelude::full_palette::RED;
#[allow(unused_imports)]
use plotters::prelude::full_palette::YELLOW;
use plotters::style::full_palette::ORANGE;
use std::path::Path;

use plotters::backend::BitMapBackend;
use plotters::drawing::IntoDrawingArea;

use plotters::style::AsRelative;

fn main() {
    let this_file = file!();
    println!("filename: {}", this_file);

    let name_only = Path::new(this_file)
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap();

    let output_png = name_only.replace("rs", "png");

    let output_png_filename: String = format!("images/{output_png}");

    println!("target filename: {}", output_png_filename);

    let root: plotters::prelude::DrawingArea<BitMapBackend<'_>, plotters::coord::Shift> =
        
        BitMapBackend::new(&output_png_filename, (1920, 1080)).into_drawing_area();

    // split panel to left and right
    let (left, right) = root.split_horizontally((15_i32).percent_width());

    // give left side color
    left.fill(&YELLOW).unwrap();

    // split upper and lower panel in three parts
    let upper_areas = right.split_evenly((3, 1));

    // let (upper_one, upper_two) = right.split_vertically(2);

    // upper_one.fill(&RED).unwrap();

    // upper_one.relative_to_height(30.0);

    // let _left_title = upper_two.titled("This is the title", ("serif", 30));

    
    // upper_two.fill(&GREEN).unwrap();

    let chart_0 = upper_areas.get(0);

    // chart_1.fill(&GREEN).unwrap();
    chart_0.expect("REASON").fill(&GREEN).unwrap();

    let chart_1 = upper_areas.get(1);
    chart_1.expect("REASON").fill(&BLUE).unwrap();

    let chart_2 = upper_areas.get(2);
    chart_2.expect("REASON").fill(&ORANGE).unwrap();

    //for (id, area) in upper_areas.into_iter().enumerate() {
    //    area.fill(&Palette99::pick(id)).unwrap();
    //}

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

// cargo run --example 38_plotters_multi_panel
