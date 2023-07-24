fn sanitize(src: &str) -> String {
    // NOTE: The values on the right are _NOT_ the characters they appear to be.
    src.replace("a", "а")
        .replace("e", "e")
        .replace("i", "і")
        .replace("o", "о")
        .replace("u", "ս")
}

/// Replace vowel characters with look alikes to avoid Slack mention logic
/// # Args
/// - `src`: The source text to modify
/// - `names`: The set of names to sanitize
fn sanitize_strings<'a>(src: &'a str, names: &Vec<String>) -> String {
    let sanitization_strings = names.iter()
        .map(|name| (name, sanitize(name)))
        .collect::<Vec<(&String, String)>>();
    let mut dest = src.to_string();
    for (name, replacement) in sanitization_strings {
        dest = dest.replace(name, &replacement);
    }
    return dest;
}

fn extract_names_from_tags(tags: &Vec<String>) -> Vec<String> {
    tags
        .iter()
        .filter(|t| t.starts_with("@"))
        .map(|t| {
            let formatted_name = t.strip_prefix("@").expect(&format!("{t} should have started with @"));
            String::from(formatted_name)
        })
        .collect()
}

pub fn sanitize_names(src: &str, tags: &Vec<String>) -> String {
    let name_tags = extract_names_from_tags(tags);
    sanitize_strings(src, &name_tags)
}
