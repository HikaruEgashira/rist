use walkdir::WalkDir;

pub fn get_current_dir() -> String {
    std::env::current_dir().unwrap().display().to_string()
}

pub fn display_dir(dir_name: &str) {
    let walker = WalkDir::new(dir_name).max_depth(1);
    for entry in walker {
        let entry = entry.unwrap();
        println!("{}", entry.path().display());
    }
}
