use regex::Regex;
use std::collections::BTreeMap;
use std::{fs, path::Path};
use walkdir::WalkDir;

fn main() {
    let src_path = Path::new("src");
    let frontmatter_re = Regex::new(r"(?s)^---\n(.*?)\n---").unwrap();
    let status_re = Regex::new(r#"status:\s*(\w+)"#).unwrap();

    let mut entries: BTreeMap<String, String> = BTreeMap::new();

    for entry in WalkDir::new(src_path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.path().extension().map(|ext| ext == "md").unwrap_or(false))
    {
        let path = entry.path();
        let filename = path.file_stem().unwrap().to_string_lossy().to_string();
        if filename == "SUMMARY" {
            continue;
        }

        let content = fs::read_to_string(path).unwrap_or_default();
        let status = frontmatter_re
            .captures(&content)
            .and_then(|caps| caps.get(1))
            .and_then(|first| status_re.captures(&first.as_str()))
            .and_then(|s| s.get(1))
            .map(|m| m.as_str().to_string())
            .unwrap_or_else(|| "unspecified".to_string());

        entries.insert(filename, status);
    }

    println!("| Chapter      | Status       |");
    println!("|--------------|--------------|");
    for (chapter, status) in &entries {
        println!("| {:12} | {:12} |", chapter, status);
    }

    let status_weights = |status: &str| -> f64 {
        match status {
            "idea" => 0.0,
            "draft" => 0.25,
            "revise" => 0.5,
            "review" => 0.75,
            "done" => 1.0,
            _ => 0.0, // treat unspecified as idea
        }
    };
    let total = entries.len() as f64;
    let sum: f64 = entries.values().map(|s| status_weights(s)).sum();
    let percentage = if total > 0.0 {
        (sum / total) * 100.0
    } else {
        0.0
    };

    println!("\nProject completion: {:.1}%", percentage);
}
