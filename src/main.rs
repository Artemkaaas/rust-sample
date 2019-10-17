extern crate serde_json;

use std::thread;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::sync::{Mutex, Arc};

fn main() -> Result<(), SampleError> {
    let files: Vec<String> = env::args().skip(1).collect();

    let common_config: Arc<Mutex<HashMap<String, String>>> = Arc::new(Mutex::new(HashMap::new()));

    files
        .into_iter()
        .map(|file| {
            let copy_ref = common_config.clone();
            thread::spawn(move || {
                process_file(&file, copy_ref)
            })
        })
        .map(|handle| handle.join().expect("Cannot join Thread"))
        .collect::<Result<(), SampleError>>()?;

    println!("Result: {:?}", common_config.lock()?);

    Ok(())
}

fn process_file(file: &str, common_config: Arc<Mutex<HashMap<String, String>>>) -> Result<(), SampleError> {
    let content: String = fs::read_to_string(file)?;
    let json: serde_json::Value = serde_json::from_str(&content)?;
    process_json(&json, common_config)
}

fn process_json(config: &serde_json::Value, values: Arc<Mutex<HashMap<String, String>>>) -> Result<(), SampleError> {
    for (key, value) in config.as_object().ok_or(SampleError::ConfigValueError)?.iter() {
        match value {
            serde_json::Value::String(value) => {
                values.lock()?.insert(key.to_string(), value.to_string());
            }
            obj_val @ serde_json::Value::Object(_) => {
                process_json(obj_val, values.clone())?;
            }
            _ => {
                // handle other values here (like numbers, arrays, ...)
                return Err(SampleError::ConfigValueError);
            }
        }
    }
    Ok(())
}

#[derive(Debug)]
enum SampleError {
    FileReadingError(String),
    FileParingError(String),
    LockError,
    ConfigValueError
}

impl From<std::io::Error> for SampleError {
    fn from(err: std::io::Error) -> Self {
        SampleError::FileReadingError(err.to_string())
    }
}

impl From<serde_json::error::Error> for SampleError {
    fn from(err: serde_json::error::Error) -> Self {
        SampleError::FileParingError(err.to_string())
    }
}

impl<T> From<std::sync::PoisonError<T>> for SampleError {
    fn from(_: std::sync::PoisonError<T>) -> Self {
        SampleError::LockError
    }
}