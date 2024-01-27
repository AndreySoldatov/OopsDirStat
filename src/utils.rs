pub fn bytes_to_string(bytes: u64) -> String {
    if (bytes / 1_000_000_000_000) >= 1 {
        format!("{:.1}TB", bytes as f64 / 1_000_000_000_000.0)
    } else if (bytes / 1_000_000_000) >= 1 {
        format!("{:.1}GB", bytes as f64 / 1_000_000_000.0)
    } else if (bytes / 1_000_000) >= 1 {
        format!("{:.1}MB", bytes as f64 / 1_000_000.0)
    } else if (bytes / 1_000) >= 1 {
        format!("{:.1}KB", bytes as f64 / 1_000.0)
    } else {
        format!("{}B", bytes)
    }
}