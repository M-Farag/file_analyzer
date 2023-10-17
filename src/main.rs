mod analyzer;

use analyzer::analyzer::*;

fn main()
{

    let analyzer = Analyzer::new("data.txt".to_string());

    println!("{:#?}",analyzer);

}