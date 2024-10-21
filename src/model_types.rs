use std::collections::HashMap;


pub(crate) struct Defaults {
    pub values: HashMap<String, String>,
}

pub(crate) struct RcFile {
    pub defaults: Option<Defaults>,
}