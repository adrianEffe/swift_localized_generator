pub fn format(keys: Vec<&str>) -> String {
    let mut localizable_case = "".to_string();
    let indentation = "    ";
    for key in keys {
        localizable_case = format!("{}{}{}{}{}", localizable_case, indentation, "case ", key, "\n");
    }

    format!(
        "{}{}{}",
        "enum LocalizationKey: String {\n", localizable_case, "}"
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
