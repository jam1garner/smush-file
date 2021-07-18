use std::env;
use std::convert::TryInto;

fn main() {
    let [file_name]: [String; 1] = env::args()
        .skip(1)
        .collect::<Vec<String>>()
        .try_into()
        .unwrap_or_else(|_| {
            println!("Missing argument: filepath");
            std::process::exit(1);
        });

    let contents = std::fs::read(file_name).unwrap();
    
    println!();
    println!("{}", smush_file::get_from_magic(&contents));
}
