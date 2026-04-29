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

    /// The `--ip` argument is missing and was not provided.
    #[error("missing required field: --ip")]
    MissingIp,

    /// The `--port` argument is missing and was not provided.
    #[error("missing required field: --port")]
    MissingPort,

    /// The `--ip-range` argument has an invalid format or start > end.
    #[error(
        "invalid IPv4 range format, must be <starting IPv4>-<ending IPv4> where <starting IPv4> is lower or equal than <ending IPv4>"
    )]
    IpRangeFormat,

    /// The `--port-range` argument has an invalid format or start > end.
    #[error(
        "invalid port range format, must be <starting port>-<ending port> where <starting port> is lower or equal than <ending port>"
    )]
    PortRangeFormat,

    /// The `--ip-range` argument is missing and was not provided.
    #[error("missing required field: --ip-range")]
    MissingIpRange,

    /// The `--port-range` argument is missing and was not provided.
    #[error("missing required field: --port-range")]
    MissingPortRange,

    /// The starting IPv4 address in the range is not a valid IPv4 address.
    #[error("invalid starting IPv4 syntaxe")]
    InvalidStartIp,

    /// The ending IPv4 address in the range is not a valid IPv4 address.
    #[error("invalid ending IPv4 syntaxe")]
    InvalidEndIp,

    /// The starting port in the range is not a valid `u16`.
    #[error("invalid starting port number")]
    InvalidStartPort,

    /// The ending port in the range is not a valid `u16`.
    #[error("invalid ending port number")]
    InvalidEndPort,

    /// An error propagated from the `dtiw385` decoder library.
    #[error("decoder error : {0}")]
    Decoder(#[from] dtiw385::DecoderError),

    /// The `--key` argument is missing and was not provided.
    #[error("missing required field: --key")]
    MissingKey,

    /// The `--key` argument have the wrong syntaxe.
    #[error(
        "invalid key syntaxe, must be one of (PowerOnOff, Ok, Up, Down, Left, Right, Back, Menu, VolumeUp, VolumeDown, Mute, ChannelUp, ChannelDown, Play, Pause, Stop, Forward, Rewind, N0, N1, N2, N3, N4, N5, N6, N7, N8, N9)"
    )]
    InvalidKey,
}
