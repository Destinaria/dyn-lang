mod args;

fn main() {
    args::parse_args(std::env::args());
}
