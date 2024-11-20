use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("failed to open")]
    OpenFile(#[from] std::io::Error),
    #[error("failed to close")]
    CloseFIle(#[from] std::io::Error),
}

fn main() {}
