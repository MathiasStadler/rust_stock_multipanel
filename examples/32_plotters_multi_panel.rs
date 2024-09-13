// FROM HERE
// https://plotters-rs.github.io/book/basic/multipanel.html

use plotters::prelude::*;
use plotters::prelude::full_palette::YELLOW;
fn main() {
    let root = BitMapBackend::new("images/32_1_split-drawing-area.png", (640, 480)).into_drawing_area();

    // We can split to left panel and right panel 15% left, 85% right
    let (left, right) = root.split_horizontally((15).percent_width());

    left.fill(&YELLOW).unwrap();

    // We can also split upper and lower panel
    let (upper, lower) = right.split_vertically(140);
    upper.fill(&GREEN).unwrap();

    // Then we can split the lower area evenly to 3 row 2 col
    let lower_areas = lower.split_evenly((2, 3));

    // for (id, area) in lower_areas.into_iter().enumerate() {
    //     area.fill(&Palette99::pick(id)).unwrap();
    // }
}

// cargo run --example 

// cargo run --example 32_plotters_multi_panel