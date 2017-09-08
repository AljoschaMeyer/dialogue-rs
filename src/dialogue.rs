// TODO closing/aborting a Dialogue

use std::marker::PhantomData;
use std::fmt;
use std::error::Error;

use futures::{Future, Sink, Stream, Poll, StartSend};

use packet::{PacketWritable, PacketReadable, PacketId, PacketType};
use transport_error::TransportError;

/// Type-Level indicator for whether a `Dialogue` takes the server or the client
/// role. This information determines behaviour during the closing handshake.
pub trait Role {
    /// Returns whether the corresponding `Dialogue` has the server role.
    fn is_server() -> bool;
}

/// The server role.
pub enum Server {}

impl Role for Server {
    fn is_server() -> bool {
        true
    }
}

/// The client role.
pub enum Client {}

impl Role for Client {
    fn is_server() -> bool {
        false
    }
}

/// The main struct for communicating with a peer.
///
/// Incoming packets are emitted via the `Stream` implementation of `Dialogue`.
/// Packets can be sent via the corresponding methods of the struct.
pub struct Dialogue<P, T, SinkErr, StreamErr, Data, R> {
    transport: T,
    packet_type: PhantomData<P>,
    sink_err_type: PhantomData<SinkErr>,
    stream_err_type: PhantomData<StreamErr>,
    data_type: PhantomData<Data>,
    role_type: PhantomData<R>,
}

impl<P, T, SinkErr, StreamErr, Data, R> Dialogue<P, T, SinkErr, StreamErr, Data, R>
    where P: PacketReadable<Data = Data> + PacketWritable<Data = Data>,
          T: Sink<SinkItem = P, SinkError = SinkErr> + Stream<Item = P, Error = StreamErr>,
          R: Role
{
    /// Create a new `Dialogue` over the given transport.

    /// After starting sending packets via `message`, `request` or `duplex`
    /// this must be called to ensure that the packets have been written to the
    /// underlying transport. This simply delegates to `transport.poll_complete()`.
    pub fn poll_complete(&mut self) -> Poll<(), SinkErr> {
        self.transport.poll_complete()
    }

    /// Start sending the given data as a message.
    ///
    /// You have to call poll_complete to actually send the packet.
    pub fn message(&mut self, data: Data) -> StartSend<P, ClosedDialogue> {
        unimplemented!()
    }

    /// Start sending the given dataas a request.
    ///
    /// If sending fails, the returned `Response` `Future` yields an error.
    ///
    /// You have to call poll_complete to actually send the packet.
    pub fn request(&mut self, data: Data) -> Response<P, T, SinkErr, StreamErr, Data, R> {
        unimplemented!()
    }

    /// Start sending the given data as a duplex.
    ///
    /// If sending fails, the returned `SubDuplex`'s `Stream` and `Sink`
    /// implementations directly yield errors since the dialogue closed (erronously).
    ///
    /// You have to call poll_complete to actually send the packet.
    pub fn sub_duplex(&mut self,
                      data: Data)
                      -> SubDuplex<P, T, SinkErr, StreamErr, Data, OutSubDuplex, R> {
        unimplemented!()
    }

    // TODO sub_stream, sub_sink, sub_reduce_stream, sub_reduce_sink

    /// Creates a `Request` which allows correct handling of the packet. Use
    /// this for incoming packets for which
    /// packet.get_type() == PacketType::Request` is true.
    ///
    /// Passing a packet which is not an incoming request packet will lead to
    /// wrong communication.
    ///
    /// In debug mode, this method checks `packet.get_type() == PacketType::Request`
    /// and panics if it does not return true.
    pub fn packet_as_request(&mut self, packet: P) -> Request<P, T, SinkErr, StreamErr, Data, R> {
        debug_assert!(packet.get_type() == PacketType::Request);

        unimplemented!()
    }

    /// Creates a `SubDuplex` which allows correct handling of the packet. Use
    /// this for incoming packets for which
    /// packet.get_type() == PacketType::DuplexInitial` is true.
    ///
    /// Passing a packet which is not an incoming duplex packet will lead to
    /// wrong communication.
    ///
    /// In debug mode, this method checks `packet.get_type() == PacketType::DuplexInitial`
    /// and panics if it does not return true.
    pub fn packet_as_sub_duplex(&mut self,
                                packet: P)
                                -> SubDuplex<P, T, SinkErr, StreamErr, Data, InSubDuplex, R> {
        debug_assert!(packet.get_type() == PacketType::DuplexInitial);

        unimplemented!()
    }

    // // TODO variations of this for restricted duplexes: SubStream, SubSink, SubReduceStream, SubReduceSink
}

