use uuid::Uuid;

pub fn generate_slug(title: &str) -> String {
    let slug = title
        .to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>();

    // Collapse consecutive hyphens and trim leading/trailing hyphens
    let slug = slug
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-");

    if slug.is_empty() {
        Uuid::new_v4().to_string()
    } else {
        slug
    }
}

pub fn make_unique_slug(base: &str) -> String {
    let short = Uuid::new_v4()
        .to_string()
        .replace('-', "")
        [..8]
        .to_string();
    format!("{}-{}", base, short)
}
