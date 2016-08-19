use json::JsonValue;
use std::path::PathBuf;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::*;

extern crate json;

pub struct JsonUtil<'a>(&'a PathBuf);

impl<'a> JsonUtil<'a> {
    pub fn new(conf_path: &'a PathBuf) -> Self {
        JsonUtil(conf_path)
    }
    
    pub fn read(&self) -> JsonValue {
        let mut file = OpenOptions::new()
            .read(true)
            .open(&self.0)
            .unwrap();
        
        let mut data = String::new();
        file.read_to_string(&mut data).expect("Failed to read JSON config file in ~/.ricem/.conf");
        
        json::parse(&data).unwrap()
    }

    pub fn write(&self, json_obj: &JsonValue) {
        let mut file = OpenOptions::new()
            .write(true)
            .open(&self.0)
            .unwrap();
        
        file.set_len(0).expect("Could not edit JSON config file in ~/.ricem/.conf");
        file.write_fmt(format_args!("{:#}", json_obj)).expect("Could not write to JSON config file in ~/.ricem/.conf");
    }
}
