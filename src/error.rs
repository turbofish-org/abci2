use crate::messages::abci::Response;
use error_chain::error_chain;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        MpscRecv(std::sync::mpsc::RecvError);
        MpscSend(std::sync::mpsc::SendError<Response>);
        Protobuf(protobuf::error::ProtobufError);
    }
}
