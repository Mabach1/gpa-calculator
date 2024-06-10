use anyhow::Error;
use std::fs;
use thiserror::Error;

use crate::subject_result::SubjectResult;

#[derive(Debug)]
pub struct Results {
    gpa: f32,
    total_credits: u32,
    subject_results: Vec<SubjectResult>,
}

#[derive(Debug, Error)]
pub enum ResultsError {
    #[error("Could not read file: {filename}")]
    CouldNotReadFile { filename: String },

    #[error("Incorrect numebr of arguments expected: {expected} got: {actual}")]
    IncorrectNumberOfArguments { expected: usize, actual: usize },

    #[error("Could not parse: {literal}")]
    ParseErorr { literal: String },

    #[error("Cannot remove result, index {idx} is out of bounds")]
    OutOfBounds { idx: usize },
}

impl Results {
    pub fn new(gpa: f32, total_credits: u32) -> Self {
        Self {
            gpa,
            total_credits,
            subject_results: vec![],
        }
    }

    pub fn from_string(str: &str) -> Result<Self, ResultsError> {
        let data: Vec<_> = str.split(' ').collect();

        if data.len() != 2 {
            return Err(ResultsError::IncorrectNumberOfArguments {
                expected: 2,
                actual: data.len(),
            });
        }

        let gpa = data
            .get(0)
            .unwrap()
            .parse()
            .map_err(|_| ResultsError::ParseErorr {
                literal: data.get(0).unwrap().to_string(),
            })?;

        let total_credits = data
            .get(1)
            .unwrap()
            .parse()
            .map_err(|_| ResultsError::ParseErorr {
                literal: data.get(1).unwrap().to_string(),
            })?;

        Ok(Self {
            gpa,
            total_credits,
            subject_results: Vec::new(),
        })
    }

    pub fn from_file(filename: &str) -> Result<Self, Error> {
        let data = fs::read_to_string(filename).map_err(|_| ResultsError::CouldNotReadFile {
            filename: filename.to_string(),
        })?;

        let lines: Vec<&str> = data.split("\n").collect();
        let lines: Vec<_> = lines.iter().filter(|str| str.len() != 0).collect();

        let mut results = Vec::new();
        let mut gpa = 0.0;
        let mut total_credits = 0;

        for line in &lines {
            let result = SubjectResult::from_string(line)?;

            total_credits += result.credit;
            gpa += result.credit as f32 * result.points as f32;

            results.push(SubjectResult::from_string(line)?);
        }

        if total_credits > 0 {
            gpa /= total_credits as f32;
        }

        Ok(Self {
            gpa,
            total_credits,
            subject_results: results,
        })
    }

    pub fn add_result(&mut self, result: SubjectResult) {
        self.total_credits += result.credit;

        self.gpa = ((self.gpa * ((self.total_credits - result.credit) as f32))
            + ((result.credit * result.points) as f32))
            / self.total_credits as f32;

        self.subject_results.push(result);
    }

    pub fn show_results(&self) {
        for (idx, result) in self.subject_results.iter().enumerate() {
            println!("  [{idx}] {}", result.to_string());
        }
    }

    pub fn remove_result(&mut self, index: usize) -> Result<(), ResultsError> {
        if self.subject_results.len() <= index {
            return Err(ResultsError::OutOfBounds { idx: index });
        }

        self.subject_results.remove(index);

        Ok(())
    }
}
