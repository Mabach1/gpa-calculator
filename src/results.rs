use anyhow::Error;
use std::{
    fs::{self, File},
    io::Write,
};
use thiserror::Error;

use crate::subject_result::SubjectResult;

#[derive(Debug)]
pub struct Results {
    pub gpa: f32,
    pub total_credits: u32,
    pub subject_results: Vec<SubjectResult>,
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

    #[error("Failed to create file: {filename}")]
    FailedToCreateFile { filename: String },

    #[error("Failed to write into file: {filename}")]
    FailedToWriteToFile { filename: String },
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

            results.push(result);
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

    fn recalculate_gpa(&mut self) {
        self.total_credits = 0;
        self.gpa = 0.0;

        for s in &self.subject_results {
            self.total_credits += s.credit;
            self.gpa += (s.credit * s.points) as f32;
        }

        if self.total_credits > 0 {
            self.gpa /= self.total_credits as f32;
        }
    }

    pub fn save_to_file(&self, filename: &str) -> Result<(), ResultsError> {
        let mut f = File::create(filename).map_err(|_| ResultsError::FailedToCreateFile {
            filename: filename.to_string(),
        })?;

        for s in &self.subject_results {
            f.write(format!("{}\n", s.to_string()).as_bytes())
                .map_err(|_| ResultsError::FailedToWriteToFile {
                    filename: filename.to_string(),
                })?;
        }

        Ok(())
    }

    pub fn expect_points(&self, target_gpa: f32, credit: u32) -> Option<u32> {
        let curr_total = self.gpa * self.total_credits as f32;
        let required = target_gpa * (self.total_credits + credit) as f32;
        let points_needed = ((required - curr_total) / credit as f32).ceil() as u32;
        Some(points_needed)
    }

    pub fn remove_result(&mut self, index: usize) -> Result<(), ResultsError> {
        if self.subject_results.len() <= index {
            return Err(ResultsError::OutOfBounds { idx: index });
        }

        self.subject_results.remove(index);
        self.recalculate_gpa();

        Ok(())
    }
}
