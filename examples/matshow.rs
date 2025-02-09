use plotters::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("plotters-doc-data/matshow.png", (1024, 768)).into_drawing_area();

    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Matshow Example", ("Arial", 80))
        .margin(5)
        .top_x_label_area_size(40)
        .y_label_area_size(40)
        .build_ranged(0i32..15i32, 15i32..0i32)?;

    chart
        .configure_mesh()
        .x_labels(15)
        .y_labels(15)
        .x_label_offset(35)
        .y_label_offset(25)
        .disable_x_mesh()
        .disable_y_mesh()
        .label_style(("Arial", 20))
        .draw()?;

    let mut matrix = [[0; 15]; 15];

    for i in 0..15 {
        matrix[i][i] = i + 4;
    }

    chart.draw_series(
        matrix
            .iter()
            .zip(0..)
            .map(|(l, y)| l.iter().zip(0..).map(move |(v, x)| (x as i32, y as i32, v)))
            .flatten()
            .map(|(x, y, v)| {
                Rectangle::new(
                    [(x, y), (x + 1, y + 1)],
                    HSLColor(
                        240.0 / 360.0 - 240.0 / 360.0 * (*v as f64 / 20.0),
                        0.7,
                        0.1 + 0.4 * *v as f64 / 20.0,
                    )
                    .filled(),
                )
            }),
    )?;

    Ok(())
}
