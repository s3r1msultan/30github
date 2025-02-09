use linfa_linear::FittedLinearRegression;
use ndarray::{Array1, Array2};
use plotters::prelude::*;

pub fn visualize_regression(x: &Array2<f64>, y: &Array1<f64>, model: &FittedLinearRegression<f64>) {
    let root = BitMapBackend::new("linear_regression.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let max_x = *x.iter().max_by(|a, b| a.total_cmp(&b)).unwrap();
    // let min_x = *x.iter().min_by(|a, b| a.total_cmp(&b)).unwrap();
    // let n_x = x.len();
    let max_y = *y.iter().max_by(|a, b| a.total_cmp(&b)).unwrap();
    // let min_y = *y.iter().min_by(|a, b| a.total_cmp(&b)).unwrap();
    // let n_y = y.len();


    let mut chart = ChartBuilder::on(&root)
        .caption("Linear Regression: Salary vs. Experience", ("sans-serif", 30))
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(50)
        .build_cartesian_2d(
            (0f64..1.1*max_x).step(1.0),
            (0f64..1.1*max_y).step(100.0),
        )
        .unwrap();

    chart.configure_mesh().draw().unwrap();

    chart
        .draw_series(
            x.iter()
                .zip(y.iter())
                .map(|(x_val, y_val)| Circle::new((*x_val, *y_val), 3, RED.filled())),
        )
        .unwrap();

    let regression_points: Vec<(f64, f64)> = (0..=(1.1*max_x) as i64 )
        .map(|x| {
            let x_value = x as f64;
            let y_pred = model.intercept() + model.params()[0] * x_value;
            (x_value, y_pred)
        })
        .collect();

    chart
        .draw_series(LineSeries::new(regression_points, &BLUE))
        .unwrap()
        .label("Regression Line")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

    chart.configure_series_labels().border_style(&BLACK).draw().unwrap();

    root.present().expect("Unable to write to file");
    println!("Regression plot saved as 'linear_regression.png'");
}