// Enum representing a file size stored in a specific unit.
// Used by the original formatter below which picks a unit
// based on decimal thresholds (1 KB = 1000 bytes in this implementation)
enum FileSize {
    Bytes(u64),
    Kilobytes(u64),
    Megabytes(u64),
    Gigabytes(u64),
    Terabytes(u64),
}

// Original enum-based formatter: selects a unit by range and
// formats the result. It uses decimal thresholds (1000-based) and
// formats values with two decimal places when applicable.
fn format_size(size: u64) -> String {
    let filesize = match size {
        0..=999 => FileSize::Bytes(size),
        1000..=999_999 => FileSize::Kilobytes(size / 1000),
        1_000_000..=999_999_999 => FileSize::Megabytes(size / 1_000_000),
        1_000_000_000..=999_999_999_999 => FileSize::Gigabytes(size / 1_000_000_000),
        _ => FileSize::Terabytes(size / 1_000_000_000_000),
    };

    match filesize {
        FileSize::Bytes(bytes) => format!("{} bytes", bytes),
        FileSize::Kilobytes(kb) => format!("{:.2} KB", kb as f64 / 1000.0),
        FileSize::Megabytes(mb) => format!("{:.2} MB", mb as f64 / 1000.0),
        FileSize::Gigabytes(gb) => format!("{:.2} GB", gb as f64 / 1000.0),
        FileSize::Terabytes(tb) => format!("{:.2} TB", tb as f64 / 1000.0),
    }
}

// Decimal (SI-like) units using base 1000.
// This formatter converts raw bytes into the largest decimal unit
// (KB, MB, ...) and formats the result with two decimal places.
fn format_size_decimal(bytes: u64) -> String {
    const UNITS: [&str; 6] = ["bytes", "KB", "MB", "GB", "TB", "PB"];

    if bytes < 1000 {
        return format!("{} bytes", bytes);
    }

    let mut size = bytes as f64;
    let mut unit_index = 0usize;

    while size >= 1000.0 && unit_index < UNITS.len() - 1 {
        size /= 1000.0;
        unit_index += 1;
    }

    format!("{:.2} {}", size, UNITS[unit_index])
}

// Binary units using base 1024.
// Many operating systems show sizes using base 1024 (1 KiB = 1024 bytes).
// This function uses 1024-based scaling and formats values with two decimal places.
fn format_size_binary(bytes: u64) -> String {
    const UNITS: [&str; 6] = ["bytes", "KB", "MB", "GB", "TB", "PB"];

    if bytes < 1024 {
        return format!("{} bytes", bytes);
    }

    let mut size = bytes as f64;
    let mut unit_index = 0usize;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    format!("{:.2} {}", size, UNITS[unit_index])
}

fn main() {
    // Single example showing the original enum-based formatter in action
    let result = format_size(6888837399);
    println!("{}", result);

    // Test values to exercise different ranges and thresholds:
    // - 2500 bytes (should show ~2.44 KB in decimal)
    // - 1024 bytes (boundary for binary)
    // - 1 MB expressed as 1024*1024
    // - a large multi-gigabyte number
    // - a very large terabyte-scale number
    // - a small value (500 bytes)
    let test_values = [
        2500u64,
        1024u64,
        1024u64 * 1024,
        5_000_000_000u64,
        6888822237399u64,
        500u64,
    ];

    // Run and print outputs from the original enum-based formatter
    println!("\nEnum-based conversion (original):");
    for &v in &test_values {
        println!("{} bytes => {}", v, format_size(v));
    }

    // Run and print outputs from the binary (1024-based) formatter
    println!("\nBinary (base 1024) conversions:");
    for &v in &test_values {
        println!("{} bytes => {}", v, format_size_binary(v));
    }

    // Run and print outputs from the decimal (1000-based) formatter
    println!("\nDecimal (base 1000) conversions:");
    for &v in &test_values {
        println!("{} bytes => {}", v, format_size_decimal(v));
    }
}
