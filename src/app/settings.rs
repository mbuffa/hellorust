extern crate toml;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub struct Settings {
  filename: &'static str,
  parsed: Option<toml::Table>
}

impl Settings {
  pub fn new(filename: &'static str) -> Settings {
    Settings {
      filename: filename,
      parsed: None
    }
  }

  pub fn load(&mut self) {
    let path = Path::new(self.filename);

    let mut file = match File::open(path) {
      Ok(file) => file,
      Err(why) => {
        panic!("Shit happened! Couldn't load {}: {}!",
               self.filename,
               Error::description(&why));
      }
    };

    let mut content = String::new();
    file.read_to_string(&mut content);
    self.parsed = toml::Parser::new(&*content).parse();
  }

  pub fn is_initialized(&self) -> bool {
    self.parsed != None
  }

  // TODO: Add some getters/setters/save methods.
}
