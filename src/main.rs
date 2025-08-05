use polars::prelude::*;
use std::fs::File;

fn main() {
    let file = File::open("./test_data.parquet").unwrap();
    let df = ParquetReader::new(file).finish().unwrap();

    dbg!(df);
}
