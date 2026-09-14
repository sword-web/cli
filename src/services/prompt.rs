use cliclack::{
    confirm as cconfirm, input as cinput, multiselect, select as cselect,
};

/// Asks the user to pick one option. `items` are `(value, label, hint)`.
pub fn select(prompt: &str, items: &[(&str, &str, &str)]) -> Option<String> {
    cselect(prompt)
        .items(items)
        .interact()
        .ok()
        .map(|value| value.to_string())
}

/// Asks a yes/no question, defaulting to `false` on cancel.
pub fn confirm(prompt: &str) -> bool {
    cconfirm(prompt).interact().unwrap_or(false)
}

/// Asks the user to pick any number of options.
pub fn multi_select(prompt: &str, items: &[(&str, &str, &str)]) -> Vec<String> {
    multiselect(prompt)
        .items(items)
        .interact()
        .unwrap_or_default()
        .into_iter()
        .map(|value| value.to_string())
        .collect()
}

/// Free-text input.
pub fn input(prompt: &str, placeholder: &str) -> Option<String> {
    cinput(prompt)
        .placeholder(placeholder)
        .interact::<String>()
        .ok()
}
