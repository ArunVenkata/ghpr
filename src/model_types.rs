use std::collections::HashMap;

pub struct Defaults {
    pub values: HashMap<String, String>,
}

pub struct RcFile {
    pub defaults: Option<Defaults>,
}
