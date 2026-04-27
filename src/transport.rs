use clap::ValueEnum;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum Transport {
    Stdio,
    Sse,
    Http,
    Dual,
    Rest,
}
