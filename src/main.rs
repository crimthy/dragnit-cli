extern crate dragnit;
extern crate clap; 
use dragnit::*;
use clap::{Arg, App};
use std::fs::metadata;
use std::io;
use std::fs::File;
use std::io::prelude::*;

const SCHEMA_EXTENSION: &str = ".drgn";
const BINARY_SCHEMA_EXTENSION: &str = ".bdrgn";
const DEFAULT_BINARY_OUTPUT: &str = "output.bdrgn";

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

fn is_schema(name: &str) -> bool {
    return if name.ends_with(SCHEMA_EXTENSION) { true } else { false };
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

fn compile_schema(schema_path: &str, target: &str) -> Result<(), io::Error> {
    println!("{}",schema_path);
    println!("{}",target);
    let md = metadata(schema_path)?;    
    Ok(())
}

fn write_to_file(target_path: &str) -> Result<(), io::Error> {
    let mut file = r#try!(File::create(target_path));
    match file.write_all(b"This is a list of my best friends.") {
        Ok(v) => v,
        Err(e) => return Err(From::from(e)),
    }
    Ok(())
}

fn main_old() {
    let matches = App::new("dragnit-cli")
                            .version("0.0.1")
                            .author("rostegg")
                            .about("dragnit schema manipulation from console")
                            .arg(Arg::with_name("compile")
                                .short("c")
                                .long("compile")
                                .takes_value(true)
                                .help("Compile single schema or schemas in folder"))
                            .arg(Arg::with_name("schema-path")
                                .help("Single schema or schemes folder path")
                                .required(true)
                                .index(1))
                            .get_matches();

    let target_schema = matches.value_of("schema-path")
                        .unwrap_or_exit("Can't unwrap")
                        .expect_or_exit(&is_schema, "Wrong schema extension (require .drgn)")
                        .to_owned();
    
    if matches.is_present("compile") {
        let target_output = matches.value_of("compile")
                            .unwrap_or(DEFAULT_BINARY_OUTPUT)
                            .expect_or_exit(&is_binary_schema, "Wrong binary schema extension (require .bdrgn)")
                            .to_owned();
                
        match compile_schema(&*target_schema, &*target_output) {
            Ok(v) => v,
            Err(e) => println!("{:?}", e),
        }
    }
}

pub fn save_to(target: String, bytes: Vec<u8>) -> std::io::Result<()> {
    let mut pos = 0;
    let mut buffer = File::create(target)?;
    while pos < bytes.len() {
        let bytes_written = buffer.write(&bytes[pos..])?;
        pos += bytes_written;
    }
    Ok(())
}

fn main() {
    let schema = Schema::new(vec![
        Def::new("Point".to_owned(), DefKind::Struct, vec![
          Field {name: "x".to_owned(), type_id: TYPE_FLOAT, is_array: false, value: 0},
          Field {name: "y".to_owned(), type_id: TYPE_FLOAT, is_array: false, value: 0},
        ]),
    ]);
    
    match save_to("test.output".to_owned(), schema.encode()){
        Ok(v) => v,
        Err(e) => println!("{:?}", e),
    }
    
    let schema = Schema::decode(&std::fs::read("test.output").unwrap()).unwrap();
    println!("{:?}", schema);
}