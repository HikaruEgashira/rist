mod lib;

fn main() {
    let current_dir = lib::fs::get_current_dir();
    let args = lib::cli::get_args();
    let dir_name = args.get(1).unwrap_or(&current_dir);

    lib::fs::display_dir(dir_name);
}
