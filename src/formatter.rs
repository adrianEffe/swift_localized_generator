pub fn format(keys: Vec<&str>) -> String {
    let mut swift_file = "".to_string();
    let indentation = "    ";
    for key in keys {
        swift_file = format!("{}{}{}{}{}", swift_file, indentation, "case ", key, "\n");
    }

    format!(
        "{}{}{}",
        "enum LocalizationKey: String {\n", swift_file, "}"
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_format_localization_key() {
        let keys = vec!["homepage", "profileTitle", "paymentSubtitle"];
        let expectation = "enum LocalizationKey: String {\n    case homepage\n    case profileTitle\n    case paymentSubtitle\n}";
        assert_eq!(format(keys), expectation);
    }
}
