use std::{env, fs, io};

fn echo(args: &[String]){
    for arg in args.iter() {
        println!("{}", arg);
    }
}

fn concatenates_files(path_1: &str, path_2: &str, output_path: &str)
{
    let mut final_file = fs::OpenOptions::new()
        .append(true)
        .create_new(true)
        .open(output_path).unwrap();
    
    let mut first_file = fs::OpenOptions::new()
        .read(true)
        .open(path_1).unwrap();
    
    let mut second_file = fs::OpenOptions::new()
        .read(true)
        .open(path_2).unwrap();
    
    io::copy(&mut first_file, &mut final_file).unwrap();
    io::copy(&mut second_file, &mut final_file).unwrap();
}

fn ls(path: &str) {
    for file in fs::read_dir(path).unwrap() {
        println!("{}", file.unwrap().path().display());
    }
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    
    if args.is_empty() { 
        return;
    }
    
    let command_name = &args[0];
    
    match command_name.as_ref() { 
        "echo" => {
            if args.len() == 1 {
                return;
            }
            echo(&args[1..]);
        },
        "cat" => {
            if args.len() == 4 {
                concatenates_files(&args[1], &args[2], &args[3]);
            }
        },
        "ls" => {
            if args.len() == 2 {
                ls(&args[1]);
            }
        }
        _ => ()
    }
    
}
