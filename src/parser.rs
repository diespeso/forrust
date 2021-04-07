use std::fs::File;
use std::io::prelude::*;

use crate::time_series::TimeSeries;

pub const DATA_FILE_NAME: &str = "data3.txt";

pub fn data_file_to_vec(file_name: &str) -> std::io::Result<Vec<(f64, f64)>>{
    let mut file = File::open(file_name)?;
    let mut texto = String::new();
    file.read_to_string(&mut texto)?;
    let contenido: Vec<String> = texto.split(" ").map(|x| x.to_owned()).collect();
    
    let mut resultado = Vec::new();
    
    for i in 0..contenido.len() {
        resultado.push(
            ((i + 1) as f64,
            contenido[i].parse::<f64>().expect(
                format!("not a number in data input at pos{}", i).as_ref()
            ))
        )
    }
    Ok(resultado)
}

pub fn data_file_to_timeseries(file_name: &str) -> TimeSeries {
    let data = data_file_to_vec(file_name);
    TimeSeries::from_pairs_vec(data.expect(
        format!("couldn't create timeseries from data file: {}", file_name)
        .as_ref())
    )
}