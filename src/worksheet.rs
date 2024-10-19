use super::json;
use dioxus::prelude::*;
use dioxus_logger::tracing::info;
use equaio;
use equaio::block::Block;
use equaio::expression::Address;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct WorksheetData {
    pub label: String,
    pub sublabel: Option<String>,
    pub rule: String,
    pub variables: Vec<String>,
    pub initial_expressions: Vec<String>,
}

fn get_algebra_ruleset(_auto_simplify: bool) -> equaio::rule::RuleSet {
    let rulestr = json::ALGEBRA_RULES;
    let ruleset = equaio::rule::parse_ruleset_from_json(&rulestr);
    return ruleset.unwrap();
}
fn init_algebra_worksheet(variables: Vec<String>, auto_simplify: bool) -> equaio::worksheet::Worksheet {
    let mut ws = equaio::worksheet::Worksheet::new();
    let ruleset = get_algebra_ruleset(auto_simplify);
    let ctx = equaio::arithmetic::get_arithmetic_ctx().add_params(variables);
    ws.set_ruleset(ruleset);
    ws.set_expression_context(ctx);
    ws.set_normalization_function(|expr,ctx| expr.normalize_algebra(ctx));
    ws.set_get_possible_actions_function(|expr,ctx,addr_vec| 
        equaio::algebra::get_possible_actions::algebra(expr,ctx,addr_vec));
    return ws;
}

#[component]
pub fn Worksheet(ws_data: WorksheetData) -> Element {
    //TODO: separate out the initialization of the worksheet from the rendering
    let ctx = equaio::arithmetic::get_arithmetic_ctx().add_params(ws_data.variables.clone()); // debug
    let mut ws = use_signal(|| init_algebra_worksheet(ws_data.variables, false));
    for expr_str in ws_data.initial_expressions {
        let expr = equaio::parser::parser::to_expression(expr_str, &ctx);
        if let Some(expr) = expr { ws.write().introduce_expression(expr); }
    }
    let seq = ws.write().get(0).unwrap(); // debug
    let expressions = seq.history.iter().map(|line| &line.expr).collect::<Vec<_>>();
    let block_ctx = equaio::block::BlockContext::default();
    let indexed_blocks = expressions.iter()
        .enumerate().map(|(i,expr)| (i, Block::from_root_expression(expr, &block_ctx))).collect::<Vec<_>>();
    let last_index = indexed_blocks.len() - 1;
    
    let mut active_address = use_signal(|| Vec::<Address>::new());
    let possible_actions = seq.get_possible_actions(&active_address.read());
    
    let possible_actions_presentable = possible_actions.iter().enumerate()
        .map(|(i,(action, expr))| (i, action.to_string(), Block::from_root_expression(expr, &block_ctx)))
        .collect::<Vec<_>>();
    
    rsx!(
        div {
        class: "expression-sequence-container",
        for (i, block) in indexed_blocks {
            Block {
                block: block, 
                active_address: if i == last_index { Some(active_address) } else { None },
                on_address_update: move |(addr, bool)| {
                    if bool {
                        if !active_address.read().contains(&addr) { active_address.write().push(addr); }
                    } else {
                        if active_address.read().contains(&addr) { active_address.write().retain(|a| a != &addr); }
                    }
                }
            }
        }
        div {
            class: "possible-actions-container",
            for (i, action, block) in possible_actions_presentable {
                div {
                    class: "possible-action-button",
                    onclick: move |_| {
                        info!("applying action {}", i);
                        let mut seq = ws.write().get(0).unwrap(); // debug
                        seq.try_apply_action_by_index(&active_address.read(), i);
                        ws.write().store(i, seq);
                        active_address.write().clear();
                        ws.origin_scope().needs_update();
                    },
                    span { "{action}" }
                    Block { block, active_address: None, on_address_update: |_| {} }
                }
            }
        }
    })
}

#[component]
fn Block(block: Block, active_address: Option<Signal<Vec<Address>>>, on_address_update: EventHandler<(Address, bool)>) -> Element {
    use equaio::block::BlockType;
    match block.block_type {
        BlockType::Symbol => {
            let is_clickable = active_address.is_some();
            let is_active = is_clickable && active_address.unwrap().read().contains(&block.address);
            let mut classlist = vec!["block-symbol"];
            if is_clickable { classlist.push("clickable"); }
            if is_active { classlist.push("active"); }
            return rsx! {
                div {
                    class: classlist.join(" "),
                    onclick: move |_| if is_clickable { 
                        on_address_update.call((block.address.clone(), !is_active))
                    },
                    "{block.symbol.unwrap_or_default()}"
                }
            }
        }
        BlockType::HorizontalContainer => {
            let children = block.children.unwrap_or_default();
            return rsx! {
                div {
                    class: "block-horizontal",
                    for child in children {
                        Block { block: child, active_address, on_address_update }
                    }
                }
            };
        }
        BlockType::FractionContainer => {
            return rsx! { span { "todo: FractionContainer"} }
        }
    }
}