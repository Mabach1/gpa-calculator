pub mod command;
pub mod repl;
pub mod results;
pub mod subject_result;

use repl::Repl;

fn main() {
    let mut repl = Repl::new();
    repl.run();
}
