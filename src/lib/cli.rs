pub fn get_args() -> Vec<String> {
    let mut args = Vec::new();
    for arg in std::env::args() {
        args.push(arg);
    }
    args
}
