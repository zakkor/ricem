use json::JsonValue;
use std::path::PathBuf;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::*;
use std::env::home_dir;

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
    
    /// Takes a json file string and a json file path and returns a PathBuf with '~' expanded to the user's actual home.
    pub fn json_path_to_pathbuf(json_file: &JsonValue, json_path: &JsonValue) -> PathBuf {
        let (file, path) = (json_file.as_str().unwrap(), json_path.as_str().unwrap());
        
        if path.chars().nth(0).unwrap() == '~' {
            let track_string =
                path.clone()
                .to_string()
                .replace("~", home_dir().unwrap().to_str().unwrap());
            
            PathBuf::from(track_string).join(file)
        }
        else {    
            PathBuf::from(path).join(file)
        }
    }

}
