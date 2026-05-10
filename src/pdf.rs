use anyhow::Result;
use lopdf::Document;
use std::path::Path;

pub fn extract_text(path: &Path) -> Result<String> {
    let doc = Document::load(path)?;
    let text = doc.extract_text(&[])?;
    Ok(text)
}
