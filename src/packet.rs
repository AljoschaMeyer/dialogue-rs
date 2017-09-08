/// Each packet has a PacketId, to identify it for multiplexing.
pub type PacketId = u32;

/// The different types a packet can have.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PacketType {
    Message,
    Request,
    Response,
    DuplexInitial,
    DuplexRequest,
    DuplexResponse,
    DuplexRequestEnd,
    DuplexResponseEnd,
}

/// Values implementing this trait can be sent via a `Dialogue`.
pub trait PacketWritable {
    /// The data carried by the packet.
    type Data;

    /// Sets the `PacketId` of the packet.
    fn set_id(&mut self, id: PacketId);

    /// Sets the `PacketType` of the packet.
    fn set_type(&mut self, t: PacketType);

    /// Creates a new packet. If the packet type also implements `PacketReadable`,
    /// then the `get_data` method of the created packet must return the same
    /// `Option` variant as the `data` argument.
    fn new(data: Option<Self::Data>) -> Self;
}

/// Values implementing this trait can be received via a `Dialogue`.
pub trait PacketReadable {
    /// The data carried by the packet.
    type Data;

    /// Gets the `PacketType` of the packet.
    fn get_id(&self) -> PacketId;

    /// Gets the `PacketType` of the packet.
    fn get_type(&self) -> PacketType;

    /// Gets the data carried by the packet.
    fn get_data(&self) -> Option<Self::Data>;

    /// Returns whether the packet carries any data.
    fn is_empty(&self) -> bool {
        self.get_data().is_none()
    }
}
