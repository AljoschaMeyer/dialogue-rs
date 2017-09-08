use std::fmt;
use std::error::Error;

/// A transport error: Either an error emitted by the `Sink` implementation of
/// a transport, or by the `Stream` implementation.
#[derive(Debug)]
pub enum TransportError<SinkErr, StreamErr> {
    /// An error originating from a `Sink` implementation.
    SinkError(SinkErr),
    /// An error originating from a `Stream` implementation.
    StreamError(StreamErr),
}

impl<SinkErr: fmt::Display, StreamErr: fmt::Display> fmt::Display
    for TransportError<SinkErr, StreamErr> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TransportError::SinkError(ref e) => write!(fmt, "SinkError: {}", e),
            TransportError::StreamError(ref e) => write!(fmt, "StreamError: {}", e),
        }
    }
}

impl<SinkErr: Error, StreamErr: Error> Error for TransportError<SinkErr, StreamErr> {
    fn description(&self) -> &str {
        match *self {
            TransportError::SinkError(ref e) => e.description(),
            TransportError::StreamError(ref e) => e.description(),
        }
    }
}
