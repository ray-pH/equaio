#![allow(non_snake_case)]
mod json;
mod worksheet;
mod utils;

use std::collections::HashMap;

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};
use equaio::vec_strings;
use serde::{Deserialize, Serialize};

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/problems/:problem_id")]
    ProblemPage { problem_id: String },
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
struct CategoryData {
    pub name: String,
    pub problem_ids: Vec<String>,
}

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    launch(App);
}

fn App() -> Element {
    rsx! {
        link { rel: "stylesheet", href: "main.css" }
        link { rel: "stylesheet", href: "block.css" }
        link { rel: "stylesheet", href: "worksheet.css" }
        Router::<Route> {}
    }
}


#[component]
fn Home() -> Element {
    let categories: Result<Vec<CategoryData>, _> = serde_json::from_str(json::MAIN_MENU_DATA);
    let categories = categories.unwrap_or_default();
    let worksheet_data_map: HashMap<String, worksheet::WorksheetData> = serde_json::from_str(json::PROBLEMS_DATA_MAP).unwrap_or_default();
    let convert_mathvar = |s: String| utils::convert_mathvar(s);
    rsx! {
        div {
            id: "main",
            for cat in categories {
                div {
                    class: "category-container",
                    div { 
                        class: "category-header",
                        span { "{cat.name}" }
                    }
                    for id in cat.problem_ids {
                        if let Some(ws_data) = worksheet_data_map.get(&id) {
                            Link {
                                to: Route::ProblemPage { problem_id: id.clone() },
                                class: "category-button",
                                onclick: move |_| { },
                                span { "{ws_data.label.clone()}" }
                                span { 
                                    class: "problem-sublabel",
                                    "{convert_mathvar(ws_data.sublabel.clone().unwrap_or_default())}" 
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ProblemPage(problem_id: String) -> Element {
    let problems_data_map: HashMap<String, worksheet::WorksheetData> = serde_json::from_str(json::PROBLEMS_DATA_MAP).unwrap_or_default();
    if let Some(ws_data) = problems_data_map.get(&problem_id) {
        rsx! {
            link { rel: "stylesheet", href: "../main.css" }
            link { rel: "stylesheet", href: "../block.css" }
            link { rel: "stylesheet", href: "../worksheet.css" }
            worksheet::Worksheet {
                ws_data: ws_data.clone(),
            }
        }
    } else {
        rsx! {
            link { rel: "stylesheet", href: "../main.css" }
            div {
                "ERROR: problem not found"
            }
        }
    }
}