#![allow(non_snake_case)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::needless_return)]
mod json;
mod worksheet;
mod utils;

use std::collections::HashMap;

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
struct CategoryData {
    pub name: String,
    pub problem_ids: Vec<String>,
}

#[derive(Clone, PartialEq, Default)]
enum Route {
    #[default]
    Home,
    ProblemPage { problem_id: String },
}
type Router = Signal<Vec<Route>>;

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    launch(App);
}

fn App() -> Element {
    let router: Router = use_signal(|| vec![Route::Home]);
    rsx! {
        link { rel: "stylesheet", href: "main.css" }
        link { rel: "stylesheet", href: "block.css" }
        link { rel: "stylesheet", href: "worksheet.css" }
        match router.read().last().cloned().unwrap_or_default() {
            Route::Home => rsx! { Home { router } },
            Route::ProblemPage { problem_id } => rsx! { ProblemPage { router, problem_id } },
        }
    }
}


#[component]
fn Home(router: Router) -> Element {
    let categories: Result<Vec<CategoryData>, _> = serde_json::from_str(json::MAIN_MENU_DATA);
    let categories = categories.unwrap_or_default();
    let worksheet_data_map: HashMap<String, worksheet::WorksheetData> = serde_json::from_str(json::PROBLEMS_DATA_MAP).unwrap_or_default();
    let convert_mathvar = |s: String| utils::convert_mathvar(s);
    rsx! {
        div {
            class: "navbar",
            div {
                class: "navbar-center",
                img {
                    style: "height: 4em",
                    src: "equaio.png"
                }
                span {
                    class: "logo-span",
                    "EQUAIO"
                }
            }
        }
        div {
            class: "main-menu",
            for cat in categories {
                div {
                    class: "category-container",
                    div { 
                        class: "category-header",
                        span { "{cat.name}" }
                    }
                    for id in cat.problem_ids {
                        if let Some(ws_data) = worksheet_data_map.get(&id) {
                            div {
                                // to: Route::ProblemPage { problem_id: id.clone() },
                                class: "category-button",
                                onclick: move |_| { router.write().push(Route::ProblemPage { problem_id: id.clone() }); },
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
fn ProblemPage(router: Router, problem_id: String) -> Element {
    let problems_data_map: HashMap<String, worksheet::WorksheetData> = serde_json::from_str(json::PROBLEMS_DATA_MAP).unwrap_or_default();
    
    rsx! {
        div {
            class: "navbar",
            div {
                class: "navbar-left",
                button {
                    class: "navbar-button",
                    onclick: move |_| { router.write().pop(); },
                    "<"
                }
            }
        }
        if let Some(ws_data) = problems_data_map.get(&problem_id) {
            worksheet::Worksheet {
                ws_data: ws_data.clone(),
            }
        } else {
            div {
                "ERROR: problem not found"
            }
        }
    }
    
}