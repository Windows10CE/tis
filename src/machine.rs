use crate::puzzle::{NodeType, Puzzle, StreamType};
use crate::node::*;

pub struct Machine {
    pub puzzle: Puzzle,
    pub nodes: Vec<Box<dyn Node>>,
    step_count: usize,
}

impl Machine {
    pub fn new(puzzle: Puzzle) -> Result<Machine, Box<dyn std::error::Error>> {
        let mut nodes: Vec<Box<dyn Node>> = vec![];

        let mut index = 0usize;

        for node in &puzzle.layout {
            if *node == NodeType::TILE_COMPUTE {
                nodes.push(Box::new(Compute::new(index)));
            }
            else if *node == NodeType::TILE_MEMORY {
                nodes.push(Box::new(Memory::new(index)));
            }
            else {
                nodes.push(Box::new(Corrupted(index)));
            }
            index += 1;
        }

        Ok(Machine { puzzle, nodes, step_count: 0, })
    }

    pub fn load_instructions(&self, definition: String) -> Result<(), Box<dyn std::error::Error>> {


        Ok(())
    }

    pub fn update(&self) {
        if self.step_count == usize::MAX {
            panic!("Program takes more steps to complete than is possible to keep track of!")
        }
    }

    pub fn not_complete(&self) -> bool {
        self.puzzle.streams.iter().filter(|x| x.stream_type == StreamType::STREAM_OUTPUT).any(|x| !x.check_completion())
    }

    pub fn get_neighbor(&mut self, node: &dyn Node, loc: Location) -> Option<&mut dyn Node> {
        let index = node.get_index();
        
        match loc {
            Location::UP => {
                if index < 4 {
                    return self.puzzle.streams.iter_mut().find(|x| x.offset == index && x.stream_type == StreamType::STREAM_INPUT).map(|x| x as &mut dyn Node);
                }
                Some(self.nodes[index - 4].as_mut())
            },
            Location::DOWN => {
                if index > 7 {
                    return self.puzzle.streams.iter_mut().find(|x| x.offset == index && x.stream_type == StreamType::STREAM_OUTPUT).map(|x| x as &mut dyn Node);
                }
                Some(self.nodes[index + 4].as_mut())
            },
            Location::LEFT => {
                if index % 4 == 0 {
                    return None;
                }
                Some(self.nodes[index - 1].as_mut())
            },
            Location::RIGHT => {
                if index == 3 || index == 7 || index == 11 {
                    return None;
                }
                Some(self.nodes[index + 1].as_mut())
            },
            _ => None,
        }
    }

    pub fn end_results(&self) -> String {
        String::new()
    }
}