/// All incoming packets with fresh ids are emitted via this stream instance.
///
/// To correctly use the packets, use the `packet_as_request` and `packet_as_duplex`
/// methods of the `Dialogue`. TODO list other methods
///
/// Even if you want to ignore all incoming requests, you must still consume
/// this stream. Else, responses from the peer are not consumed either.
impl<P, T, SinkErr, StreamErr, Data, R> Stream for Dialogue<P, T, SinkErr, StreamErr, Data, R>
    where P: PacketReadable<Data = Data> + PacketWritable<Data = Data>,
          T: Sink<SinkItem = P, SinkError = SinkErr> + Stream<Item = P, Error = StreamErr>,
          R: Role
{
    type Item = P;
    type Error = TransportError<SinkErr, StreamErr>;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        unimplemented!()
    }
}

/// An error indicating that an operation failed because the corresponding
/// `Dialogue` has been closed.
#[derive(Debug)]
pub struct ClosedDialogue;

impl fmt::Display for ClosedDialogue {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "ClosedDialogue")
    }
}

impl Error for ClosedDialogue {
    fn description(&self) -> &str {
        "dialogue has been closed"
    }
}

/// A request that has been received from the peer.
///
/// This implements `Future` to be notified when/if the peer cancels the request.
pub struct Request<'ps, P: 'ps, T: 'ps, SinkErr: 'ps, StreamErr: 'ps, Data: 'ps, R: 'ps> {
    ps: &'ps mut Dialogue<P, T, SinkErr, StreamErr, Data, R>,
}

impl<'ps, P: 'ps, T: 'ps, SinkErr: 'ps, StreamErr: 'ps, Data: 'ps, R: 'ps> Request<'ps,
                                                                                   P,
                                                                                   T,
                                                                                   SinkErr,
                                                                                   StreamErr,
                                                                                   Data,
                                                                                   R>
    where P: PacketReadable<Data = Data> + PacketWritable<Data = Data>,
          T: Sink<SinkItem = P, SinkError = SinkErr> + Stream<Item = P, Error = StreamErr>,
          R: Role
{
    /// Gets the data that was sent with the request.
    pub fn get_data(&self) -> Data {
        unimplemented!()
    }

    /// Consumes the `Request` and writes some response data to the peer.
    ///
    /// The `StartSend` error variant is returned if the packet stream has closed.
    ///
    /// To make sure the response has actually been sent, call `poll_complete`
    /// on either the `Request` or the `Dialogue`.
    pub fn start_responding(self, data: Data) -> StartSend<Self, ClosedDialogue> {
        unimplemented!()
    }

    /// Consumes the `Request` and cancels it.
    ///
    /// The `StartSend` error variant is returned if the packet stream has closed.
    ///
    /// To make sure the cancellation has actually been sent, call `poll_complete`
    /// on either the `Request` or the `Dialogue`.
    pub fn start_cancelling(self) -> StartSend<Self, ClosedDialogue> {
        unimplemented!()
    }

    /// Delegates to the `poll_complete` method of the `Dialogue`.
    pub fn poll_complete(&mut self) -> Poll<(), ClosedDialogue> {
        unimplemented!()
    }
}

/// The future completes when this request is cancelled. It may never complete.
/// It is guaranteed to never yield an error (and the error type will be changed
/// once `!` becomes a legal rust type).
impl<'ps, P: 'ps, T: 'ps, SinkErr: 'ps, StreamErr: 'ps, Data: 'ps, R: 'ps> Future
    for
    Request<'ps, P, T, SinkErr, StreamErr, Data, R>
    where P: PacketReadable<Data = Data> + PacketWritable<Data = Data>,
          T: Sink<SinkItem = P, SinkError = SinkErr> + Stream<Item = P, Error = StreamErr>,
          R: Role
{
    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        unimplemented!();
    }
}

/// When dropping a `Request`, the corresponding `Dialogue` is notified so
/// that it stops waiting for cancellation.
impl<'ps, P: 'ps, T: 'ps, SinkErr: 'ps, StreamErr: 'ps, Data: 'ps, R: 'ps> Drop
    for Request<'ps, P, T, SinkErr, StreamErr, Data, R> {
    fn drop(&mut self) {
        unimplemented!()
    }
}

