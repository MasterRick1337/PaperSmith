pub fn apply_markdown(content: &String) -> String {
    markdown::to_html(content.as_str())
}
