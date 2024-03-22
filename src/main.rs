use csv_diff::{csv::Csv, csv_diff::CsvByteDiffLocal, diff_row::DiffByteRecord};

fn get_csv_data(file_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let csv_data = std::fs::read_to_string(file_path)?;
    Ok(csv_data)
}

fn compare_csv_strings(
    csv_data_left: &str,
    csv_data_right: &str,
) -> Result<Vec<DiffByteRecord>, Box<dyn std::error::Error>> {
    let csv_byte_diff = CsvByteDiffLocal::new()?;

    let mut diff_byte_records = csv_byte_diff.diff(
        Csv::with_reader_seek(csv_data_left.as_bytes()),
        Csv::with_reader_seek(csv_data_right.as_bytes()),
    )?;

    diff_byte_records.sort_by_line();

    let diff_byte_rows = diff_byte_records.as_slice();

    Ok(diff_byte_rows.to_vec())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // read paths from command line arguments
    let path1 = std::env::args().nth(1).expect("missing file path 1");
    let path2 = std::env::args().nth(2).expect("missing file path 2");

    // read csv files into strings
    let csv_data_1 = get_csv_data(&path1)?;
    let csv_data_2 = get_csv_data(&path2)?;

    // compare csv files
    let diff = compare_csv_strings(&csv_data_1, &csv_data_2)?;
    println!("{:?}", diff);
    Ok(())
}
