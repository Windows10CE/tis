use std::error;

use crate::node::Node;

use mlua::{FromLua, Lua, LuaSerdeExt, Table, Value};

pub struct Puzzle {
    pub streams: Vec<AnswerStream>,
    pub layout: Vec<NodeType>,
}

pub struct AnswerStream {
    pub stream_type: StreamType,
    pub name: String,
    pub offset: usize,
    pub values: Vec<i32>,
    pub answers: Vec<i32>,
}

impl<'lua> FromLua<'lua> for AnswerStream {
    fn from_lua(lua_value: Value<'lua>, lua: &'lua Lua) -> mlua::Result<Self> {
        let table = Table::from_lua(lua_value, &lua)?;

        mlua::Result::Ok(AnswerStream {
            stream_type: match table.get(1)? {
                1 => StreamType::STREAM_INPUT,
                2 => StreamType::STREAM_OUTPUT,
                3 => StreamType::STREAM_IMAGE,
                _ => return mlua::Result::Err(mlua::prelude::LuaError::DeserializeError("Unknown AnswerStream type!".to_string()))
            },
            name: table.get(2)?,
            offset: table.get(3)?,
            values: table.get(4)?,
            answers: Vec::new(),
        })
    }
}

impl AnswerStream {
    pub fn check_completion(&self) -> bool {
        if self.values.len() != self.answers.len() {
            return false;
        }
        
        let mut result = true;

        for index in 0..self.values.len() {
            result &= self.values[index] == self.answers[index];
        }

        return result;
    }
}

impl Node for AnswerStream {
    fn new(_: usize) -> Self {
        unimplemented!()
    }
    fn get_index(&self) -> usize {
        self.offset
    }
}

#[derive(PartialEq)]
#[allow(non_camel_case_types)]
pub enum StreamType {
    STREAM_INPUT,
    STREAM_OUTPUT,
    STREAM_IMAGE,
}

#[derive(PartialEq)]
#[allow(non_camel_case_types)]
pub enum NodeType {
    TILE_COMPUTE,
    TILE_MEMORY,
    TILE_DAMAGED,
}

impl<'lua> FromLua<'lua> for NodeType {
    fn from_lua(lua_value: Value<'lua>, lua: &'lua Lua) -> mlua::Result<Self> {
        mlua::Result::Ok(match lua.from_value(lua_value)? {
            4 => NodeType::TILE_COMPUTE,
            5 => NodeType::TILE_MEMORY,
            6 => NodeType::TILE_DAMAGED,
            _ => return mlua::Result::Err(mlua::prelude::LuaError::DeserializeError("Unknown node type!".to_string()))
        })
    }
}

impl Puzzle {
    pub fn from_lua(puzzle_string: String) -> Result<Puzzle, Box<dyn error::Error>> {
        let lua = Lua::new();

        let globals = lua.globals();
        globals.set("STREAM_INPUT", 1)?;
        globals.set("STREAM_OUTPUT", 2)?;
        globals.set("STREAM_IMAGE", 3)?;
        globals.set("TILE_COMPUTE", 4)?;
        globals.set("TILE_MEMORY", 5)?;
        globals.set("TILE_DAMAGED", 6)?;

        lua.load(&puzzle_string).exec()?;
        
        println!("{}", lua.used_memory());

        Ok(Puzzle {
            streams: lua.load("get_streams()").eval()?,
            layout: lua.load("get_layout()").eval()?,
        })
    }
}
