use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use csv::Reader;
use serde::Deserialize;
use smartcore::ensemble::random_forest_regressor::RandomForestRegressor;
use smartcore::linalg::basic::arrays::ArrayView1;
use smartcore::linalg::basic::matrix::DenseMatrix;
use smartcore::metrics::mean_squared_error;
use smartcore::model_selection::train_test_split;

fn load_csv(path: &str) -> Result<Vec<HouseData>, Box<dyn Error>> {
    let file = File::open(path)?;
    let mut rdr = Reader::from_reader(BufReader::new(file));
    let mut data = Vec::new();

    for result in rdr.deserialize() {
        let record: HouseData = result?;
        data.push(record);
    }

    Ok(data)
}
#[derive(Debug, Deserialize)]
struct HouseData {
    #[serde(rename = "OverallQual")]
    overall_quality: f64,

    #[serde(rename = "GrLivArea")]
    living_area: f64,

    #[serde(rename = "GarageCars")]
    garage_cars: f64,

    #[serde(rename = "TotalBsmtSF")]
    basement_area: f64,

    #[serde(rename = "SalePrice")]
    sale_price: f64
}

fn dataset_to_matrix(ds: &Vec<HouseData>) -> (DenseMatrix<f64>, Vec<f64>) {
    let features: Vec<Vec<f64>> = ds.iter().map(|house| vec![
        house.basement_area,
        house.garage_cars,
        house.living_area,
        house.overall_quality
    ]).collect();

    let target: Vec<f64> = ds.iter().map(|house| house.sale_price).collect();

    (DenseMatrix::from_2d_vec(&features).unwrap(), target)
}



fn train_random_forest(x: &DenseMatrix<f64>, y: &Vec<f64>) -> RandomForestRegressor<f64, f64, DenseMatrix<f64>, Vec<f64>> {
    let (x_train, x_test, y_train, y_test) = train_test_split(x, y, 0.2, true, None);

    let model = RandomForestRegressor::fit(&x_train, &y_train, Default::default())
        .expect("Model training failed");

    let y_pred = model.predict(&x_test).expect("Prediction failed");

    let mse = mean_squared_error(&y_test, &y_pred);
    let rmse = mse.sqrt();

    let rows = x.iter().len();
    println!("Dataset size: {}", rows);
    println!("Min price: {:.2}", y_pred.iter().min_by(|a, b| a.total_cmp(&b)).unwrap());
    println!("Mean price: {:.2}", y_pred.mean_by());
    println!("Max price: {:.2}", y_pred.iter().max_by(|a, b| a.total_cmp(&b)).unwrap());
    println!("Root Mean Squared Error (RMSE): {:.2}", rmse);

    model
}

fn main() {
    let data_path = "./train.csv";
    let data = load_csv(data_path).expect("There's an error with loading CSV file");
    let (x_train, y_train) = dataset_to_matrix(&data);

    let model = train_random_forest(&x_train, &y_train);
}


