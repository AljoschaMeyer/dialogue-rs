# Dialogue
A simple abstraction layer for multiplexed, streaming communication between two peers. Inpired by/based on [packet-stream](https://github.com/ssbc/packet-stream), but with a few [breaking changes](https://github.com/ssbc/packet-stream/issues/12#issuecomment-326543887).

Over a reliable, ordered, one-to-one, duplex communication channel, the following types of communication are supported:

- *message*: Send a packet to the peer.
- *request*: Send a packet to the peer, receive a response packet. Can be cancelled by either participant.
- *duplex*: Send many packets to the peer, receive many response packets. Both participants can cancel the duplex.

Additionally, implementations may provide restricted versions of duplexes, e.g. unidirectional streams, or unidirectional streams which expect a single response after signalling their end. On the wire, these would simply use duplex packets, only the interface for the programmer is restricted.

## How it works

A `packet` is a type, which allows getting and setting:

- an `id` (unsigned 32 bit integer)
- a packet_type (an enum, see the section on packet types below)

Additionally, the `packet` type is generic over some `Data` type. Any packet may either carry data, or it may carry none The packet abstraction allows to:

- retrieve the data of a packet (and in particular, check whether it carries data at all)
- create a packet which does not carry data

Dialogue is unopiniated about how the packet determines whether it carries data. For example, it could set a flag, or it might have a length indicator of zero.

### Packet types:
The `PacketType` enum has the following variants:

- Message
- Request
- Response
- DuplexInitial
- DuplexRequest
- DuplexResponse
- DuplexRequestEnd
- DuplexResponseEnd

Their use is described below.

### Messages
A message is transmitted to the peer, and that's it. Because of this, the `id` of a `Message` packet is completely irrelevant.

### Requests and Responses
A request is a way to ask for a single piece of data (the response). To respond to a `Request` packet, send a `Response` packet with the same id as the original request.

A request or a response without data signals cancellation. When receiving a request cancellation, you know that you are free to not respond to the original request, without impairing the peer. You may however ignore the cancellation and send a response anyways. Responses to cancelled requests are ignored.

When receiving a request you don't want to answer, you can signal this by sending a response without data. This way, the peer knows that it should not continue waiting for an answer. Of course, you could also simply ignore the request and never answer. Because of that, clients should specify timeouts on their requests.

### Duplexes
A duplex is a bidirectional stream. To initate a duplex, send a `DuplexInitial` packet with a fresh id. To write further data to the duplex, send any number of `DuplexRequest` packets with the same id. To terminate the duplex, send a `DuplexRequestEnd` packet with the same id. The peer will then cancel the duplex as described below. If you do not receive a `DuplexResponseEnd` packet after a certain timeout, you may simply ignore all further incoming packets for the duplex.

After receiving a `DuplexInitial` packet, you can write data to the duplex by sending any number of `DuplexResponse` packets with the same id. To cancel such a duplex, send a `DuplexResponseEnd` packet with the same id. No data signals regular termination, whereas data signals an error. Cancelling the duplex in this way only closes one half of the connection, you should not send any more packets to the duplex after sending the cancellation packet. More packets may however still be received. The peer signals that it won't send any more packets to the duplex via a `DuplexRequestEnd` packet. It should send this packet once it received the `DuplexResponseEnd` packet, and afterwards it should not send any more packets to the duplex.

### Closing the Dialogue
To allow for an [asymmetric](http://250bpm.com/blog:90) shutdown, one of the peers in a dialogue takes the `Server` role, and the other peer takes the `Client` role. How these roles are assigned is irrelevant, but usually the peer initiating the dialogue becomes the `Client` and the other peer the `Server`.

The `Client` signals closing of the dialogue by sending a `Message` packet without data. Aftwerwards, it does not send any more packets of any type. When the server receives the `Message` packet, it finishes all outstanding requests/streams and then answers with another `Message` packet without data.

The `Server` signals closing of the dialogue by sending a `Message` packet without data. It then continues to operate normally, the `Client` then initiates shutdown as described above. If the `Server` does not receive a `Message` packet without data after a certain timeout, it may simply consider the dialogue closed.
