use plotters::prelude::*;

pub fn draw(coordinate : Vec<(f32 , f32)>) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("./assets/result.png", (1200, 1200)).into_drawing_area();
    let _ = root.fill(&WHITE);
    let root = root.margin(12, 12, 12, 12);

    let (max_first, max_second) = coordinate.iter().fold(
        (f32::NEG_INFINITY, f32::NEG_INFINITY),
        |(max_x, max_y), &(x, y)| (x.max(max_x), y.max(max_y)),
    );

    let mut chart = ChartBuilder::on(&root)
        .caption("Rustnetic plot", ("sans-serif", 40)
        .into_font())
        .x_label_area_size(20)
        .y_label_area_size(40)
        .build_cartesian_2d(0f32..max_first + 3.0 , 0f32..max_second + 3.0)?;

    chart
        .configure_mesh()
        .x_labels(5)
        .y_labels(5)
        .y_label_formatter(&|x| format!("{:.3}", x))
        .draw()?;

    
    chart.draw_series(LineSeries::new(
        coordinate.clone(),
        &GREEN,
    ))?;

    chart.draw_series(PointSeries::of_element(
        coordinate.clone(),
        5,
        &GREEN,
        &|c, s, st| {
            return EmptyElement::at(c)    
            + Circle::new((0,0),s,st.filled()) 
            + Text::new(format!("{:?}", c), (10, 0), ("sans-serif", 13).into_font());
        },
    ))?;

    root.present()?;
    Ok(())
}