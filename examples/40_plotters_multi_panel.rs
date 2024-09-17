// FROM HERE
// https://plotters-rs.github.io/book/basic/multipanel.html

#[allow(unused_imports)]
use plotters::prelude::full_palette::GREEN;
use plotters::prelude::full_palette::RED;
#[allow(unused_imports)]
use plotters::prelude::full_palette::YELLOW;
use plotters::style::BLUE;
use std::path::Path;
// use plotters::prelude::*;
use plotters::backend::BitMapBackend;
use plotters::drawing::IntoDrawingArea;
use plotters::prelude::Palette99;
use plotters::style::AsRelative;
use plotters::style::Palette;
#[allow(unused_imports)]
use plotters::prelude::ChartBuilder;
#[allow(unused_imports)]
use plotters::prelude::IntoSegmentedCoord;
// use plotters::prelude::full_palette::ORANGE;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let this_file = file!();
    println!("filename: {}", this_file);

    let name_only = Path::new(this_file)
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap();

    // println!("filename only: {}", name_only);

    // println!("{}", name_only.replace("rs", "png"));

    #[allow(unused_variables)]
    let output_png = name_only.replace("rs", "png");

    let output_png_filename: String = format!("images/{output_png}");

    println!("target filename: {}", output_png_filename);

    let root: plotters::prelude::DrawingArea<BitMapBackend<'_>, plotters::coord::Shift> =
        // BitMapBackend::new("images/36_split-drawing-area.png", (640, 480)).into_drawing_area();
        BitMapBackend::new(&output_png_filename, (1920, 1080)).into_drawing_area();

    // We can split to left panel and right panel 15% left, 85% right

    // left side
    let (left, right) = root.split_horizontally((15_i32).percent_width());
    left.fill(&RED).unwrap();

    // We can also split upper and lower panel
    let upper_areas = right.split_evenly((3, 1));

    let (upper_one, upper_two) = right.split_vertically(2);

    upper_one.fill(&Palette99::pick(1)).unwrap();

    upper_one.relative_to_height(30.0);

    let _left_title = upper_two.titled("This is the title", ("serif", 30));

    upper_two.fill(&Palette99::pick(5)).unwrap();

    let chart_1 = upper_areas.get(0);

    // chart_1.fill(&GREEN).unwrap();
    chart_1.expect("REASON").fill(&GREEN).unwrap();

    // https://github.com/plotters-rs/plotters/blob/master/plotters/examples/histogram.rs

    let mut chart_2: Option<&plotters::prelude::DrawingArea<BitMapBackend<'_>, plotters::coord::Shift>> = upper_areas.get(1);
    chart_2.

    // chart_2 = ChartBuilder::on(&root)
    //     .x_label_area_size(35)
    //     .y_label_area_size(40)
    //     .margin(5)
    //     .caption("Histogram Test", ("sans-serif", 50.0))
    //     .build_cartesian_2d((0u32..10u32).into_segmented(), 0u32..10u32)?;

    // chart_1.fill(&GREEN).unwrap();
   // chart_2.expect("REASON").fill(&BLUE).unwrap();
   // chart_2.expect("REASON").relative_to_height(30.0_f64);

    let chart_3 = upper_areas.get(2);

    // chart_1.fill(&GREEN).unwrap();
    chart_3.expect("REASON").fill(&GREEN).unwrap();

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
    Ok(())
}

// cargo run --example

// cargo run --example 39_plotters_multi_panel
