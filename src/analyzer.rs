pub mod analyzer {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    #[derive(Debug)]
   pub struct Analyzer {
       file_path: String
   }


    impl Analyzer {
        pub fn new(file_path:String) -> Analyzer
        {
            Analyzer {
                file_path
            }
        }
    }
}