use chrono::{DateTime, Local};
use std::{
    env, fs, io,
    path::{Path, PathBuf},
    time::SystemTime,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {err}");
        std::process::exit(1);
    }
}

fn run() -> io::Result<()> {
    let filename = env::args()
        .nth(1)
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Usage: tstat <file>"))?;

    let path = Path::new(&filename);

    let metadata = fs::metadata(path)?;

    if !metadata.is_file() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("'{}' is not a file.", filename),
        ));
    }

    let contents = fs::read_to_string(path)?;
    let absolute_path = clean_path(fs::canonicalize(path)?);

    let line_count = contents.lines().count();
    let word_count = contents.split_whitespace().count();
    let char_count = contents.chars().count();

    println!("File Information");
    println!("----------------");

    println!(
        "{:<12} : {}",
        "Name",
        path.file_name().unwrap().to_string_lossy()
    );

    println!(
        "{:<12} : {}",
        "Extension",
        path.extension()
            .and_then(|e| e.to_str())
            .unwrap_or("(none)")
    );

    println!(
        "{:<12} : {}",
        "Directory",
        absolute_path.parent().unwrap().display()
    );

    println!("{:<12} : {}", "Path", absolute_path.display());

    println!("{:<12} : {}", "Size", format_file_size(metadata.len()));

    if let Ok(created) = metadata.created() {
        println!("{:<12} : {}", "Created", format_time(created));
    }

    if let Ok(modified) = metadata.modified() {
        println!("{:<12} : {}", "Modified", format_time(modified));
    }

    if let Ok(accessed) = metadata.accessed() {
        println!("{:<12} : {}", "Accessed", format_time(accessed));
    }

    println!();
    println!("Text Statistics");
    println!("---------------");

    println!("{:<12} : {}", "Lines", line_count);
    println!("{:<12} : {}", "Words", word_count);
    println!("{:<12} : {}", "Characters", char_count);
    println!("{:<12} : {}", "Bytes", metadata.len());

    Ok(())
}

fn format_file_size(bytes: u64) -> String {
    const KB: f64 = 1024.0;
    const MB: f64 = KB * 1024.0;
    const GB: f64 = MB * 1024.0;
    const TB: f64 = GB * 1024.0;

    let size = bytes as f64;

    if size >= TB {
        format!("{:.2} TB ({} bytes)", size / TB, bytes)
    } else if size >= GB {
        format!("{:.2} GB ({} bytes)", size / GB, bytes)
    } else if size >= MB {
        format!("{:.2} MB ({} bytes)", size / MB, bytes)
    } else if size >= KB {
        format!("{:.2} KB ({} bytes)", size / KB, bytes)
    } else {
        format!("{} bytes", bytes)
    }
}

fn format_time(time: SystemTime) -> String {
    let datetime: DateTime<Local> = time.into();
    datetime.format("%d %b %Y %H:%M:%S").to_string()
}

fn clean_path(path: PathBuf) -> PathBuf {
    #[cfg(windows)]
    {
        let path_str = path.to_string_lossy();

        if let Some(stripped) = path_str.strip_prefix(r"\\?\") {
            return PathBuf::from(stripped);
        }
    }

    path
}
