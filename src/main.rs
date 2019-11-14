extern crate dragnit;
extern crate clap; 
extern crate crimthy_dragnit_schemes ;

use dragnit::*;
use clap::{Arg, App};
use std::io;
use std::io::prelude::*;
use crimthy_dragnit_schemes::build_schemes;
use std::path::{Path, PathBuf};

const BINARY_SCHEMA_EXTENSION: &str = ".bdrgn";
const DEFAULT_BINARY_OUTPUT: &str = "compiled_schemes";

macro_rules! print_exit_message {
    ($fmt:expr) => (write!(io::stderr(), concat!("error: ", $fmt, "\n")).unwrap());
    ($fmt:expr, $($arg:tt)*) =>
        (write!(io::stderr(), concat!("error: ", $fmt, "\n"), $($arg)*).unwrap());
}

macro_rules! error_exit {
    ($($arg:tt)*) => { {
        print_exit_message!($($arg)*);
        std::process::exit(1);
    } }
}

fn is_binary_schema(name: &str) -> bool {
    return if name.ends_with(BINARY_SCHEMA_EXTENSION) { true } else { false };
}

trait UnwrapErrorHandling<T> {
    fn unwrap_or_exit(self, message: &str) -> T;
}

trait StringErrorHandling {
    fn expect_or_exit(self, f: &dyn Fn(&str) -> bool, message: &str) -> String;
}

impl StringErrorHandling for &str {
    fn expect_or_exit(self, f: &dyn Fn(&str) -> bool, message: &str) -> String {
        if !f(self){ error_exit!("{}", message); }
        return self.to_string();
    }
}

impl<T> UnwrapErrorHandling<T> for Option<T> {
    fn unwrap_or_exit(self, message: &str) -> T {
        match self {
            Some(r) => r,
            None => {
                error_exit!("{}", message);
            }
        }
    }
}

fn display_schema(schema_path: &str) -> Result<(), io::Error> {
    let path = Path::new(schema_path);
    let file_stem = path.file_stem();
    // really bad, but really don't care,
    let schema = Schema::decode(file_stem.unwrap().to_str().unwrap().to_owned(), &std::fs::read(path).unwrap()).unwrap();
    println!("{:?}", schema);
    Ok(())
}

fn compile_schema(schema: &Schema, target_path: PathBuf) -> Result<(), io::Error> {
    let path = target_path.into_os_string().into_string().unwrap();
    match Schema::save_to(path, schema.encode()) {
        Ok(v) => v,
        Err(e) => println!("{:?}", e),
    }
    Ok(())
}

fn compile_schemes(schemes_path: &str) -> Result<(), io::Error>{
    fn make_dir(path: &str) -> std::io::Result<()> {
        std::fs::create_dir_all(path)?;
        Ok(())
    }

    match make_dir(schemes_path) {
        Ok(v) => v,
        Err(e) => println!("{:?}", e),
    }

    let schemes = build_schemes();

    schemes.iter().for_each(|schema| {
        let path = Path::new(schemes_path).join(format!("{}{}", schema.name, BINARY_SCHEMA_EXTENSION));
        match compile_schema(schema, path) {
            Ok(v) => v,
            Err(e) => println!("{:?}", e),
        }
    });
    Ok(())
}

fn main() {
    let matches = App::new("dragnit-cli")
                            .version("0.0.1")
                            .author("rostegg")
                            .about("dragnit schema manipulation from console")
                            .arg(Arg::with_name("compile")
                                .short("c")
                                .long("compile")
                                .takes_value(true)
                                .help("Compile all schemes to binary"))
                            .arg(Arg::with_name("display-schema")
                                .help("Decode single schema")
                                .short("d")
                                .long("display-schema")
                                .takes_value(true))
                            .get_matches();
    
    if matches.is_present("compile") {
        let output_dir = matches.value_of("compile").unwrap_or(DEFAULT_BINARY_OUTPUT);
        match compile_schemes(output_dir) {
            Ok(v) => v,
            Err(e) => println!("{:?}", e),
        }                       
    }
    else if matches.is_present("display-schema") {
        let target_schema = matches.value_of("display-schema")
                            .unwrap_or_exit("Can't unwrap target schema path")
                            .expect_or_exit(&is_binary_schema, "Wrong schema format (.bdrgn required)");
        match display_schema(&target_schema) {
            Ok(v) => v,
            Err(e) => println!("{:?}", e),
        }
    }
}