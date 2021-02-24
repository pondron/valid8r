use std::io::{self, Write};
use colored::*;

#[derive(Debug)]
pub struct Rezzy {
    pub message: String,
}

impl Rezzy {
    pub fn write_green(&self) {
        println!("\u{2705} {}", self.message.green());
    }

    pub fn write_red(&self) {
        println!("\u{274C} {}", self.message.red());
    }

    pub fn write_yellow(&self) {
        println!("\u{26A0} {}", self.message.yellow());
    }

    pub fn build_output(&self) {
        println!("\u{26A0} {}", self.message.yellow());
    }

    pub fn new(result: char, message: String) -> Rezzy {
        Rezzy {
            message: message,
        }
    }

}