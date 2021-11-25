use walkdir::WalkDir;

fn get_args() -> Vec<String> {
    let mut args = Vec::new();
    for arg in std::env::args() {
        args.push(arg);
    }
    args
}

fn get_current_dir() -> String {
    std::env::current_dir().unwrap().display().to_string()
}

fn main() {
    let current_dir = get_current_dir();
    let args = get_args();
    let dir_name = args.get(1).unwrap_or(&current_dir);

    let walker = WalkDir::new(dir_name);
    for entry in walker.max_depth(1) {
        println!("{}", entry.unwrap().path().display());
    }
}
