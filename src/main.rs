use polars::prelude::*;
use std::{fs::File, path::Path, process};

fn main() {
    let df = match read_df("./breast_cancer_dataset.csv", FileFormat::Csv) {
        Err(_) => process::exit(1),
        Ok(df) => df,
    };
    dbg!(&df);
}

enum FileFormat {
    Parquet,
    Csv,
}

fn read_df<P: AsRef<Path>>(path: P, file_type: FileFormat) -> PolarsResult<DataFrame> {
    let file = File::open(path)?;

    match file_type {
        FileFormat::Parquet => ParquetReader::new(file).finish(),
        FileFormat::Csv => CsvReader::new(file).finish(),
    }
}
