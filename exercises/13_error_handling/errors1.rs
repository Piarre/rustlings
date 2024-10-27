// TODO: This function refuses to generate text to be printed on a nametag if
// TODO: you pass it an empty string. It'd be nicer if it explained what the problem
// TODO: was instead of just returning `None`. Thankfully, Rust has a similar
// TODO: construct to `Option` that can be used to express error conditions. Change
// TODO: the function signature and body to return `Result<String, String>` instead
// TODO: of `Option<String>`.
fn generate_nametag_text(name: String) -> Result<String, String> {
    if name.is_empty() {
        // Empty names aren't allowed
        Err("Empty names aren't allowed".to_string())
    } else {
        Ok(format!("Hi! My name is {name}"))
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_nametag_text_for_a_nonempty_name() {
        assert_eq!(
            generate_nametag_text("Beyoncé".to_string()).as_deref(),
            Ok("Hi! My name is Beyoncé"),
        );
    }

    #[test]
    fn explains_why_generating_nametag_text_fails() {
        assert_eq!(
            generate_nametag_text(String::new())
                .as_ref()
                .map_err(|e| e.as_str()),
            Err("Empty names aren't allowed"),
        );
    }
}
