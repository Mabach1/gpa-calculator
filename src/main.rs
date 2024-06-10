pub mod results;
pub mod subject_result;

use crate::results::Results;
use anyhow::Result;

fn main() -> Result<()> {
    let mut result = Results::from_file("results.txt")?;
    result.show_results();
    result.remove_result(4)?;
    result.show_results();
    Ok(())
}
