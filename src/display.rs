use colored::Colorize;

/// Print a section header with box-drawing chars
pub fn section_header(title: &str) {
    let width = 60;
    let line = "─".repeat(width);
    println!("{}", format!("┌{}┐", line).bright_blue());
    let padding = width.saturating_sub(title.len() + 2);
    let left = padding / 2;
    let right = padding - left;
    println!(
        "{}",
        format!("│ {}{}{} │", " ".repeat(left), title.bold(), " ".repeat(right)).bright_blue()
    );
    println!("{}", format!("└{}┘", line).bright_blue());
}

/// Print a key-value pair
pub fn kv(key: &str, value: &str) {
    println!("  {} {}", format!("{key}:").dimmed(), value);
}

/// Print a tree item with an icon
pub fn tree_item(indent: usize, icon: &str, label: &str, detail: &str) {
    let prefix = "  ".repeat(indent);
    if detail.is_empty() {
        println!("  {prefix}{icon} {}", label.bold());
    } else {
        println!("  {prefix}{icon} {}  {}", label.bold(), detail.dimmed());
    }
}

pub fn success(msg: &str) {
    println!("{} {}", "✓".green().bold(), msg);
}

pub fn warning(msg: &str) {
    println!("{} {}", "⚠".yellow().bold(), msg);
}

pub fn error(msg: &str) {
    eprintln!("{} {}", "✗".red().bold(), msg);
}

pub fn info(msg: &str) {
    println!("{} {}", "·".cyan(), msg);
}

pub fn created(path: &str) {
    println!("  {} {}", "create".green().bold(), path);
}

pub fn modified(path: &str) {
    println!("  {} {}", "update".yellow().bold(), path);
}

/// Render a simple ASCII dependency graph line
#[allow(dead_code)]
pub fn dep_arrow(from: &str, to: &str) {
    println!("  {} {} {}", from.cyan().bold(), "→".dimmed(), to.green());
}
