mod visualize;

use linfa::prelude::*;
use linfa_linear::{FittedLinearRegression, LinearRegression};
use ndarray::{Array1, Array2};
use polars::prelude::*;

fn load_dataset(path: &str) -> PolarsResult<DataFrame> {
    CsvReadOptions::default()
        .with_has_header(true)
        .with_infer_schema_length(None)
        .try_into_reader_with_file_path(Some(path.into()))?
        .finish()
}

fn preprocess_data(df: &DataFrame) -> (Array2<f64>, Array1<f64>) {
    let x_series = df
        .column("Years of Experience")
        .expect("There's no such a column")
        .cast(&DataType::Float64)
        .expect("Unable to convert data into Float64")
        .f64()
        .expect("Invalid column type")
        .into_no_null_iter()
        .collect::<Vec<f64>>();

    let y_series = df
        .column("Salary")
        .expect("There's no such a column")
        .cast(&DataType::Float64)
        .expect("Unable to convert data into Float64")
        .f64()
        .expect("Invalid column type")
        .into_no_null_iter()
        .collect::<Vec<f64>>();

    let x_array = Array2::from_shape_vec((x_series.len(), 1), x_series).expect("There's an error with Array2 data type");

    let y_array = Array1::from_vec(y_series);
    (x_array, y_array)
}


fn train_model(x: &Array2<f64>, y: &Array1<f64>) -> FittedLinearRegression<f64> {
    let dataset = Dataset::new(x.clone(), y.clone());
    let model = LinearRegression::new().fit(&dataset).expect("Model training failed");
    model
}

fn evaluate_model(model: &FittedLinearRegression<f64>, x: &Array2<f64>, y: &Array1<f64>) {
    let predictions = model.predict(x);

    let mse = y.iter()
        .zip(predictions.iter())
        .map(|(yi, yi_hat)| (yi - yi_hat).powf(2.0))
        .sum::<f64>() / y.len() as f64;

    let y_mean = y.mean().expect("Unable to compute mean");

    let ss_total = y.iter()
        .map(|yi| (yi - y_mean).powf(2.0))
        .sum::<f64>();

    let ss_residual = y.iter()
        .zip(predictions.iter())
        .map(|(yi, yi_hat)| (yi - yi_hat).powf(2.0))
        .sum::<f64>();

    let r2_score = 1.0 - (ss_residual / ss_total);

    let sum_year = x.iter().sum::<f64>();
    let sum_year_2 = x.iter().map(|year| year*year).sum::<f64>();
    let sum_salary = y.iter().sum::<f64>();
    let sum_salary_2 = y.iter().map(|salary| salary * salary).sum::<f64>();
    let sum_year_salary = y.iter().zip(x.iter()).map(|(salary, year)| salary * year).sum::<f64>();
    let min_salary = y.iter().min_by(|a, b| a.total_cmp(&b)).unwrap();
    let max_salary = y.iter().max_by(|a, b| a.total_cmp(&b)).unwrap();

    println!("Sx: {:.2}", sum_year);
    println!("Sy: {:.2}", sum_salary);
    println!("Sxy: {:.2}", sum_year_salary);
    println!("Sxx: {:.2}", sum_year_2);
    println!("Syy: {:.2}", sum_salary_2);
    println!("(Sx)^2: {:.2}", sum_year*sum_year);


    println!();
    println!("Minimum salary: {:.2}", min_salary);
    println!("Mean salary: {:.2}", y_mean);
    println!("Maximum salary: {:.2}", max_salary);
    println!("Mean Squared Error: {:.2}", mse);
    println!("R² Score: {:.4}", r2_score);
    println!("y = Ax + B");
    println!("A: {}", model.params()[0]);
    println!("B: {}", model.intercept());
    println!();
    println!()
}

fn main() {
    let df = load_dataset("./Salary Data.csv").expect("Failed to load dataset");
    let (x, y) = preprocess_data(&df);

    let trained_model = train_model(&x, &y);

/*    let intercept = trained_model.intercept();
    let param = trained_model.params()[0];
    println!("y = {param:.2}*x + {intercept:.2}");
    let check_data = 15.0;
    println!("Experience years: {}, Manual Prediction: {}", check_data, param * check_data + intercept);*/

    evaluate_model(&trained_model, &x, &y);

    let experience_years = vec![5.0, 7.5, 10.0, 20.0];

    let experience = Array2::from_shape_vec((experience_years.len(), 1), experience_years.clone()).unwrap();
    let prediction = trained_model.predict(&experience);

    for i in 0..experience_years.len() {
        println!("Experience years: {} | Predicted salary: {:.2}", experience_years[i], prediction[i]);
    }

    visualize::visualize_regression(&x, &y, &trained_model);
}
