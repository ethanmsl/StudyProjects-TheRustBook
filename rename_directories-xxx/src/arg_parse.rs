pub fn check_for_path_argument(args_iterator: std::env::Args) -> Option<String> {
    // Guardian
    if args_iterator.len() != 2 {
        eprintln!(
            "Error: Expected 1 valid path argument to be explicitely provided, got {}",
            args_iterator.len() - 1
        );
        return None;
    }
    // Passed Guardian
    let mut args = args_iterator;
    args.next(); // skip the first arg, which is the path to the executable
    args.next()
}
