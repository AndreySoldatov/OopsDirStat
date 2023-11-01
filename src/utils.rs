pub fn bytes_to_string(bytes: u64) -> String {
    if (bytes / 1_000_000_000_000) >= 1 {
        format!("{}TB", bytes / 1_000_000_000_000)
    } else if (bytes / 1_000_000_000) >= 1 {
        format!("{}GB", bytes / 1_000_000_000)
    } else if (bytes / 1_000_000) >= 1 {
        format!("{}MB", bytes / 1_000_000)
    } else if (bytes / 1_000) >= 1 {
        format!("{}KB", bytes / 1_000)
    } else {
        format!("{}B", bytes)
    }
}