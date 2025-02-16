use std::error::Error;
use csv;

fn read_from_csv(path: &str)-> Result<(),Box<dyn Error>>{
    let mut reader = csv::Reader::from_path(path)?;
    for result in reader.records(){
        let record = result?;

        println!("{:?}",record);
    }
    Ok(())
}

pub fn csv_handling(){
    if let Err(e) = read_from_csv("./topmedrooms1.csv"){
        eprintln!("{}",e);
    }
}
