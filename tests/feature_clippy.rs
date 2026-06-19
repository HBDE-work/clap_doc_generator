use std::process::Command;

#[test]
fn clippy_passes_for_all_feature_combinations() {
    let features: Vec<String> = read_feature_names();
    let combinations = combinations(&features);

    for combination in &combinations {
        let feature_str = combination.join(",");

        let output = Command::new("cargo")
            .args([
                "clippy",
                "--no-default-features",
                "--features",
                &feature_str,
                "--",
                "-D",
                "warnings",
            ])
            .output()
            .expect("failed to execute cargo clippy");

        let stderr = String::from_utf8_lossy(&output.stderr);

        assert!(
            output.status.success(),
            "clippy failed for features [{}]:\n{}",
            feature_str,
            stderr,
        );
    }
}

fn read_feature_names() -> Vec<String> {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let cargo_toml_path = format!("{manifest_dir}/Cargo.toml");
    let content = std::fs::read_to_string(&cargo_toml_path).expect("failed to read Cargo.toml");

    let parsed: toml::Table = toml::from_str(&content).expect("failed to parse Cargo.toml");

    parsed
        .get("features")
        .and_then(|f| f.as_table())
        .map(|table| {
            table
                .keys()
                .filter(|key| *key != "default")
                .cloned()
                .collect()
        })
        .unwrap_or_default()
}

fn combinations(items: &[String]) -> Vec<Vec<String>> {
    let count = 1_usize << items.len();
    let mut result = Vec::with_capacity(count - 1);

    for mask in 1..count {
        let mut subset = Vec::new();
        for (index, item) in items.iter().enumerate() {
            if mask & (1 << index) != 0 {
                subset.push(item.clone());
            }
        }
        result.push(subset);
    }

    result
}
