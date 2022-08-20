use rusqlite::{params, Connection, Error, Row};
use serde::Serialize;
use tauri::{AppHandle, Manager};

use crate::connection::with_connection;

#[derive(Debug, Clone, Serialize)]
pub struct Todo {
    pub id: Option<i64>,
    pub title: String,
    pub completed: bool,
    pub description: String,
}

impl From<&Row<'_>> for Todo {
    fn from(row: &Row) -> Self {
        Todo {
            id: row.get(0).unwrap(),
            title: row.get(1).unwrap(),
            completed: row.get(2).unwrap(),
            description: row.get(3).unwrap(),
        }
    }
}

pub struct TodoRepository<'a> {
    connection: &'a Connection,
}

// fn map(row: &Row) -> Result<Todo, Error> {
//     let title: String = row.get(1)?;
//     let description: String = row.get(3)?;
//     Ok(Todo {
//         id: Some(row.get(0)?),
//         title: title.as_str(),
//         completed: row.get(2)?,
//         description: description.as_str(),
//     }
//     .clone())
// }

impl<'a> TodoRepository<'a> {
    pub fn new(connection: &'a Connection) -> Self {
        Self { connection }
    }

    pub fn get_all(&self) -> Result<Vec<Todo>, Error> {
        let mut stmt = self
            .connection
            .prepare("SELECT id, title, completed, description FROM T_TODOS")?;

        let todos: Result<Vec<Todo>, Error> = stmt
            .query_map(params![], |row| Ok(Todo::from(row)))?
            .collect();

        todos
    }

    pub fn get_by_id(&self, id: i64) -> Result<Todo, Error> {
        let mut stmt = self
            .connection
            .prepare("SELECT id, title, completed, description FROM T_TODOS WHERE id = ?")?;
        let todo = stmt.query_row(params![id], |row| Ok(Todo::from(row)));
        todo
    }

    pub fn save(&self, todo: &Todo) -> Result<Todo, Error> {
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

            let todo = self.get_by_id(todo.id.unwrap())?;
            return Ok(todo);
        }
        let mut stmt = self
            .connection
            .prepare("INSERT INTO T_TODOS (title, completed, description) VALUES (?, ?, ?)")?;
        let row_id = stmt.insert(params![todo.title, todo.completed, todo.description])?;
        let todo = self.get_by_id(row_id)?;
        Ok(todo)
    }

    pub fn delete(&self, id: i64) -> Result<(), Error> {
        let mut stmt = self
            .connection
            .prepare("DELETE FROM T_TODOS WHERE id = ?")?;
        stmt.execute(params![id])?;
        Ok(())
    }
}

#[tauri::command]
pub fn get_todos(app: AppHandle) {
    with_connection(|connection| {
        app.emit_all("todos", TodoRepository::new(connection).get_all()?)
            .unwrap();
        Ok(())
    })
}

#[tauri::command]
pub fn create_todo<'a>(app: AppHandle, title: &'a str, description: &'a str) {
    with_connection(|connection| {
        let todo_repository = TodoRepository::new(connection);
        let todo = Todo {
            id: None,
            title: title.to_string(),
            completed: false,
            description: description.to_string(),
        };
        let todo = todo_repository.save(&todo)?;
        app.emit_all("todo_created", todo).unwrap();
        Ok(())
    });
}

#[tauri::command]
pub fn delete_todo(app: AppHandle, id: i64) {
    with_connection(|connection| {
        let todo_repository = TodoRepository::new(connection);
        todo_repository.delete(id)?;
        app.emit_all("todo_deleted", id).unwrap();
        Ok(())
    });
}

#[tauri::command]
pub fn update_todo(app: AppHandle, id: i64, title: &str, description: &str) {
    with_connection(|connection| {
        let todo_repository = TodoRepository::new(connection);
        let todo = todo_repository.get_by_id(id)?;
        let todo = Todo {
            id: Some(id),
            title: title.to_string(),
            completed: todo.completed,
            description: description.to_string(),
        };
        let todo = todo_repository.save(&todo)?;
        app.emit_all("todo_updated", todo).unwrap();
        Ok(())
    });
}
