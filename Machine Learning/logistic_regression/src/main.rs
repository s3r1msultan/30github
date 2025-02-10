use std::ops::BitAnd;
use linfa::Dataset;
use linfa::prelude::{Fit, Predict};
use linfa_logistic::{FittedLogisticRegression, LogisticRegression};
use ndarray::{Array1, Array2, Axis};
use polars::prelude::*;
use rand::prelude::*;

fn main() -> PolarsResult<()> {
    let path = "./diabetes_cleaned.csv";
    let df_cleaned = load_dataset(path).expect("There's something wrong with the dataframe");
    // clean_dataset(&df)?; // Uncomment this line to clean the dataset

    let feature_cols: Vec<&str> = df_cleaned
        .get_column_names()
        .iter()
        .filter(|&&x| x.ne("Outcome".into()))
        .map(|x| x.as_str())
        .collect();


    let num_rows = df_cleaned.height();
    let num_features = feature_cols.len();

    let mut features = Array2::<f64>::zeros((num_rows, num_features));
    let mut labels = Array1::<i64>::zeros(num_rows);

    for i in 0..num_rows {
        for (j, col) in feature_cols.iter().enumerate() {
            features[[i, j]] = df_cleaned.column(col)?.cast(&DataType::Float64)?.f64()?.get(i).unwrap();
        }
        labels[i] = df_cleaned.column("Outcome")?.cast(&DataType::Int64)?.i64()?.get(i).unwrap();
    }

    // println!("Features: {:?}", features);
    // println!("Labels: {:?}", labels);

    let mut indices: Vec<usize> = (0..num_rows).collect();
    indices.shuffle(&mut thread_rng());

    let train_size = (0.8 * num_rows as f64) as usize;
    let train_features = features.select(Axis(0), &indices[..train_size]);
    let test_features = features.select(Axis(0), &indices[train_size..]);
    let train_labels = labels.select(Axis(0), &indices[..train_size]);
    let test_labels = labels.select(Axis(0), &indices[train_size..]);

    let model = train_model(&train_features, &train_labels);

    let accuracy = compute_accuracy(&model, &test_features, &test_labels);
    println!("Logistic Regression Model Accuracy: {:.2}%", accuracy * 100.0);

    Ok(())
}

fn load_dataset(path: &str) -> PolarsResult<DataFrame> {
    CsvReadOptions::default()
        .with_has_header(true)
        .with_infer_schema_length(None)
        .try_into_reader_with_file_path(Some(path.into()))?
        .finish()
}

fn clean_dataset(df: &DataFrame) -> PolarsResult<()> {
    let mut df = df;
    let columns = vec!["Glucose", "BloodPressure", "SkinThickness", "BMI"];

    let default_column = Column::new("Default".into(), vec![0.0; df.height()]);
    let df_cleaned = df.filter(&columns.iter().fold(
        df.column(&columns[0])?.not_equal(&default_column)?,
        |acc, col| acc.bitand(df.column(col).unwrap().not_equal(&default_column).unwrap()),
    ))?;

    CsvWriter::new(std::fs::File::create("diabetes_cleaned.csv")?)
        .include_header(true)
        .finish(&mut df_cleaned.clone())?;

    println!(
        "Data cleaning complete! {} rows removed, {} rows remaining.",
        df.height() - df_cleaned.height(),
        df_cleaned.height()
    );

    Ok(())
}


fn train_model(
    features: &Array2<f64>,
    labels: &Array1<i64>
) -> FittedLogisticRegression<f64, i64> {
    let dataset = Dataset::new(features.clone(), labels.clone());
    LogisticRegression::default().fit(&dataset).unwrap()
}


fn compute_accuracy(model: &FittedLogisticRegression<f64, i64>, test_features: &Array2<f64>, test_labels: &Array1<i64>) -> f64 {
    let predictions = model.predict(test_features);
    let correct = predictions.iter()
        .zip(test_labels.iter())
        .filter(|(pred, actual)| pred == actual)
        .count();

    correct as f64 / test_labels.len() as f64
}