/*
 * This is the main function. The arguments are read for get_args() and passed
 * to run().
 */
fn main() {
    if let Err(e) = catr::get_args().and_then(catr::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
