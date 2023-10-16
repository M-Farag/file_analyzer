use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()>
{

    let file_handler = file_handler("data.txt").expect("File not found");
    let buf_reader = BufReader::new(file_handler);

    for line in buf_reader.lines()
    {
        let line = line?;
        println!("{}",line);
    }

    Ok(())
}


fn file_handler(file_name:&str) -> Result<File, Box<dyn std::error::Error>>
{
    let file_handler = File::open(file_name)?;
    Ok(file_handler)
}
