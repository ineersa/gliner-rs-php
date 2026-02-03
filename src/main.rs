fn main() {
}

#[cfg(test)]
mod tests {
    use ext_php_rs::embed::Embed;
    use gliner_rs_php::build_text_input;

    #[test]
    fn build_text_input_success() {
        let input = build_text_input(
            vec!["Hello world".to_string(), "Second text".to_string()],
            vec!["PERSON".to_string(), "ORG".to_string()],
        )
        .expect("Expected valid text input");

        assert_eq!(input.texts, vec!["Hello world", "Second text"]);
        assert_eq!(input.entities, vec!["PERSON", "ORG"]);
    }

    #[test]
    fn build_text_input_rejects_empty_texts() {
        Embed::run(|| {
            let result = build_text_input(vec![], vec!["PERSON".to_string()]);

            assert!(result.is_err());
        });
    }

    #[test]
    fn build_text_input_rejects_empty_labels() {
        Embed::run(|| {
            let result = build_text_input(vec!["Hello".to_string()], vec![]);

            assert!(result.is_err());
        });
    }
}