/// Type-Level indicator for whether a `SubDuplex` has been initiated by this side
/// of the dialogue (`is_out() == true`) or not.
pub trait SubDuplexType {
    /// Returns whether the corresponding `SubDuplex` has has been initiated by
    /// this side of the dialogue.
    fn is_out() -> bool;
}

/// Signifies a `SubDuplex` initiated by this side of the dialogue.
pub enum OutSubDuplex {}

impl SubDuplexType for OutSubDuplex {
    fn is_out() -> bool {
        true
    }
}

/// Signifies a `SubDuplex` *not* initiated by this side of the dialogue.
pub enum InSubDuplex {}

impl SubDuplexType for InSubDuplex {
    fn is_out() -> bool {
        false
    }
}

/// A duplex connection with the peer.
///
/// The `SubDuplexType` parameter does not exist at runtime, it just enforces some
/// static type safety about how duplex cancellation works.
pub struct SubDuplex<'ps,
                     P: 'ps,
                     T: 'ps,
                     SinkErr: 'ps,
                     StreamErr: 'ps,
                     Data: 'ps,
                     R: 'ps,
                     SubDuplexType: 'static>
{
    ps: &'ps mut Dialogue<P, T, SinkErr, StreamErr, Data, R>,
    duplex_type: PhantomData<SubDuplexType>,
}

impl<'ps, P: 'ps, T: 'ps, SinkErr: 'ps, StreamErr: 'ps, Data: 'ps, R: 'ps, SubDuplexType: 'static>
    SubDuplex<'ps, P, T, SinkErr, StreamErr, Data, R, SubDuplexType>
    where P: PacketReadable<Data = Data> + PacketWritable<Data = Data>,
          T: Sink<SinkItem = P, SinkError = SinkErr> + Stream<Item = P, Error = StreamErr>,
          R: Role
{
    /// Same as `close`, but the receiving duplex is given some error data.
    pub fn close_error(&mut self, err: Data) -> Poll<(), ClosedDialogue> {
        unimplemented!()
    }

    /// Directly close the stream (without error), not waiting for confirmation
    /// by the peer and dropping any outstanding responses or stream packets.
    pub fn abort(&mut self) -> Poll<(), ClosedDialogue> {
        unimplemented!()
    }

    /// Same as `abort`, but the receiving duplex is given some error data.
    pub fn abort_error(&mut self, err: Data) -> Poll<(), ClosedDialogue> {
        unimplemented!()
    }
}

/// Data written to this sink is passed to the corresponding stream on the
/// peer's side.
///
/// An error is emitted if the Dialogue has closed.
///
/// Use `close_error()` to terminate the duplex with an error value.
impl<'ps, P: 'ps, T: 'ps, SinkErr: 'ps, StreamErr: 'ps, Data: 'ps, R: 'ps, SubDuplexType: 'static> Sink
    for
    SubDuplex<'ps, P, T, SinkErr, StreamErr, Data, R, SubDuplexType>
    where P: PacketReadable<Data = Data> + PacketWritable<Data = Data>,
          T: Sink<SinkItem = P, SinkError = SinkErr> + Stream<Item = P, Error = StreamErr>, R: Role
{
    type SinkItem = Data;
    type SinkError = ClosedDialogue;

    fn start_send(&mut self, item: Self::SinkItem) -> StartSend<Self::SinkItem, Self::SinkError> {
        unimplemented!()
    }

    fn poll_complete(&mut self) -> Poll<(), Self::SinkError> {
        unimplemented!()
    }

    /// Performs a half-close of the duplex. Will wait for completely closing
    /// the duplex until the peer confirms the close. In between, responses and
    /// stream packets are still received.
    fn close(&mut self) -> Poll<(), Self::SinkError> {
        unimplemented!()
    }
}

/// The error for `Stream` implementation of substreams.
#[derive(Debug)]
pub enum SubStreamError<Data> {
    /// The corresponding dialogue has been closed.
    ClosedDialogue,
    /// The peer terminated the stream with some error data.
    EndWithError(Data),
}

impl<Data: fmt::Display> fmt::Display for SubStreamError<Data> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SubStreamError::ClosedDialogue => write!(fmt, "ClosedDialogue"),
            SubStreamError::EndWithError(ref data) => write!(fmt, "EndWithError: {}", data),
        }
    }
}

