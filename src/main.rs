use clap::{command, value_parser, Arg, Command};

fn main() {
    if let Err(e) = ptodo::run(construct_command()) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

fn construct_command() -> Command {
    command!()
        .name("ptodo")
        .about("POORMAN'S TASK MANAGER")
        .long_about("Simple CLI tool for managing tasks. Create/Done/Delete tasks. Integrated with sqlite.") 
        .version("0.1.0")
        .author("kishore <kishore_nx@protonmail.com>")
        .arg_required_else_help(true)
        .arg(Arg::new("view").short('v').required(false).num_args(0))
        .subcommand(
            Command::new("add").about("Adds a new task").arg(
                Arg::new("task-value")
                    .required(true)
                    .value_parser(value_parser!(String)),
            ),
        )
        .subcommand(
            Command::new("done")
                .about("Complete the task at the specified index")
                .arg(
                    Arg::new("task-index")
                        .required(true)
                        .value_parser(value_parser!(usize)),
                ),
        )
        .subcommand(
            Command::new("delete")
                .about("Deletes the task at the specified index")
                .arg(
                    Arg::new("task-index")
                        .required(true)
                        .value_parser(value_parser!(usize)),
                ),
        )
}
