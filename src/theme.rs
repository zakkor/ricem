pub struct ConfigFile {
    pub name: String,
    pub path: String
}

impl ConfigFile {
    pub fn new(name: String, path: String) -> Self {
        ConfigFile {
            name: name,
            path: path
        }
    }
}

pub struct Theme {
    pub name: String,
    pub tracking: Vec<ConfigFile>,
}

impl Theme {
    pub fn new(name: String) -> Self {
        Theme {
            name: name,
            tracking: vec![]
        }
    }
}
