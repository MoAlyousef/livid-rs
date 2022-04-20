use livid_cli::handle_args;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    handle_args(&args);
}
