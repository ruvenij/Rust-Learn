mod arrays;
mod average;
mod cli;
mod conditionals;
mod crates;
mod enums;
mod functions;
mod loops;
mod pointer_ref;
mod print;
mod strings;
mod structs;
mod tuples;
mod types;
mod vars;
mod vectors;
mod generic;
mod traits;

fn main() {
    print::run();
    vars::run();
    types::run();
    strings::run();
    tuples::run();
    arrays::run();
    vectors::run();
    conditionals::run();
    loops::run();
    functions::run();
    pointer_ref::run();
    structs::run();
    enums::run();
    cli::run();
    average::find_average();
    //crates::guess_number();
    generic::run();
    traits::run();
}
