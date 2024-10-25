use super::json;
use super::utils;
use std::collections::HashMap;
use dioxus::prelude::*;
use equaio::block::Block;
use equaio::expression::Address;
use equaio::{pair_map, vec_strings, vec_index_map};
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct WorksheetData {
    pub label: String,
    pub sublabel: Option<String>,
    pub rule: String,
    pub variables: Vec<String>,
    pub initial_expressions: Vec<String>,
}

fn get_ruleset(rulename: String) -> equaio::rule::RuleSet {
    let rulestr = match rulename.as_str() {
        "algebra" => json::ALGEBRA_RULES,
        "algebra_simplify" => json::ALGEBRA_SIMPLIFY_RULES,
        _ => json::ALGEBRA_RULES // TODO: change the default
    };
    let ruleset = equaio::rule::parse_ruleset_from_json(rulestr);
    return ruleset.unwrap();
}
fn init_worksheet(ws_data: WorksheetData) -> equaio::worksheet::Worksheet {
    let ruleset = get_ruleset(ws_data.rule);
    let mut ws = equaio::worksheet::Worksheet::new();
    ws.set_ruleset(ruleset);
    // TODO: load general normalization and possible actions functions
    ws.set_normalization_function(|expr,ctx| expr.normalize_algebra(ctx));
    ws.set_get_possible_actions_function(|expr,ctx,addr_vec| 
        equaio::algebra::get_possible_actions::algebra(expr,ctx,addr_vec));
    
    let ctx = ws.get_expression_context().add_params(ws_data.variables);
    for expr_str in ws_data.initial_expressions {
        let expr = equaio::parser::parser::to_expression(expr_str, &ctx);
        if let Some(expr) = expr { ws.introduce_expression(expr); }
    }
    return ws;
}

#[component]
pub fn Worksheet(ws_data: WorksheetData) -> Element {
    let ws = use_signal(|| init_worksheet(ws_data));
    
    rsx! {
        div {
            class: "worksheet",
            for i in 0..ws.read().len() {
                if let Some(seq) = ws.read().get(i) {
                    ExpressionSequence { seq, seq_index: 0, ws }
                }
            }
        }
    }
    
}

type AlignableBlocks = (Option<Block>, Option<Block>, Option<Block>);

#[derive(PartialEq, Clone)]
struct GroupedHistory {
    pub history: Vec<equaio::worksheet::ExpressionLine>,
    pub line_index: usize, // index of the first line in the history
}
impl GroupedHistory {
    pub fn new(history: Vec<equaio::worksheet::ExpressionLine>, line_index: usize) -> Self {
        GroupedHistory { history, line_index }
    }
    // action_str, alignable_blocks
    pub fn to_block_data(&self, block_ctx: &equaio::block::BlockContext) -> Vec<(String, AlignableBlocks)> {
        self.history.iter().map(|line| {
            let action_str = line.action.to_string();
            let alignable_blocks = Block::from_root_expression_to_alignable_blocks(&line.expr, block_ctx);
            (action_str, alignable_blocks)
        }).collect::<Vec<_>>()
    }
}

fn group_auto_history(history: Vec<equaio::worksheet::ExpressionLine>) -> Vec<GroupedHistory> {
    let mut history_grouped = vec![];
    let mut current_group = vec![];
    let mut current_group_index = 0;
    for (i,line) in history.iter().enumerate() {
        if line.is_auto_generated {
            current_group.push(line.clone());
        } else {
            if !current_group.is_empty() { history_grouped.push(GroupedHistory::new(current_group, current_group_index)); }
            current_group = vec![line.clone()];
            current_group_index = i;
        }
    }
    
    if !current_group.is_empty() {  history_grouped.push(GroupedHistory::new(current_group, current_group_index)); }
    return history_grouped;
}

