#![allow(non_snake_case)]
use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Read;
use std::io::Write;

use marine_rs_sdk::{marine, module_manifest, WasmLoggerBuilder};

module_manifest!();

pub fn main() {
    WasmLoggerBuilder::new().build().unwrap();
}

#[marine]
pub fn calculate_pi(num_samples: u64) -> f64 {
    let mut pi = 0.0;
    let step = 1.0 / num_samples as f64;
    for i in 0..num_samples {
        let x = (i as f64 + 0.5) * step;
        pi += 4.0 / (1.0 + x * x);
    }
    pi *= step;
    pi
}

#[marine]
pub fn matrix_multiply(a: &Vec<Vec<f64>>, b: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let rows_a = a.len();
    let cols_a = a[0].len();
    let cols_b = b[0].len();
    let mut result = vec![vec![0.0; cols_b]; rows_a];

    for i in 0..rows_a {
        for j in 0..cols_b {
            for k in 0..cols_a {
                result[i][j] += a[i][k] * b[k][j];
            }
        }
    }
    result
}

#[marine]
pub fn fibonacci(n: u64) -> u64 {
    if n <= 1 {
        return n;
    }
    let mut a = 0;
    let mut b = 1;
    for _ in 2..=n {
        let temp = a + b;
        a = b;
        b = temp;
    }
    b
}

#[marine]
pub fn calculate_factorial(n: u64) -> u64 {
    if n <= 1 {
        return 1;
    }
    n * calculate_factorial(n - 1)
}

#[marine]
pub struct ReadFileResult {
    result: String,
    error: String,
    has_error: bool,
}

#[marine]
pub fn read_file(path: &str) -> ReadFileResult {
    let file = File::open(path);
    let file = match file {
        Ok(file) => file,
        Err(e) => {
            return ReadFileResult {
                result: "".to_string(),
                error: e.to_string(),
                has_error: true,
            }
        }
    };
    let mut reader = BufReader::new(file);
    let mut content = String::new();
    if let Err(e) = reader.read_to_string(&mut content) {
        ReadFileResult {
            result: "".to_string(),
            error: e.to_string(),
            has_error: true,
        }
    } else {
        ReadFileResult {
            result: content,
            error: "".to_string(),
            has_error: false,
        }
    }
}

#[marine]
pub struct WriteFileResult {
    error: String,
    has_error: bool,
}

#[marine]
pub fn write_file(path: &str, content: &str) -> WriteFileResult {
    let file = File::create(path);
    let file = match file {
        Ok(file) => file,
        Err(e) => {
            return WriteFileResult {
                error: e.to_string(),
                has_error: true,
            }
        }
    };
    let mut writer = BufWriter::new(file);

    if let Err(e) = writer.write_all(content.as_bytes()) {
        WriteFileResult {
            error: e.to_string(),
            has_error: true,
        }
    } else {
        WriteFileResult {
            error: "".to_string(),
            has_error: false,
        }
    }
}

#[marine]
pub struct ListFilesResult {
    result: Vec<String>,
    error: String,
    has_error: bool,
}

#[marine]
pub fn list_files_in_directory(path: &str) -> ListFilesResult {
    let entries = std::fs::read_dir(path);
    let entries = match entries {
        Ok(entries) => entries,
        Err(e) => {
            return ListFilesResult {
                result: vec![],
                error: e.to_string(),
                has_error: true,
            }
        }
    };
    let entries = entries
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .filter(|path| path.is_file())
        .filter_map(|path| path.to_str().map(|s| s.to_string()))
        .collect();
    ListFilesResult {
        result: entries,
        error: "".to_string(),
        has_error: false,
    }
}
