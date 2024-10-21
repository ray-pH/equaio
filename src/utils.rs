pub fn convert_mathvar(original: String) -> String {
    original.chars().map(|c| to_mathvar(c).unwrap_or(c)).collect()
}

#[allow(dead_code)]
pub fn convert_mathvar_vec(original: String, variables: Vec<String>) -> String {
    original.chars().map(|c| {
        if variables.contains(&c.to_string()) { to_mathvar(c).unwrap_or(c) } else { c }
    }).collect()
}

fn to_mathvar(c: char) -> Option<char> {
    match c {
        'a'..='z' => std::char::from_u32(c as u32 + 0x1D44E - 'a' as u32),
        'A'..='Z' => std::char::from_u32(c as u32 + 0x1D434 - 'A' as u32),
        '-' => std::char::from_u32(0x2212),
        _ => Some(c), 
    }
}