use std::sync::Mutex;

use once_cell::sync::Lazy;
use rusqlite::{params, Connection, Result};

#[derive(Clone, PartialEq, Debug)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub tag: String,
    pub status: String,
}

pub static DB_CONN: Lazy<Mutex<Connection>> = Lazy::new(|| {
    let conn = Connection::open("todo_app.db").expect("Failed to open database");
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tasks (
            id     INTEGER PRIMARY KEY AUTOINCREMENT,
            title  TEXT NOT NULL,
            tag    TEXT NOT NULL,
            status TEXT NOT NULL
        )",
        [],
    )
    .expect("Failed to create table");
    Mutex::new(conn)
});

pub fn insert_task(task: &Task) -> Result<()> {
    DB_CONN.lock().unwrap().execute(
        "INSERT INTO tasks (title, tag, status) VALUES (?1, ?2, ?3)",
        params![task.title, task.tag, task.status],
    )?;
    Ok(())
}

pub fn get_all_tasks() -> Result<Vec<Task>> {
    let conn = DB_CONN.lock().unwrap();
    let mut stmt = conn.prepare("SELECT id, title, tag, status FROM tasks")?;
    let iter = stmt.query_map([], |row| {
        Ok(Task {
            id: row.get(0)?,
            title: row.get(1)?,
            tag: row.get(2)?,
            status: row.get(3)?,
        })
    })?;

    let mut result = vec![];
    for task in iter {
        result.push(task?);
    }
    Ok(result)
}

pub fn update_task_status(id: i32, new_status: &str) -> Result<()> {
    DB_CONN.lock().unwrap().execute(
        "UPDATE tasks SET status = ?1 WHERE id = ?2",
        params![new_status, id],
    )?;
    Ok(())
}

pub fn delete_task(id: i32) -> Result<()> {
    DB_CONN
        .lock()
        .unwrap()
        .execute("DELETE FROM tasks WHERE id = ?1", params![id])?;
    Ok(())
}
