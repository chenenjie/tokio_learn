extern crate futures;
extern crate tokio_core;
extern crate tokio_service;
extern crate tokio_proto;

use std::io;
use std::str;
use tokio_core::io::{Codec, EasyBuf, Framed, Io};
use futures::future;
use futures::{Future, BoxFuture};
use tokio_proto::TcpServer;
use tokio_proto::pipeline::ServerProto;
use tokio_service::Service;

struct LineCodec;

impl Codec for LineCodec {
    type In = String;
    type Out = String;

    fn decode(&mut self, buf: &mut EasyBuf) -> io::Result<Option<Self::In>>{
        Ok(None)
    }

    fn encode(&mut self, out: Self::Out, buf :&mut Vec<u8>) -> io::Result<()> {
        Ok(())
    }
}

struct LineProto;

impl<T: Io + 'static> ServerProto<T> for LineProto {
    type Request = String;
    type Response = String;
    type Transport = Framed<T, LineCodec>;
    type BindTransport = Result<Self::Transport, io::Error>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(io.framed(LineCodec))
    }

}

struct Echo;

impl Service for Echo {
    type Request = String;
    type Response = String;
    type Error = io::Error;
    type Future = BoxFuture<Self::Response, Self::Error>;


    fn call(&self, req: Self::Request) -> Self::Future {
        future::ok(req).boxed()
    }
}




fn main() {
    let addr = "127.0.0.1:12345".parse().unwrap();

    let server = TcpServer::new(LineProto, addr);
    server.serve(|| Ok(Echo));
}