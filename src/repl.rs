use crate::{command::Command, results::Results, subject_result::SubjectResult};
use anyhow::Result;
use std::io::{self, stdout, Write};

pub struct Repl {
    history: Vec<String>,
    results: Results,
}

impl Repl {
    pub fn new() -> Self {
        Self {
            history: Vec::new(),
            results: Results::new(0.0, 0),
        }
    }

    fn execute_command(&mut self, command: Command) -> Result<()> {
        match command {
            Command::Add(value) => self
                .results
                .add_result(SubjectResult::new(value.points, value.credit)?),
            Command::Show(_) => self.results.show_results(),
            Command::Remove(c) => self.results.remove_result(c.index)?,
            Command::Import(c) => {
                self.results = Results::from_file(&c.filename)?;
            }
            Command::Drop(_) => {
                self.results = Results::new(0.0, 0);
            }
            Command::Gpa(_) => {
                println!("{}", self.results.gpa);
            }
            Command::History => {
                for el in &self.history {
                    println!("{}", el);
                }
            }
            _ => (),
        }

        Ok(())
    }

    pub fn run(&mut self) {
        loop {
            print!("> ");
            stdout().flush().unwrap();

            let mut input = String::new();

            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            input = input.trim().to_string();

            let command = match Command::new(&input) {
                Ok(c) => c,
                Err(e) => {
                    println!("Error: {}", e);
                    Command::Illegal
                }
            };

            self.history.push(input);

            if command == Command::Illegal {
                continue;
            }

            if command == Command::Quit {
                break;
            }

            match self.execute_command(command) {
                Err(err) => {
                    println!("Error: {}", err);
                }
                _ => (),
            }
        }
    }
}
