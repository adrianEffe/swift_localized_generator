use regex::Regex;

pub fn parse(content: &str) -> Vec<&str> {
    let regex = Regex::new(r#""([a-zA-Z_]+)"\s*=\s*"[^"]*";"#).unwrap();
    let mut localizable_keys = Vec::new();
    for captures in regex.captures_iter(content) {
        let key = captures.get(1).unwrap().as_str();
        localizable_keys.push(key);
    }

    localizable_keys
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse() {
        let content = "\"homeTitle\" = \"Home\"; \"next_Title\" = \"This is next\";";
        assert_eq!(parse(content), vec!["homeTitle", "next_Title"]);
    }
}
