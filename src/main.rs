extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
use std::env;
use std::fs::File;
use std::process;

mod heap_dump;
mod sys_check;

fn main() {
    let args: Vec<_> = env::args().collect();
    let first_arg = args.get(1);

    if first_arg.is_none() {
        println!("Usage: osn [ruby object space dump]");
        process::exit(1);
    }
    let filename = first_arg.unwrap();
    let mut file = File::open(filename);

    if file.is_err() {
        println!("File '{}' read failure ({:?})", filename, file);
        process::exit(1);
    }

    let mut hd = heap_dump::HeapDump::load_file(file.unwrap());
    hd.print_roots();

    let fsize = sys_check::FileCheck::size_kb(filename);
    println!("File size: {}kb", fsize);
    match sys_check::SysCheck::rss() {
        Some(value) => println!("RSS value: {}kb", value),
        None => println!("Failed to retrieve process RSS")
    }
}
