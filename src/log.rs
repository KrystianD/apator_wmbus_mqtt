#[macro_export]
macro_rules! tprintln {
    ($($arg:tt)*) => {{
       let ts = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
       let file = std::path::Path::new(file!())
        .file_name()
        .and_then(|f| f.to_str())
        .unwrap_or(file!());

        println!("[{}] [{}:{}] {}", ts, file, line!(), format!($($arg)*));
    }};
}
