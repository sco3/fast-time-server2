use crate::args::Args;
use crate::logger::init_logger;

pub fn init_main() -> Args {
    let args = Args::get();
    init_logger(&args);
    args
}
