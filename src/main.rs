#![feature(proc_macro_hygiene, decl_macro)]

use rocket::*;
use rocket_contrib::json::Json;
use rusqlite::Connection;
use serde::Serialize;

#[derive(Serialize)]
struct ToDoList {
    items: Vec<ToDoItem>,
}

#[derive(Serialize)]
struct ToDoItem {
    id: i64,
    item: String,
}

#[derive(Serialize)]
struct StatusMessage {
    message: String,
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/todo")]
fn fetch_all_todos() -> Result<Json<ToDoList>, String> {
    // Using match to see if there is a connection to the SQLite database
    // returning an error statement if there isn't
    let db_connnection = match Connection::open("data.sqlite") {
        Ok(connection) => connection,
        Err(_) => {
            return Err(String::from("Failed to connect to database"));
        }
    };

    // This prepares an SQL statement for execution
    let mut statement = match db_connnection.prepare("select id, item from todo_list;") {
        Ok(statement) => statement,
        Err(_) => return Err("Failed to prepare query".into()),
    };

    // This executes the statement prepared and maps a function over the resulting rows
    let results = statement.query_map([], |row| {
        Ok(ToDoItem {
            // row.get() gets the value from the result row
            id: row.get(0)?,
            item: row.get(1)?,
        })
    });

    match results {
        Ok(rows) => {
            // Turns the iterated result into a collection
            let collection: rusqlite::Result<Vec<_>> = rows.collect();

            // If there is a collection...
            match collection {
                // Pass them into the ToDoList struct as JSON data
                Ok(items) => Ok(Json(ToDoList { items })),
                Err(_) => Err("Could not collect items".into()),
            }
        }

        Err(_) => Err("Failed to fetch todo items".into()),
    }
}

#[post("/todo", format = "json", data = "<item>")]
fn add_todo_item(item: Json<String>) -> Result<Json<StatusMessage>, String> {
    // Using match to see if there is a connection to the SQLite database
    // returning an error statement if there isn't
    let db_connnection = match Connection::open("data.sqlite") {
        Ok(connection) => connection,
        Err(_) => {
            return Err(String::from("Failed to connect to database"));
        }
    };

    // This prepares an SQL statement for execution
    let mut statement =
        match db_connnection.prepare("insert into todo_list (id, item) values (null, $1);") {
            Ok(statement) => statement,
            Err(_) => return Err("Failed to prepare query".into()),
        };

    let results = statement.execute(&[&item.0]);

    match results {
        Ok(rows_affected) => Ok(Json(StatusMessage {
            message: format!("{} rows inserted!", rows_affected),
        })),
        Err(_) => Err("Failed to fetch todo items".into()),
    }
}

#[delete("/todo/<id>")]
fn remove_todo_item(id: i64) -> Result<Json<StatusMessage>, String> {
    let db_connection = match Connection::open("data.sqlite") {
        Ok(connection) => connection,
        Err(_) => {
            return Err(String::from("Failed to connect to database"));
        }
    };

    let mut statement = match db_connection.prepare("delete from todo_list where id = $1;") {
        Ok(statement) => statement,
        Err(_) => return Err("Failed to prepare query".into()),
    };
    let results = statement.execute(&[&id]);

    match results {
        Ok(rows_affected) => Ok(Json(StatusMessage {
            message: format!("{} rows deleted!", rows_affected),
        })),
        Err(_) => Err("Failed to delete todo item".into()),
    }
}

fn main() {
    {
        // Opens a new connection to the SQLite database and assigns it to the
        // db_connection variable.
        let db_connection = Connection::open("data.sqlite").unwrap();

        // Execute method is called to execute the SQL statement.
        db_connection
            .execute(
                "create table if not exists todo_list (
                    id integer primary key,
                    item varchar(64) not null
                );",
                [],
            )
            .unwrap();
    }

    // This creates a new rocket instance and mounts the index route to the "/" base path,
    // making rocket aware of the route.
    rocket::ignite()
        .mount("/", routes![index, fetch_all_todos, add_todo_item, remove_todo_item])
        .launch();
}
