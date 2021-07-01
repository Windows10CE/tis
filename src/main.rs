#[macro_use]
extern crate lazy_static;

use std::{error, fs::{File, OpenOptions}, io::{Read, Write}};

mod puzzle;
use puzzle::Puzzle;
mod machine;
use machine::Machine;
mod node;
mod instruction;

fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.len() != 2 {
        return Err("Need exactly 2 args, first is the .tis file to run and the second is the puzzle definition lua file".into());
    }

    if args.iter().any(|x| File::open(x).is_err()) {
        return Err("Both files need to exist and be accessable".into());
    }
    
    let mut lua_string = String::new();

    File::open(&args[1])?.read_to_string(&mut lua_string)?;

    let puzzle = Puzzle::from_lua(lua_string)?;

    let machine = Machine::new(puzzle)?;
    
    let mut tis_string = String::new();

    File::open(&args[0])?.read_to_string(&mut tis_string)?;

    machine.load_instructions(tis_string)?;

    while machine.not_complete() {
        machine.update();
    }

    if let Ok(mut file) = OpenOptions::new().write(true).create(true).open("results.txt") {
        file.write_all(machine.end_results().as_bytes())?;
    }

    Ok(())
}
