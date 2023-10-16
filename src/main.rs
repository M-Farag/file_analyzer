use std::fs::File;
use std::io::BufReader;

fn main() {

    let file_handler = file_handler("data.txt");
    let buf_reader = BufReader::new(file_handler);

}


fn file_handler(file_name:&str) -> Result<File,String>
{
    let file_handler = File::open(file_name)?;
    Ok(file_handler)
}
