use std::{future::Future, pin::Pin, sync::Arc, time::Duration};

use super::{Context as TokioContext, Handle, RefContext};

pub struct SuperContext {
    ctx: RefContext,
    _sender: Arc<tokio::sync::oneshot::Sender<()>>,
}

impl SuperContext {
    pub fn new(timeout: Option<Duration>) -> (Self, SuperHandle) {
        let (ctx, handle) = match timeout {
            Some(timeout) => RefContext::with_timeout(timeout),
            None => RefContext::new(),
        };

        let (sender, receiver) = tokio::sync::oneshot::channel();

        let send = Arc::new(sender);

        (
            Self {
                ctx,
                _sender: send.clone(),
            },
            SuperHandle {
                handle,
                _parent: None,
                _sender: send,
                _receiver: receiver,
            },
        )
    }

    pub fn new_child(&self, timeout: Option<Duration>) -> (Self, SuperHandle) {
        let (ctx, handle) = RefContext::with_parent(&self.ctx, timeout);

        let (sender, receiver) = tokio::sync::oneshot::channel();
        let send = Arc::new(sender);

        (
            Self {
                ctx,
                _sender: send.clone(),
            },
            SuperHandle {
                handle,
                _parent: Some(self._sender.clone()),
                _sender: send,
                _receiver: receiver,
            },
        )
    }

    pub fn done(&mut self) -> Pin<Box<dyn Future<Output = ()> + '_ + Send>> {
        self.ctx.done()
    }
}

impl Clone for SuperContext {
    fn clone(&self) -> Self {
        Self {
            ctx: self.ctx.clone(),
            _sender: self._sender.clone(),
        }
    }
}

pub struct Context {
    ctx: TokioContext,
    _sender: Arc<tokio::sync::oneshot::Sender<()>>,
}

impl Context {
    pub fn new(parent: &SuperContext, timeout: Option<Duration>) -> Self {
        let (ctx, _) = TokioContext::with_parent(&parent.ctx, timeout);
        Self {
            ctx,
            _sender: parent._sender.clone(),
        }
    }

    pub fn done(&mut self) -> Pin<Box<dyn Future<Output = ()> + '_>> {
        self.ctx.done()
    }
}

pub struct SuperHandle {
    handle: Handle,
    _parent: Option<Arc<tokio::sync::oneshot::Sender<()>>>,
    _receiver: tokio::sync::oneshot::Receiver<()>,
    _sender: Arc<tokio::sync::oneshot::Sender<()>>,
}

impl SuperHandle {
    pub fn cancel(self) -> tokio::sync::oneshot::Receiver<()> {
        self.handle.cancel();
        self._receiver
    }

    pub fn decompose(self) -> (Handle, tokio::sync::oneshot::Receiver<()>) {
        (self.handle, self._receiver)
    }
}
