use std::collections::HashMap;
use std::path::PathBuf;
use crate::parser::Interface;

struct Context {
    root: PathBuf,
    cache: HashMap<String, (PathBuf, Interface)>
}