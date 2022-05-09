use std::{marker::PhantomData, net::SocketAddr};

use bytes::Bytes;
use serde::{de::DeserializeOwned, Serialize};

type IoError = std::io::Error;
type IoResult<T> = std::io::Result<T>;

pub const MSG_TYPE_INVALID: u32 = 0;
pub const MSG_TYPE_RPC_CALL: u32 = 1;
pub const MSG_TYPE_RPC_REPLY: u32 = 2;
pub const MSG_TYPE_OBJ: u32 = 3;
pub const MSG_TYPE_RESERVE_MAX: u32 = 100;

pub struct Chan {}

impl Chan {
    fn new() -> Self {
        // TODO
        Chan {}
    }

    pub async fn connect_addr<A>(_addr: A) -> IoResult<Chan>
    where
        A: Into<SocketAddr>,
    {
        // TODO
        Ok(Chan::new())
    }

    pub fn register<F>(&mut self, r#_type: u32, _handler: F)
    where
        F: FnMut(u32, Bytes, IoResult<u32>) + Send,
    {
        // TODO
    }

    pub fn run(self) -> ChanHandle {
        // TODO
        ChanHandle::new()
    }
}

pub struct ChanHandle {}

impl ChanHandle {
    fn new() -> ChanHandle {
        ChanHandle {}
    }

    pub async fn join(self) {
        // TODO
    }

    pub fn stop(&self) {
        // TODO
    }

    pub fn queue_msg(&self, r#_type: u32, _data: Bytes) -> IoResult<()> {
        // TODO
        Ok(())
    }
}

impl Clone for ChanHandle {
    fn clone(&self) -> Self {
        // TODO
        ChanHandle {}
    }
}

pub struct ChanServer {}

impl ChanServer {
    /// Bind to a socket addr.
    pub async fn bind_addr<A>(_addr: A) -> IoResult<ChanServer>
    where
        A: Into<SocketAddr>,
    {
        // TODO
        Ok(ChanServer {})
    }

    /// Wait for the new Chan to come.
    pub async fn accept() -> IoResult<Chan> {
        // TODO
        Ok(Chan::new())
    }
}

// Channel style interface.
pub trait TypeId {
    fn type_id() -> u32;
}

pub struct Sender<T: Serialize + TypeId> {
    _p: PhantomData<T>,
}

impl<T: Serialize + TypeId> Sender<T> {
    pub fn new(_chan: &mut Chan) -> IoResult<Sender<T>> {
        // TODO
        Ok(Self {
            _p: Default::default(),
        })
    }

    pub async fn send(&self, _data: T) -> Result<(), (IoError, T)> {
        // TODO
        Ok(())
    }
}

pub struct Receiver<T>
where
    T: DeserializeOwned + TypeId,
{
    _p: PhantomData<T>,
}

impl<T> Receiver<T>
where
    T: DeserializeOwned + TypeId,
{
    pub fn new(_chan: &mut Chan) -> IoResult<Receiver<T>> {
        // TODO
        Ok(Self {
            _p: Default::default(),
        })
    }

    pub async fn recv(&mut self) -> IoResult<T> {
        // TODO
        Err(IoError::from_raw_os_error(22))
    }
}

// Rpc style interface.
pub trait RpcCtx {
    type Arg: Serialize + DeserializeOwned;
    type Ret: Serialize + DeserializeOwned;
    fn rpc_id() -> u32;
}

pub struct RpcClient<R: RpcCtx> {
    _p: PhantomData<R>,
}

impl<R: RpcCtx> RpcClient<R> {
    pub fn new(_chan: &mut Chan) -> IoResult<RpcClient<R>> {
        // TODO
        Ok(RpcClient::<R> {
            _p: Default::default(),
        })
    }

    pub async fn call(&mut self, _arg: R::Arg) -> IoResult<R::Ret> {
        // TODO
        Err(IoError::from_raw_os_error(22))
    }
}

pub struct RpcServer<R: RpcCtx> {
    _p: PhantomData<R>,
}

impl<R: RpcCtx> RpcServer<R> {
    pub fn new(_chan: &mut Chan) -> IoResult<RpcServer<R>> {
        // TODO
        Err(IoError::from_raw_os_error(22))
    }

    pub async fn accept(&mut self) -> IoResult<(R::Arg, RpcReplier<R>)> {
        // TODO
        Err(IoError::from_raw_os_error(22))
    }
}

pub struct RpcReplier<R: RpcCtx> {
    _p: PhantomData<R>,
}

impl<R: RpcCtx> RpcReplier<R> {
    pub fn reply(self, _ret: R::Ret) -> Result<(), (R::Ret, IoError)> {
        // TODO
        Ok(())
    }
}
