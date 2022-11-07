pub fn check_for_path_argument(args_iterator: std::env::Args) -> Option<String> {
    let mut args = args_iterator;
    args.next(); // skip the first arg, which is the path to the executable
    args.next()
}
