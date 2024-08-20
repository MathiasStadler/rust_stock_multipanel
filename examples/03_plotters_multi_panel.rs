// FROM HERE
// own example

use plotters::prelude::*;

fn main() {
    let root = BitMapBackend::new("images/easy_multi-panel.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE).unwrap();
    let (upper, lower) = root.split_vertically((80).percent());

    let mut upper_chart = ChartBuilder::on(&upper)
        .set_label_area_size(LabelAreaPosition::Left, 30)
        .set_label_area_size(LabelAreaPosition::Right, 30)
        .set_label_area_size(LabelAreaPosition::Top, 30)
        .build_cartesian_2d(0.0..10.0, -1.0..1.0)
        .unwrap();
    upper_chart.configure_mesh().draw().unwrap();

    upper_chart.draw_series(LineSeries::new(
        (0..100).map(|x| x as f64 / 10.0).map(|x| (x, x.sin())),
        &BLACK,
    )).unwrap();

}





// cargo eun --example