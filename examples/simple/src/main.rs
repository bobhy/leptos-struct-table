use crate::uuid::Uuid;
use async_trait::async_trait;
use chrono::NaiveDate;
use leptos::*;
use leptos_struct_table::*;
use serde::{Deserialize, Serialize};

// This generates the component BookTable
#[derive(TableComponent, Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
#[table(sortable)]
pub struct Book {
    #[table(key)]
    pub id: Uuid,
    pub title: String,
    pub author: String,
    pub publish_date: NaiveDate,
    #[table(skip)]
    pub hidden_field: String,
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to_body(|cx| {
        let items = create_rw_signal(
            cx,
            vec![
                Book {
                    id: Uuid::default(),
                    title: "The Great Gatsby".to_string(),
                    author: "F. Scott Fitzgerald".to_string(),
                    publish_date: NaiveDate::from_ymd_opt(1925, 4, 10).unwrap(),
                    hidden_field: "hidden".to_string(),
                },
                Book {
                    id: Uuid::default(),
                    title: "The Grapes of Wrath".to_string(),
                    author: "John Steinbeck".to_string(),
                    publish_date: NaiveDate::from_ymd_opt(1939, 4, 14).unwrap(),
                    hidden_field: "not visible in the table".to_string(),
                },
                Book {
                    id: Uuid::default(),
                    title: "Nineteen Eighty-Four".to_string(),
                    author: "George Orwell".to_string(),
                    publish_date: NaiveDate::from_ymd_opt(1949, 6, 8).unwrap(),
                    hidden_field: "hidden".to_string(),
                },
                Book {
                    id: Uuid::default(),
                    title: "Ulysses".to_string(),
                    author: "James Joyce".to_string(),
                    publish_date: NaiveDate::from_ymd_opt(1922, 2, 2).unwrap(),
                    hidden_field: "hidden".to_string(),
                },
            ],
        );

        view! { cx,
            <BookTable items=items />
        }
    })
}