#[component]
pub fn ExpressionSequence(
    seq: equaio::worksheet::WorkableExpressionSequence,  
    seq_index: usize,  ws: Signal<equaio::worksheet::Worksheet>
)  -> Element 
{
    //TODO: load from json
    let block_ctx = equaio::block::BlockContext {
        inverse_ops: pair_map![("+", "-"), ("*", "/")],
        fraction_ops: vec_strings!["/"],
        conceal_ops: vec_strings!["*"],
        op_precedence: vec_index_map!["-", "+", "/", "*"]
    };
    
    let grouped_history = group_auto_history(seq.history.clone());
    let last_index = grouped_history.len() - 1;
    
    let mut active_address = use_signal(|| Vec::<Address>::new());
    let possible_actions = seq.get_possible_actions(&active_address.read());
    
    let possible_actions_presentable = possible_actions.iter().enumerate()
        .map(|(i,(action, expr))| (i, action.to_string(), Block::from_root_expression(expr, &block_ctx)))
        .collect::<Vec<_>>();
    
    #[allow(clippy::collapsible_else_if)]
    let address_update_handler: EventHandler<(Address, bool)> = EventHandler::new(move |(addr, bool)| {
        // bool determines whether to add or remove the address from the active addresses
        if bool {
            if !active_address.read().contains(&addr) { active_address.write().push(addr); }
        } else {
            if active_address.read().contains(&addr) { active_address.write().retain(|a| a != &addr); }
        }
    });
    
    rsx!( div {
        class: "expression-sequence-container",
        div {
            class: "expression-sequence-history-container",
            for (i, group) in grouped_history.iter().enumerate() {
                if !group.history.is_empty() {
                    GroupedHistoryBlock {
                        group: group.clone(), 
                        is_first: i == 0, is_last: i == last_index,
                        active_address,
                        block_ctx: block_ctx.clone(),
                        address_update_handler, ws, seq_index
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
                        let mut seq = ws.write().get(seq_index).unwrap();
                        seq.try_apply_action_by_index(&active_address.read(), i);
                        ws.write().store(seq_index, seq);
                        active_address.write().clear();
                    },
                    span { class:"possible-action-caption" , "{action}" }
                    Block { block, active_address: None, on_address_update: |_| {} }
                }
            }
        }
    })
}

#[component]
fn GroupedHistoryBlock(
    group: GroupedHistory, 
    is_first: bool, is_last: bool,
    active_address: Signal<Vec<Address>>,
    block_ctx: equaio::block::BlockContext,
    address_update_handler: EventHandler<(Address, bool)>,
    seq_index: usize,
    ws: Signal<equaio::worksheet::Worksheet>
) -> Element 
{
    let is_expanded = use_signal(|| false);
    let group_data = group.to_block_data(&block_ctx);
    let (first_action_str, _) = group_data.first().unwrap().clone();
    let (last_action_str, (last_lhs, last_mid, last_rhs)) = group_data.last().unwrap().clone();
    let is_multiline = group_data.len() > 1;
    let last_line_index = group.line_index + group_data.len() - 1;
    
    rsx! {
        if *is_expanded.read() {
            for (i, (action_str, (lhs, mid, rhs))) in group_data.iter().enumerate().take(group_data.len() - 1){
                ExpressionLine {
                    is_first, is_last: false, is_multiline: false,
                    action_str, 
                    lhs: lhs.clone(), mid: mid.clone(), rhs: rhs.clone(),
                    line_index: group.line_index + i,
                    active_address, is_expanded, address_update_handler, // unused props
                    ws, seq_index
                }
            }
        } 
        ExpressionLine {
            is_first, is_last, is_multiline,
            action_str: if *is_expanded.read() { last_action_str } else { first_action_str },
            lhs: last_lhs, mid: last_mid, rhs: last_rhs,
            line_index: last_line_index,
            active_address, is_expanded, address_update_handler, ws, seq_index
        }
        
    }
}

