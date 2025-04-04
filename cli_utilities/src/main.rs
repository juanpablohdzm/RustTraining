// rust
use clap::{Parser, Subcommand};
use thiserror::Error;
use std::{env, fs};
use std::io::Write;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Error, Debug)]
pub enum CliError {
    #[error("error: {0}")]
    Io(#[from] std::io::Error),
    #[error("File is empty")]
    EmptyFile,
    #[error("Error writing file")]
    WriteError(#[from] std::convert::Infallible),
    #[error("Pattern not found")]
    PatternNotFound,
}

#[derive(Subcommand)]
enum Commands {
    Echo {
        #[arg(short, long)]
        message: String,
    },
    Cat {
        #[arg(short = 'i', long, value_name = "File1")]
        f1: String,
        #[arg(short = 'o', long, value_name = "File2")]
        f2: String,
    },
    Ls,
    Find {
        #[arg(short, long)]
        path: Option<String>,
        #[arg(short, long)]
        name: String,
    },
    Grep {
        #[arg(short, long)]
        pattern: String,
        #[arg(short, long)]
        file_name: String,
    }
}

fn main() -> Result<(), CliError> {
    let args = Cli::parse();
    match args.cmd {
        Commands::Echo { message } => {
            println!("{}", message);
            Ok(())
        },
        Commands::Cat { f1, f2 } => {
            // Check if the source file is empty
            let metadata = fs::metadata(&f1)?;
            if metadata.len() == 0 {
                return Err(CliError::EmptyFile);
            }
            
            let mut content: Vec<u8> = vec![b'\n'];
            content.append(fs::read(f1)?.as_mut());
            let mut file = fs::OpenOptions::new().append(true).open(&f2)?;
            file.write_all(content.as_slice())?;
            Ok(())
        },
        Commands::Ls => {
            let entries = fs::read_dir(env::current_dir().unwrap())?;
            for entry in entries  {
                let entry = entry?;
                let file_name = entry.file_name();
                println!("{}", file_name.to_string_lossy());
            }
            Ok(())
        },
        Commands::Find { path, name} => {
            let path = path.unwrap_or_else(|| env::current_dir().unwrap().to_string_lossy().to_string());
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let file_name = entry.file_name();
                if file_name.to_string_lossy().contains(&name) {
                    println!("{}", file_name.to_str().unwrap());
                }
            }
            Ok(())
        },
        Commands::Grep {pattern, file_name} => {
            let content = fs::read_to_string(&file_name)?;
            content.find(pattern.as_str()).ok_or(CliError::PatternNotFound)?;
            println!("{}", pattern);
            Ok(())
        }
    }
}