use polars::prelude::*;

fn main() {


    println!("Hello, world!");

    // Read a csv file in Polars 

    let df0 = CsvReadOptions::default().try_into_reader_with_file_path(Some("agentes-geracao-energia-eletrica.csv".into()))
    .unwrap()
    .finish()
    .unwrap();

    //Preview 
    println!("{}",df0);


    // Read a csv file in Polars 

   let df1 = CsvReadOptions::default().try_into_reader_with_file_path(Some("interrupcoes-energia-eletrica-2026.csv".into()))
    .unwrap()
    .finish()
    .unwrap();

    //Preview 
    println!("{}",df1);

}