#[component]
fn ExpressionLine(
    is_first: bool, is_last: bool, is_multiline: bool,
    action_str: String, lhs: Option<Block>, mid: Option<Block>, rhs: Option<Block>,
    active_address: Signal<Vec<Address>>,
    is_expanded: Signal<bool>,
    address_update_handler: EventHandler<(Address, bool)>,
    line_index: usize, seq_index: usize,
    ws: Signal<equaio::worksheet::Worksheet>
) -> Element {
    rsx!{
        if !is_first {
            div {
                class: "expression-line-gap"
            }
            div {
                class: "expression-line-left-bar"
            }
            div {
                class: "expression-line-action",
                "{action_str}"
                if is_multiline {
                    span {
                        class: "expression-line-expand-elipsis",
                        "..."
                    }
                    button {
                        class: "expression-line-expand-button",
                        onclick: move |_| { 
                            let new_value = !*is_expanded.peek();
                            is_expanded.set(new_value); 
                        },
                        if *is_expanded.read() { "hide" } else { "expand" }
                    }
                }
            }
        }
        div {
            class: "expression-line",
            div {
                class: "expression-line-lhs",
                if lhs.is_some() {
                    Block {
                        block: lhs.unwrap(), 
                        active_address: if is_last { Some(active_address) } else { None },
                        on_address_update: move |evt| address_update_handler.call(evt)
                    }
                }
            }
            div {
                class: "expression-line-mid",
                if mid.is_some() {
                    Block {
                        block: mid.unwrap(), 
                        active_address: if is_last { Some(active_address) } else { None },
                        on_address_update: move |evt| address_update_handler.call(evt)
                    }
                }
            }
            div {
                class: "expression-line-rhs",
                if rhs.is_some() {
                    Block {
                        block: rhs.unwrap(), 
                        active_address: if is_last { Some(active_address) } else { None },
                        on_address_update: move |evt| address_update_handler.call(evt)
                    }
                }
            }
        }
        if !is_last {
            div {
                class: "expression-line-right-panel",
                button {
                    onclick: move |_| {
                        let mut seq = ws.write().get(seq_index).unwrap();
                        seq.reset_to(line_index);
                        ws.write().store(seq_index, seq);
                        active_address.write().clear();
                    },
                    "reset"
                }
            }
        }
    }
}

#[component]
fn Block(block: Block, active_address: Option<Signal<Vec<Address>>>, on_address_update: EventHandler<(Address, bool)>) -> Element {
    use equaio::block::{BlockType, BlockTag};
    let mut classlist = vec![];
    if block.contains_tag(&BlockTag::Parentheses) { classlist.push("parenthesis"); }
    match block.block_type {
        BlockType::Symbol => {
            classlist.push("block-symbol");
            let is_clickable = active_address.is_some();
            let is_active = is_clickable && active_address.unwrap().read().contains(&block.address);
            if is_clickable { classlist.push("clickable"); }
            if is_active { classlist.push("active"); }
            if block.contains_tag(&BlockTag::Concealed) { classlist.push("concealed") };
            let symbol = utils::convert_mathvar(block.symbol.unwrap_or_default());
            return rsx! {
                div {
                    class: classlist.join(" "),
                    onclick: move |_| if is_clickable { 
                        on_address_update.call((block.address.clone(), !is_active))
                    },
                    "{symbol}"
                }
            }
        }
        BlockType::HorizontalContainer => {
            classlist.push("block-horizontal");
            let children = block.children.unwrap_or_default();
            return rsx! {
                div {
                    class: classlist.join(" "),
                    for child in children {
                        Block { block: child, active_address, on_address_update }
                    }
                }
            };
        }
        BlockType::FractionContainer => {
            classlist.push("block-fraction");
            let children = block.children.unwrap_or_default();
            let numerator = children.first();
            let denominator = children.last();
            if numerator.is_none() || denominator.is_none() { return rsx! { span { "ERROR: FractionContainer"} } }
            let numerator = numerator.unwrap();
            let denominator = denominator.unwrap();
            return rsx! {
                div {
                    class: classlist.join(" "),
                    div {
                        class: "block-fraction-numerator",
                        Block { block: numerator.clone(), active_address, on_address_update },
                    }
                    div {
                        class: "block-fraction-line"
                    }
                    div {
                        class: "block-fraction-denominator",
                        Block { block: denominator.clone(), active_address, on_address_update }
                    }
                }
            };
        }
    }
}