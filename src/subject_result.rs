use thiserror::Error;

#[derive(Debug)]
pub struct SubjectResult {
    pub points: u32,
    pub credit: u32,
}

#[derive(Debug, Error)]
pub enum SubjectResultError {
    #[error("Incorrect number of arguments expected: {expected} got: {actual}")]
    IncorrectNumberOfArguments { expected: usize, actual: usize },

    #[error("Could not parse {value} to u32")]
    CouldNotParse { value: String },
}

impl SubjectResult {
    pub fn new(points: u32, credit: u32) -> Self {
        Self { points, credit }
    }

    fn from_str_to_int(str: &str) -> Result<u32, SubjectResultError> {
        let str = str.to_string();

        match str.to_string().parse() {
            Ok(value) => Ok(value),
            _ => Err(SubjectResultError::CouldNotParse { value: str }),
        }
    }

    pub fn from_string(str: &str) -> Result<Self, SubjectResultError> {
        let data: Vec<_> = str.split(' ').collect();

        if data.len() != 2 {
            return Err(SubjectResultError::IncorrectNumberOfArguments {
                expected: 2,
                actual: data.len(),
            });
        }

        let points = SubjectResult::from_str_to_int(data.get(0).unwrap())?;
        let credit = SubjectResult::from_str_to_int(data.get(1).unwrap())?;

        Ok(Self { points, credit })
    }

    pub fn to_string(&self) -> String {
        format!("{} {}", self.points, self.credit)
    }
}
