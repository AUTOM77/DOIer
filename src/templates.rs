use crate::config::constant::EXAMPLE_DOIS;

pub fn generate_examples_html() -> String {
    EXAMPLE_DOIS
        .iter()
        .map(|doi| format!(r#"<span class="example-item" onclick="window.fillDOI('{}')">{}</span>"#, doi, doi))
        .collect::<Vec<_>>()
        .join("\n            ")
}