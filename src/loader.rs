use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

use serde_json::Value;

pub fn load_json(path: impl Into<PathBuf>) -> Result<Value, String> {
    let mut s = String::new();
    File::open(path.into())
        .map_err(|e| format!("{}", e))?
        .read_to_string(&mut s)
        .map_err(|e| format!("{}", e))?;
    serde_json::from_str(&s).map_err(|e| format!("{}", e))
}
