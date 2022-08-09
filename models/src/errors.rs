use snafu::Snafu;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum Error {
    #[snafu(display("Invalid point: {}", err))]
    InvalidPoint { err: String },

    #[snafu(display("Invalid tag: {}", err))]
    InvalidTag { err: String },

    #[snafu(display("Invalid field: {}", err))]
    InvalidField { err: String },

    #[snafu(display("Invalid flatbuffer message: {}", err))]
    InvalidFlatbufferMessage { err: String },

    #[snafu(display("Invalid serde message: {}", err))]
    InvalidSerdeMessage { err: String },
}
