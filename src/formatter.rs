pub fn format(keys: &[&str]) -> String {
    let disclaimer = "/// This code is automatically generated by `swift_localized_generator`.\n\n";
    let mut localizable_case = "".to_string();
    let indentation = "    ";
    for key in keys {
        localizable_case = format!("{}{}{}{}{}", localizable_case, indentation, "case ", key, "\n");
    }

    format!(
        "{}{}{}{}",
        disclaimer,
        "enum LocalizationKey: String {\n", localizable_case, "}\n"
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_format_localization_key() {
        let keys = vec!["homepage", "profileTitle", "paymentSubtitle"];
        let expectation = "/// This code is automatically generated by `swift_localized_generator`.\n\nenum LocalizationKey: String {\n    case homepage\n    case profileTitle\n    case paymentSubtitle\n}\n";
        assert_eq!(format(&keys), expectation);
    }
}
