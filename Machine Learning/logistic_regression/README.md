# Logistic Regression in Rust

## **Overview**
This project implements **Logistic Regression** in Rust using the `linfa-logistic` crate for binary classification on the [**Pima Indians Diabetes Dataset**](https://www.kaggle.com/datasets/uciml/pima-indians-diabetes-database/data). The dataset is preprocessed using `polars`, and the model is trained and evaluated using `ndarray` and `linfa`.

---

## **1. Preprocessing the Data**
Before training the model, the dataset undergoes **preprocessing** to ensure clean and meaningful data:
- **Removing invalid rows**: Any row with `0` values in critical columns (`Glucose`, `BloodPressure`, `SkinThickness`, `BMI`) is removed.
- **Extracting features and labels**:
    - Features include **Pregnancies, Glucose, BloodPressure, BMI, etc.**
    - The target variable (`Outcome`) is used for classification.
- **Splitting the dataset**: 80% of the data is used for **training**, and 20% for **testing**.

---

## **2. Working with Polars Crate**
[Polars](https://pola.rs/) is a high-performance DataFrame library used for:
- **Loading the dataset** from a CSV file.
- **Filtering rows** with invalid values.
- **Converting the dataset** into `ndarray::Array2<f64>` for `linfa-logistic`.

### **Example: Loading and Cleaning Data**
```rust
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
```

---

## **3. Creating the Logistic Regression Model**
We use the `linfa-logistic` crate to build and train the **logistic regression model**.

### **Steps to Train Logistic Regression**
1. **Prepare data** (convert `polars::DataFrame` → `ndarray::Array2<f64>`).
2. **Split data into training (80%) and testing (20%)**.
3. **Fit the model** using `linfa-logistic`.
4. **Make predictions and evaluate accuracy**.

### **Example: Training Logistic Regression**
```rust
fn train_model(
  features: &Array2<f64>,
  labels: &Array1<i64>
) -> FittedLogisticRegression<f64, i64> {
  let dataset = Dataset::new(features.clone(), labels.clone());
  LogisticRegression::default().fit(&dataset).unwrap()
}
```

---

## **4. Testing the Model**
The model is evaluated by **calculating accuracy** on the test dataset.

### **Accuracy Calculation**
```rust
fn compute_accuracy(model: &FittedLogisticRegression<f64, i64>, test_features: &Array2<f64>, test_labels: &Array1<i64>) -> f64 {
  let predictions = model.predict(test_features);
  let correct = predictions.iter()
          .zip(test_labels.iter())
          .filter(|(pred, actual)| pred == actual)
          .count();

  correct as f64 / test_labels.len() as f64
}
```
---

## **Results**
- After training the model on **cleaned data**, it achieves an accuracy of **~80%**.
- Logistic Regression effectively predicts whether a patient is **diabetic (1) or not (0)**.

---

## **How to Run the Project**
### **1. Clone Repository**
```sh
git clone https://github.com/s3r1msultan/30github.git
cd ./"Machine Learning"/logistic_regression
```

### **2. Install Dependencies**
```sh
cargo add linfa linfa-logistic polars ndarray rand
```

### **3. Run the Program**
```sh
cargo run
```

---

## **Dependencies Used**
| Crate | Purpose |
|--------|---------|
| `polars` | DataFrame processing |
| `ndarray` | Matrix operations |
| `linfa` | Machine Learning library |
| `linfa-logistic` | Logistic Regression model |
| `rand` | Data shuffling |


---

## **Future Improvements**
- Implement **manual logistic regression** (without `linfa`).
- Add **feature scaling (Standardization / Min-Max Scaling)**.
- Implement **L1/L2 Regularization** for improved generalization.


