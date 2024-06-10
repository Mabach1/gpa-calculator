use thiserror::Error;

#[derive(Debug, PartialEq)]
pub enum Command {
    Add(CommandAdd),
    Show(CommandShow),
    Remove(CommandRemove),
    Import(CommandImport),
    Gpa(CommandGpa),
    Drop(CommandDrop),
    Save(CommandSave),
    Expect(CommandExpect),
    Quit,
    History,
    Illegal,
}

#[derive(Debug, Error)]
pub enum CommandError {
    #[error("Incorrect number of arguments: expected {expected} got {actual}")]
    IncorrectNumberOfArguments { expected: usize, actual: usize },

    #[error("Could not parse {value}")]
    ParseError { value: String },

    #[error("Unknown command: {value}")]
    UnknownCommand { value: String },

    #[error("Expected command, no command provided")]
    NoCommandProvided,
}

impl Command {
    pub fn new(command_lit: &str) -> Result<Self, CommandError> {
        let data: Vec<_> = command_lit.split(' ').collect();
        let command = match data.get(0) {
            Some(value) => value,
            _ => return Err(CommandError::NoCommandProvided),
        };
        let arguments = Vec::from(&data[1..]);

        if command == &"q" || command == &"quit" {
            return Ok(Command::Quit);
        }

        if command == &"h" || command == &"history" {
            return Ok(Command::History);
        }

        match command {
            &"add" => Ok(Command::Add(CommandAdd::new(arguments)?)),
            &"show" => Ok(Command::Show(CommandShow::new(arguments)?)),
            &"remove" => Ok(Command::Remove(CommandRemove::new(arguments)?)),
            &"import" => Ok(Command::Import(CommandImport::new(arguments)?)),
            &"gpa" => Ok(Command::Gpa(CommandGpa::new(arguments)?)),
            &"drop" => Ok(Command::Drop(CommandDrop::new(arguments)?)),
            &"save" => Ok(Command::Save(CommandSave::new(arguments)?)),
            &"expect" => Ok(Command::Expect(CommandExpect::new(arguments)?)),
            _ => Err(CommandError::UnknownCommand {
                value: command_lit.to_string(),
            }),
        }
    }
}

fn check_num_args(expected: usize, actual: usize) -> Result<(), CommandError> {
    if expected != actual {
        Err(CommandError::IncorrectNumberOfArguments { expected, actual })
    } else {
        Ok(())
    }
}

fn parse_int(str: &str) -> Result<u32, CommandError> {
    let str = str.to_string();

    match str.to_string().parse() {
        Ok(value) => Ok(value),
        _ => Err(CommandError::ParseError { value: str }),
    }
}

fn parse_float(str: &str) -> Result<f32, CommandError> {
    let str = str.to_string();

    match str.to_string().parse() {
        Ok(value) => Ok(value),
        _ => Err(CommandError::ParseError { value: str }),
    }
}

#[derive(Debug, PartialEq)]
pub struct CommandAdd {
    pub points: u32,
    pub credit: u32,
}

impl CommandAdd {
    fn new(literals: Vec<&str>) -> Result<Self, CommandError> {
        check_num_args(2, literals.len())?;

        Ok(Self {
            points: parse_int(literals.get(0).unwrap())?,
            credit: parse_int(literals.get(1).unwrap())?,
        })
    }
}

#[derive(Debug, PartialEq)]
pub struct CommandShow {}

impl CommandShow {
    fn new(literal: Vec<&str>) -> Result<Self, CommandError> {
        check_num_args(0, literal.len())?;
        Ok(Self {})
    }
}

#[derive(Debug, PartialEq)]
pub struct CommandRemove {
    pub index: usize,
}

impl CommandRemove {
    fn new(literals: Vec<&str>) -> Result<Self, CommandError> {
        check_num_args(1, literals.len())?;
        Ok(Self {
            index: parse_int(literals.get(0).unwrap())? as usize,
        })
    }
}

#[derive(Debug, PartialEq)]
pub struct CommandImport {
    pub filename: String,
}

impl CommandImport {
    fn new(literals: Vec<&str>) -> Result<Self, CommandError> {
        check_num_args(1, literals.len())?;
        Ok(Self {
            filename: literals.get(0).unwrap().to_string(),
        })
    }
}

#[derive(Debug, PartialEq)]
pub struct CommandGpa {}

impl CommandGpa {
    fn new(literals: Vec<&str>) -> Result<Self, CommandError> {
        check_num_args(0, literals.len())?;
        Ok(Self {})
    }
}

#[derive(Debug, PartialEq)]
pub struct CommandDrop {}

impl CommandDrop {
    fn new(literals: Vec<&str>) -> Result<Self, CommandError> {
        check_num_args(0, literals.len())?;
        Ok(Self {})
    }
}

#[derive(Debug, PartialEq)]
pub struct CommandSave {
    pub filename: String,
}

impl CommandSave {
    fn new(literals: Vec<&str>) -> Result<Self, CommandError> {
        check_num_args(1, literals.len())?;
        Ok(Self {
            filename: literals.get(0).unwrap().to_string(),
        })
    }
}

#[derive(Debug, PartialEq)]
pub struct CommandExpect {
    pub expected_gpa: f32,
    pub credit: u32,
}

impl CommandExpect {
    fn new(literals: Vec<&str>) -> Result<Self, CommandError> {
        check_num_args(2, literals.len())?;
        Ok(Self {
            expected_gpa: parse_float(literals.get(0).unwrap())?,
            credit: parse_int(literals.get(1).unwrap())?,
        })
    }
}
