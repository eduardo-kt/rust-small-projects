use plotters::prelude::*;

pub fn desenhar_grafico(valores: &Vec<f64>) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("grafico.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let max_val = valores
        .iter()
        .cloned()
        .fold(f64::NAN, f64::max); // valor máximo para ajustar escala

    let mut chart = ChartBuilder::on(&root)
        .caption("Evolução dos valores", ("sans-serif", 30))
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0..valores.len(), 0.0..max_val)?;

    chart.configure_mesh().draw()?;

    chart.draw_series(LineSeries::new(
        valores.iter().enumerate().map(|(i, v)| (i, *v)),
        &BLUE,
    ))?;

    Ok(())
}
