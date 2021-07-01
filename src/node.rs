use std::collections::{HashMap, VecDeque};

use crate::{instruction::{Instruction, ParseError}, machine::Machine};

pub trait Node {
    fn new(index: usize) -> Self where Self : Sized;
    fn get_index(&self) -> usize;
    fn read_from(&mut self, location: &Location) -> Option<i32> { None }
    fn update(&mut self, machine: &mut Machine) {}
    fn try_set_instructions(&mut self, instructions: Vec<Instruction>) -> Result<(), Box<dyn std::error::Error>> { Err("Node not capable of executing an instruction!".into()) }
}

pub struct Compute {
    last_direction: Option<Location>,
    acc: i32,
    bak: i32,
    index: usize,
    read_map: HashMap<Location, (i32, bool)>,
    instructions: Vec<Instruction>,
    instruction_index: usize,
}

impl Node for Compute {
    fn new(index: usize) -> Self {
        Compute {
            last_direction: None,
            acc: 0,
            bak: 0,
            index,
            read_map: HashMap::new(),
            instructions: Vec::new(),
            instruction_index: 0,
        }
    }

    fn get_index(&self) -> usize {
        self.index
    }
    fn read_from(&mut self, location: &Location) -> Option<i32> {
        match self.read_map.remove(location) {
            Some((val, lock)) => match lock {
                true => { self.read_map.insert(*location, (val, lock)); None },
                false => Some(val),
            },
            None => None,
        }
    }
    fn update(&mut self, machine: &mut Machine) {
        if self.read_map.len() != 0 {
            self.read_map.iter_mut().for_each(|x| x.1.1 = false);
            return;
        }

        self.instructions[self.instruction_index].clone().execute(self);
    }
    fn try_set_instructions(&mut self, instructions: Vec<Instruction>) -> Result<(), Box<dyn std::error::Error>> {
        self.instructions = instructions;

        Ok(())
    }
}

pub struct Memory {
    stack: VecDeque<i32>,
    index: usize,
}

impl Node for Memory {
    fn new(index: usize) -> Self {
        Memory {
            stack: VecDeque::new(),
            index,
        }
    }
    
    fn get_index(&self) -> usize {
        self.index
    }
    fn read_from(&mut self, _: &Location) -> Option<i32> {
        self.stack.pop_front()
    }
    fn update(&mut self, machine: &mut Machine) {
        for dir in DIRECTIONS.iter() {
            if let Some(node) = machine.get_neighbor(self, *dir) {
                if let Some(val) = node.read_from(&Location::opposite(dir)) {
                    self.stack.push_front(val);
                }
            }
        }
    }
}

pub struct Corrupted(pub usize);

impl Node for Corrupted {
    fn new(index: usize) -> Self {
        Corrupted(index)
    }
    fn get_index(&self) -> usize {
        self.0
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub enum Location {
    LEFT,
    RIGHT,
    UP,
    DOWN,
    ANY,
    ACC,
    NIL,
    CONSTANT(i32)
}

lazy_static! {
    static ref DIRECTIONS: Vec<Location> = vec![Location::UP, Location::DOWN, Location::LEFT, Location::RIGHT];
}

impl Location {
    pub fn parse(text: &str) -> Result<Location, ParseError> {
        match text {
            "LEFT" => Ok(Location::LEFT),
            "RIGHT" => Ok(Location::RIGHT),
            "UP" => Ok(Location::UP),
            "DOWN" => Ok(Location::DOWN),
            "ANY" => Ok(Location::ANY),
            "ACC" => Ok(Location::ANY),
            "NIL" => Ok(Location::NIL),
            constant if i32::from_str_radix(constant, 10).is_ok() => Ok(Location::CONSTANT(i32::from_str_radix(constant, 10).unwrap())),
            _ => Err(ParseError("".to_string()))
        }
    }

    pub fn opposite(&self) -> Location {
        match *self {
            Location::UP => Location::DOWN,
            Location::DOWN => Location::UP,
            Location::LEFT => Location::RIGHT,
            Location::RIGHT => Location::LEFT,
            _ => Location::NIL,
        }
    }
}
