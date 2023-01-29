use std::error::Error;

use rusqlite::{params, Connection};

type DatabaseResult<T> = Result<T, Box<dyn Error>>;

pub struct Database {
    connection: Connection,
    items: Vec<TaskItem>,
}

#[derive(Debug)]
pub struct TaskItem {
    pub id: u32,
    pub value: String,
    pub checked: bool,
    pub date_created: String,
}

impl Database {
    pub fn new() -> DatabaseResult<Self> {
        let connection = Self::connect_to_database()?;
        let items = Vec::new();

        let mut database = Database { connection, items };
        database.populate_task_items()?;
        Ok(database)
    }

    pub fn create_new_task(&mut self, task: &str) -> DatabaseResult<()> {
        self.connection
            .execute("INSERT INTO tasks (task) VALUES (?1)", params![task])?;
        self.populate_task_items()?;
        Ok(())
    }

    pub fn check_task(&mut self, index: usize) -> DatabaseResult<()> {
        let task_item = self.items.get(index - 1).unwrap();
        self.connection.execute(
            "UPDATE tasks SET checked = 1 WHERE id = ?1",
            params![task_item.id],
        )?;
        self.populate_task_items()?;
        Ok(())
    }

    pub fn delete_task(&mut self, index: usize) -> DatabaseResult<()> {
        let task_item = self.items.get(index - 1).unwrap();
        self.connection
            .execute("DELETE FROM tasks WHERE id = ?1", params![task_item.id])?;
        self.populate_task_items()?;
        Ok(())
    }

    pub fn get_tasks(&self) -> &Vec<TaskItem> {
        &self.items
    }

    fn populate_task_items(&mut self) -> DatabaseResult<()> {
        self.items.clear();

        let mut stmt = self
            .connection
            .prepare("SELECT id,task,checked,dt FROM tasks ORDER BY dt")?;
        let mut result = stmt.query([])?;

        while let Some(row) = result.next()? {
            let check: u32 = row.get(2)?;
            let task_item = TaskItem {
                id: row.get(0)?,
                value: row.get(1)?,
                checked: check != 0,
                date_created: row.get(3)?,
            };
            self.items.push(task_item);
        }
        Ok(())
    }

    fn connect_to_database() -> DatabaseResult<Connection> {
        let conn = Connection::open("./data.db")?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS tasks(
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            task TEXT NOT NULL,
            checked INTEGER DEFAULT 0 NOT NULL,
            dt datetime DEFAULT current_timestamp
        )",
            (),
        )?;
        Ok(conn)
    }
}
