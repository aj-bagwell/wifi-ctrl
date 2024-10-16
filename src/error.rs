use super::*;
use broadcast::error::SendError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("start-up aborted")]
    StartupAborted,
    #[error("error parsing wifi status {e}: \n{s}")]
    ParsingWifiStatus { e: config::ConfigError, s: String },
    #[error("error parsing wifi config {e}: \n{s}")]
    ParsingWifiConfig { e: config::ConfigError, s: String },
    #[error("unexpected wifi ap response: {0}")]
    UnexpectedWifiApRepsonse(String),
    #[error("timeout waiting for response")]
    Timeout,
    #[error("did not write all bytes {0}/{1}")]
    DidNotWriteAllBytes(usize, usize),
    #[error("error parsing int: {0}")]
    ParseInt(#[from] std::num::ParseIntError),
    #[error("utf8 error: {0}")]
    Utf8Parse(#[from] std::str::Utf8Error),
    #[error("recv error: {0}")]
    Recv(#[from] oneshot::error::RecvError),
    #[error("unsolicited socket io error: {0}")]
    UnsolicitedIoError(std::io::Error),
    #[error("wifi_ctrl::station internal request channel unexpectedly closed")]
    WifiStationRequestChannelClosed,
    #[error("wifi_ctrl::station internal event channel unexpectedly closed")]
    WifiStationEventChannelClosed,
    #[error("wifi_ctrl::ap internal request channel unexpectedly closed")]
    WifiApRequestChannelClosed,
    #[error("wifi_ctrl::ap internal event channel unexpectedly closed")]
    WifiApEventChannelClosed,
    #[error("wifi ap broadcast: {0}")]
    WifiApBroadcast(#[from] broadcast::error::SendError<ap::Broadcast>),
    #[error("wifi::sta broadcast: {0}")]
    WifiStaBroadcast(#[from] broadcast::error::SendError<sta::Broadcast>),
    #[error("timeout opening socket {0}")]
    TimeoutOpeningSocket(String),
    #[error("permission denied opening socket {0}")]
    PermissionDeniedOpeningSocket(String),
}

impl Clone for Error {
    fn clone(&self) -> Self {
        match self {
            Self::Io(err) => Self::Io(clone_io(err)),
            Self::StartupAborted => Self::StartupAborted,
            Self::ParsingWifiStatus { e, s } => Self::ParsingWifiStatus {
                e: e.clone(),
                s: s.clone(),
            },
            Self::ParsingWifiConfig { e, s } => Self::ParsingWifiConfig {
                e: e.clone(),
                s: s.clone(),
            },
            Self::UnexpectedWifiApRepsonse(s) => Self::UnexpectedWifiApRepsonse(s.clone()),
            Self::Timeout => Self::Timeout,
            Self::DidNotWriteAllBytes(got, expected) => Self::DidNotWriteAllBytes(*got, *expected),
            Self::ParseInt(err) => Self::ParseInt(err.clone()),
            Self::Utf8Parse(err) => Self::Utf8Parse(*err),
            Self::Recv(err) => Self::Recv(err.clone()),
            Self::UnsolicitedIoError(err) => Self::UnsolicitedIoError(clone_io(err)),
            Self::WifiStationRequestChannelClosed => Self::WifiStationRequestChannelClosed,
            Self::WifiStationEventChannelClosed => Self::WifiStationEventChannelClosed,
            Self::WifiApRequestChannelClosed => Self::WifiApRequestChannelClosed,
            Self::WifiApEventChannelClosed => Self::WifiApEventChannelClosed,
            Self::WifiApBroadcast(SendError(sent)) => {
                Self::WifiApBroadcast(SendError(sent.clone()))
            }
            Self::WifiStaBroadcast(SendError(sent)) => {
                Self::WifiStaBroadcast(SendError(sent.clone()))
            }
            Self::TimeoutOpeningSocket(arg0) => Self::TimeoutOpeningSocket(arg0.clone()),
            Self::PermissionDeniedOpeningSocket(arg0) => {
                Self::PermissionDeniedOpeningSocket(arg0.clone())
            }
        }
    }
}

fn clone_io(err: &std::io::Error) -> std::io::Error {
    if let Some(raw) = err.raw_os_error() {
        std::io::Error::from_raw_os_error(raw)
    } else {
        std::io::Error::new(err.kind(), err.to_string())
    }
}
