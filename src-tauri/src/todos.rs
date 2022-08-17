use rusqlite::{params, Connection, Error, Row};

pub struct Todo {
    pub id: Option<i32>,
    pub title: String,
    pub completed: bool,
    pub description: String,
}

pub struct TodoRepository<'a> {
    connection: &'a Connection,
}

fn map(row: &Row) -> Result<Todo, Error> {
    Ok(Todo {
        id: Some(row.get(0)?),
        title: row.get(1)?,
        completed: row.get(2)?,
        description: row.get(3)?,
    })
}

impl<'a> TodoRepository<'a> {
    pub fn new(connection: &'a Connection) -> Self {
        Self { connection }
    }

    pub fn get_all(&self) -> Result<Vec<Todo>, Error> {
        let mut stmt = self
            .connection
            .prepare("SELECT id, title, completed, description FROM T_TODOS")?;
        let todos = stmt.query_map(params![], map)?;
        let todos: Vec<Todo> = todos
            .map(|todo| todo.expect("error while getting todo"))
            .collect();
        Ok(todos)
    }

    pub fn get_by_id(&self, id: i32) -> Result<Todo, Error> {
        let mut stmt = self
            .connection
            .prepare("SELECT id, title, completed, description FROM T_TODOS WHERE id = ?")?;
        let todo = stmt.query_row(params![id], map)?;
        Ok(todo)
    }

    pub fn save(&self, todo: &Todo) -> Result<(), Error> {
        if todo.id.is_some() {
            let mut stmt = self.connection.prepare(
                "UPDATE T_TODOS SET title = ?, completed = ?, description = ? WHERE id = ?",
            )?;
            stmt.execute(params![
                todo.title,
                todo.completed,
                todo.description,
                todo.id.unwrap()
            ])?;
            return Ok(());
        }
        let mut stmt = self
            .connection
            .prepare("INSERT INTO T_TODOS (title, completed, description) VALUES (?, ?, ?)")?;
        stmt.execute(params![todo.title, todo.completed, todo.description])?;
        Ok(())
    }

    pub fn delete(&self, id: i32) -> Result<(), Error> {
        let mut stmt = self
            .connection
            .prepare("DELETE FROM T_TODOS WHERE id = ?")?;
        stmt.execute(params![id])?;
        Ok(())
    }
}
