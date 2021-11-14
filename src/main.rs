#![feature(proc_macro_hygiene, decl_macro)]

use rocket::*;
use rusqlite::Connection;

struct ToDoList {
    items: Vec<ToDoItem>,
}

struct ToDoItem {
    id: i64,
    item: String,
}

struct StatusMessage {
    message: String,
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    {
        let db_connection = Connection::open("data.sqlite").unwrap();

        db_connection
            .execute(
                "create table if not exists todo_list (
                    id integer primary key
                    item varchar(64) no null
                );",
                rusqlite::NO_PARAMS,
            )
            .unwrap();
    }
    rocket::ignite().mount("/", routes![index]).launch();
}
