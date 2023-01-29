use std::error::Error;

use colored::*;

use clap::Command;
use term_table::row::Row;
use term_table::table_cell::TableCell;

use crate::database::Database;
mod database;

type TodoResult<T> = Result<T, Box<dyn Error>>;

pub fn run(command: Command) -> TodoResult<()> {
    let matches = command.get_matches();

    let mut todo_data = TodoData {
        operation: get_operation(&matches)?,
        database: Database::new()?,
        view: matches.get_flag("view"),
    };

    todo_data.perform_operation()?;
    Ok(())
}

fn get_operation(matches: &clap::ArgMatches) -> TodoResult<Operation> {
    let operation = match matches.subcommand() {
        Some(("add", args)) => {
            let info = args.get_one::<String>("task-value").unwrap();
            Operation::Add(info.to_owned())
        }
        Some(("done", args)) => {
            let index = args.get_one::<usize>("task-index").unwrap();
            Operation::Check(*index)
        }
        Some(("delete", args)) => {
            let index = args.get_one::<usize>("task-index").unwrap();
            Operation::Delete(*index)
        }
        _ => Operation::Empty,
    };
    Ok(operation)
}

enum Operation {
    Add(String),
    Check(usize),
    Delete(usize),
    Empty,
}

struct TodoData {
    operation: Operation,
    database: Database,
    view: bool,
}

impl TodoData {
    fn perform_operation(&mut self) -> TodoResult<()> {
        match &self.operation {
            Operation::Add(task) => self.database.create_new_task(&task)?,
            Operation::Check(index) => self.database.check_task(*index)?,
            Operation::Delete(index) => self.database.delete_task(*index)?,
            Operation::Empty => (),
        }
        self.construct_and_show_table();
        Ok(())
    }

    fn construct_and_show_table(&self) {
        if !self.view {
            return;
        }
        let mut table = term_table::Table::new();
        table.max_column_width = 40;

        table.style = term_table::TableStyle::thin();

        table.add_row(Row::new(vec![TableCell::new_with_alignment(
            "PTODO Tasks",
            4,
            term_table::table_cell::Alignment::Center,
        )]));

        table.add_row(Row::new(vec![
            TableCell::new("#".bold()),
            TableCell::new("[?]".bold()),
            TableCell::new("Task".bold()),
            TableCell::new("Created at".bold()),
        ]));

        let mut pending_tasks = 0;
        for (index, task) in self.database.get_tasks().iter().enumerate() {
            if !task.checked {
                pending_tasks += 1;
            }
            let check_status = if task.checked { "[✓]" } else { "[ ]" };
            if !task.checked {
                table.add_row(Row::new(vec![
                    TableCell::new(index + 1),
                    TableCell::new(check_status),
                    TableCell::new(&task.value),
                    TableCell::new(&task.date_created),
                ]));
            } else {
                table.add_row(Row::new(vec![
                    TableCell::new((index + 1).to_string().dimmed()),
                    TableCell::new(check_status.dimmed()),
                    TableCell::new((&task.value).dimmed()),
                    TableCell::new((&task.date_created).dimmed()),
                ]));
            }
        }

        table.add_row(Row::new(vec![TableCell::new_with_alignment(
            format!("Pending task(s) : {pending_tasks}"),
            4,
            term_table::table_cell::Alignment::Center,
        )]));

        println!("{}", table.render());
    }
}
