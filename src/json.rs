pub const MAIN_MENU_DATA: &str = r#"
[
  {
    "name": "Algebra (Step by Step)",
    "problem_ids": [
        "algebra0",
        "algebra1",
        "algebra2",
        "algebra3"
    ]
  },
  {
    "name": "Algebra",
    "problem_ids": [
        "algebra_simplify0",
        "algebra_simplify1",
        "algebra_simplify2",
        "algebra_simplify3"
    ]
  },
  {
    "name": "Logic",
    "problem_ids": [
        "logic0"
    ]
  }
]
"#;

pub const PROBLEMS_DATA_MAP: &str = r#"
{
    "algebra0": {
        "label": "Solve for x",
        "sublabel": "x + 3 = 5",
        "rule": "algebra",
        "variables": ["x"],
        "initial_expressions": ["x + 3 = 5"]
    },
    "algebra1":{
        "label": "Solve for x",
        "sublabel": "2x - 1 = 3",
        "rule": "algebra",
        "variables": ["x"],
        "initial_expressions": ["(2 * x) - 1 = 3"]
    },
    "algebra2": {
        "label": "Simplify the expression",
        "rule": "algebra",
        "variables": ["x"],
        "initial_expressions": ["(6 * x) + (-4) + (3 * x) + 1"]
    },
    "algebra3": {
        "label": "SLETV example",
        "rule": "algebra",
        "variables": ["x", "y"],
        "initial_expressions": ["x + y = 3", "x - y = 1"]
    },
    "algebra_simplify0": {
        "label": "Solve for x",
        "sublabel": "x + 3 = 5",
        "rule": "algebra_simplify",
        "variables": ["x"],
        "initial_expressions": ["x + 3 = 5"]
    },
    "algebra_simplify1": {
        "label": "Solve for x",
        "sublabel": "2x - 1 = 3",
        "rule": "algebra_simplify",
        "variables": ["x"],
        "initial_expressions": ["(2 * x) - 1 = 3"]
    },
    "algebra_simplify2": {
        "label": "SLETV example",
        "rule": "algebra_simplify",
        "variables": ["x", "y"],
        "initial_expressions": ["x + y = 3", "x - y = 1"]
    },
    "logic0": {
        "label": "Simplify the expression",
        "rule": "logic",
        "variables": ["P", "Q"],
        "initial_expressions": ["(~P | Q) & (P | Q)"]
    }
}
"#;

pub const ALGEBRA_RULES: &str = r#"
{
    "name": "algebra",
    "context": {
        "unary_ops": ["-"],
        "binary_ops": ["+", "-", "*", "/"],
        "assoc_ops": ["+", "*"],
        "handle_numerics": true
    },
    "variations": [
        {"expr":  "A + B = B + A"},
        {"expr":  "A * B = B * A"}
    ],
    "normalization": [
        {"expr": "NOTE: this normalization fields is not being used yet"},
        {"expr_prefix": "=(-(0),0)"}
    ],
    "rules": [
        {
            "id": "add_zero",
            "expr": "X + 0 = X",
            "label": "Addition with 0"
        },
        {
            "id": "mul_one",
            "expr": "X * 1 = X",
            "label": "Multiplication with 1"
        },
        {
            "id": "mul_zero",
            "expr": "X * 0 = 0",
            "label": "Multiplication with 0"
        },
        {
            "id": "sub_zero",
            "expr": "X - 0 = X",
            "label": "Subtraction by 0"
        },
        {
            "id": "div_one",
            "expr": "X / 1 = X",
            "label": "Division by 1"
        },
        {
            "id": "sub_self",
            "expr": "X - X = 0",
            "label": "Self subtraction"
        },
        {
            "id": "add_negative_self",
            "expr": "X + (-X) = 0",
            "label": "Self subtraction"
        },
        {
            "id": "factor_out_minus_right",
            "expr": "X * (-Y) = -(X * Y)",
            "label": "Factor out the minus sign",
            "variations": []
        },
        {
            "id": "factor_out_minus_left",
            "expr": "(-X) * Y = -(X * Y)",
            "label": "Factor out the minus sign",
            "variations": []
        },
        {
            "id": "add_self",
            "expr": "X + X = 2 * X",
            "label": "Self addition"
        },
        {
            "id": "distribution",
            "expr": "X * (A_i + ...) = (X * A_i) + ...",
            "label": "Distribution"
        },
        {
            "id": "factor_out_left",
            "expr": "(X * A_i) + ... = X * (A_i + ...)",
            "label": "Factoring Out",
            "variations": []
        },
        {
            "id": "factor_out_right",
            "expr": "(A_i * X) + ... = (A_i + ...) * X",
            "label": "Factoring Out",
            "variations": []
        }
    ]
}
"#;

pub const ALGEBRA_SIMPLIFY_RULES: &str = r#"
{
    "name": "algebra",
    "context": {
        "unary_ops": ["-"],
        "binary_ops": ["+", "-", "*", "/"],
        "assoc_ops": ["+", "*"],
        "handle_numerics": true
    },
    "variations": [
        {"expr":  "A + B = B + A"},
        {"expr":  "A * B = B * A"}
    ],
    "normalization": [
        {"expr": "NOTE: this normalization fields is not being used yet"},
        {"expr_prefix": "=(-(0),0)"}
    ],
    "rules": [
        {
            "id": "add_zero",
            "expr": "X + 0 = X",
            "label": "Addition with 0",
            "auto": true
        },
        {
            "id": "mul_one",
            "expr": "X * 1 = X",
            "label": "Multiplication with 1",
            "auto": true
        },
        {
            "id": "mul_zero",
            "expr": "X * 0 = 0",
            "label": "Multiplication with 0",
            "auto": true
        },
        {
            "id": "sub_zero",
            "expr": "X - 0 = X",
            "label": "Subtraction by 0",
            "auto": true
        },
        {
            "id": "div_one",
            "expr": "X / 1 = X",
            "label": "Division by 1",
            "auto": true
        },
        {
            "id": "sub_self",
            "expr": "X - X = 0",
            "label": "Self subtraction",
            "auto": true
        },
        {
            "id": "add_negative_self",
            "expr": "X + (-X) = 0",
            "label": "Self subtraction",
            "auto": true
        },
        {
            "id": "factor_out_minus_right",
            "expr": "X * (-Y) = -(X * Y)",
            "label": "Factor out the minus sign",
            "variations": []
        },
        {
            "id": "factor_out_minus_left",
            "expr": "(-X) * Y = -(X * Y)",
            "label": "Factor out the minus sign",
            "variations": []
        },
        {
            "id": "add_self",
            "expr": "X + X = 2 * X",
            "label": "Self addition"
        },
        {
            "id": "distribution",
            "expr": "X * (A_i + ...) = (X * A_i) + ...",
            "label": "Distribution"
        },
        {
            "id": "factor_out_left",
            "expr": "(X * A_i) + ... = X * (A_i + ...)",
            "label": "Factoring Out",
            "variations": []
        },
        {
            "id": "factor_out_right",
            "expr": "(A_i * X) + ... = (A_i + ...) * X",
            "label": "Factoring Out",
            "variations": []
        }
    ]
}
"#;