impl<Data: Error> Error for SubStreamError<Data> {
    fn description(&self) -> &str {
        match *self {
            SubStreamError::ClosedDialogue => "dialogue has been closed",
            SubStreamError::EndWithError(ref data) => data.description(),
        }
    }
}

/// Packet written to the peer's corresponding sink are passed to this sink.
///
impl<'ps,
     P: 'ps,
     T: 'ps,
     SinkErr: 'ps,
     StreamErr: 'ps,
     Data: 'ps,
     R: 'static,
     SubDuplexType: 'static> Stream
    for
    SubDuplex<'ps, P, T, SinkErr, StreamErr, Data, R, SubDuplexType>
    where P: PacketReadable<Data = Data> + PacketWritable<Data = Data>,
          T: Sink<SinkItem = P, SinkError = SinkErr> + Stream<Item = P, Error = StreamErr>,
          R: Role
{
    type Item = Data;
    type Error = SubStreamError<Data>;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        unimplemented!()
    }
}

/// When dropping a `SubDuplex`, the corresponding `Dialogue` is notified so
/// that it stops waiting for more duplex packets.
impl<'ps,
     P: 'ps,
     T: 'ps,
     SinkErr: 'ps,
     StreamErr: 'ps,
     Data: 'ps,
     R: 'ps,
     SubDuplexType: 'static> Drop
    for
    SubDuplex<'ps, P, T, SinkErr, StreamErr, Data, R, SubDuplexType> {
    fn drop(&mut self) {
        unimplemented!()
    }
}

/// This type represents the future response to a request. It also allows to
/// cancel the original request.
pub struct Response<'ps, P: 'ps, T: 'ps, SinkErr: 'ps, StreamErr: 'ps, Data: 'ps, R: 'ps> {
    ps: &'ps mut Dialogue<P, T, SinkErr, StreamErr, Data, R>,
}

impl<'ps, P: 'ps, T: 'ps, SinkErr: 'ps, StreamErr: 'ps, Data: 'ps, R: 'ps> Response<'ps,
                                                                                    P,
                                                                                    T,
                                                                                    SinkErr,
                                                                                    StreamErr,
                                                                                    Data,
                                                                                    R>
    where P: PacketReadable<Data = Data> + PacketWritable<Data = Data>,
          T: Sink<SinkItem = P, SinkError = SinkErr> + Stream<Item = P, Error = StreamErr>,
          R: Role
{
    /// Cancel the original request. To make sure the response has actually been
    /// sent, call `poll_complete` on either the `Response` or the `Dialogue`.
    ///
    /// Once the original request has been cancelled, this `Response` should be
    /// dropped.
    pub fn start_cancel() -> StartSend<Self, ClosedDialogue> {
        unimplemented!()
    }

    /// Delegates to the `poll_complete` method of the `Dialogue`.
    ///
    /// Once the original request has been cancelled, this `Response` should be
    /// dropped.
    pub fn poll_complete(&mut self) -> Poll<(), ClosedDialogue> {
        unimplemented!()
    }
}

/// The `Future` completes with `Some(Data)` when the response to the original
/// request was received. It completes with `None` if the peer signalled that it
/// won't answer the request.
///
/// It errors if the underlying `Dialogue` has shut down.
///
/// If the original request has been cancelled by this side of the dialogue,
/// this future may never resolve and should be `drop`ped.
impl<'ps, P: 'ps, T: 'ps, SinkErr: 'ps, StreamErr: 'ps, Data: 'ps, R: 'ps> Future
    for
    Response<'ps, P, T, SinkErr, StreamErr, Data, R>
    where P: PacketReadable<Data = Data> + PacketWritable<Data = Data>,
          T: Sink<SinkItem = P, SinkError = SinkErr> + Stream<Item = P, Error = StreamErr>,
          R: Role
{
    type Item = Option<Data>;
    type Error = ClosedDialogue;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        unimplemented!();
    }
}

/// When dropping a `Response`, the corresponding `Dialogue` is notified so
/// that it stops waiting for the response packet.
impl<'ps, P: 'ps, T: 'ps, SinkErr: 'ps, StreamErr: 'ps, Data: 'ps, R: 'ps> Drop
    for Response<'ps, P, T, SinkErr, StreamErr, Data, R> {
    fn drop(&mut self) {
        unimplemented!()
    }
}
