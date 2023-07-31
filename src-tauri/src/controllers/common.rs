use poem::{handler, IntoResponse, web::{Data, Query}};
use crate::models::{State, IndexParams};
use crate::services::emit_greet_event;

#[handler]
pub fn server_index(
    data: Data<&State>,
    Query(params): Query<IndexParams>,
) -> impl IntoResponse {
    let main_window = &data.0.main_window;
    let name = params.name.unwrap_or(String::from(""));

    match emit_greet_event(main_window, name.as_str()) {
        Ok(_) => {
            String::from("success")
        },
        _ => {
            String::from("Event emit failed")
        }
    }
}