use thiserror::Error;

/// All errors that can occur in the `dtiw385` CLI.
#[derive(Error, Debug)]
pub enum Error {
    /// Failed to serialize and print a JSON output object.
    #[error("failed to print output json object")]
    PrintOutput,

    /// Failed to deserialize a JSON input object from stdin.
    #[error("failed to parse input json object")]
    ParseJson(#[from] serde_json::Error),

    /// The `--ip` argument is missing and was not provided via stdin.
    #[error("missing required field: --ip")]
    MissingIp,

    /// The `--port` argument is missing and was not provided via stdin.
    #[error("missing required field: --port")]
    MissingPort,

    /// The `--ip-range` argument has an invalid format or start > end.
    #[error(
        "invalid ip range format, must be <starting ip>-<ending ip> where <starting ip> is lower or equal than <ending ip>"
    )]
    IpRangeFormat,

    /// The `--port-range` argument has an invalid format or start > end.
    #[error(
        "invalid port range format, must be <starting port>-<ending port> where <starting port> is lower or equal than <ending port>"
    )]
    PortRangeFormat,

    /// The `--ip-range` argument is missing and was not provided via stdin.
    #[error("missing required field: --ip-range")]
    MissingIpRange,

    /// The `--port-range` argument is missing and was not provided via stdin.
    #[error("missing required field: --port-range")]
    MissingPortRange,

    /// The starting IP address in the range is not a valid IPv4 address.
    #[error("invalid starting IP address, must be u8.u8.u8.u8-<ending ip>")]
    InvalidStartIp,

    /// The ending IP address in the range is not a valid IPv4 address.
    #[error("invalid ending IP address, must be <starting ip>-u8.u8.u8.u8")]
    InvalidEndIp,

    /// The starting port in the range is not a valid `u16`.
    #[error("invalid starting port number, must be u16-<ending port>")]
    InvalidStartPort,

    /// The ending port in the range is not a valid `u16`.
    #[error("invalid ending port number, must be <starting port>-u16")]
    InvalidEndPort,

    /// An error propagated from the `dtiw385` decoder library.
    #[error("decoder error : {0}")]
    Decoder(#[from] dtiw385::DecoderError),
}
