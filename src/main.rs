use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()>
{

    let buf_reader = file_handler("data.txt").expect("File not found");

    for line in buf_reader.lines()
    {
        let line = line?;
        println!("{}",line);
    }

    Ok(())
}


fn file_handler(file_name:&str) -> Result<BufReader<File>, Box<dyn std::error::Error>>
{
    let file_handler = File::open(file_name)?;
    let buf_reader = BufReader::new(file_handler)?;
    Ok(buf_reader)
}
