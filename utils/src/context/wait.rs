use std::{future::Future, pin::Pin, sync::Arc, time::Duration};

use super::{Context as TokioContext, Handle as TokioHandle, RefContext};

pub struct Context {
    ctx: RefContext,
    _sender: Arc<tokio::sync::oneshot::Sender<()>>,
}

impl Context {
    pub fn done(&mut self) -> Pin<Box<dyn Future<Output = ()> + '_ + Send>> {
        self.ctx.done()
    }
}

impl Clone for Context {
    fn clone(&self) -> Self {
        Self {
            ctx: self.ctx.clone(),
            _sender: self._sender.clone(),
        }
    }
}

pub struct Handle {
    handle: TokioHandle,
    _parent: Option<Arc<tokio::sync::oneshot::Sender<()>>>,
    _receiver: tokio::sync::oneshot::Receiver<()>,
    _sender: Arc<tokio::sync::oneshot::Sender<()>>,
}

impl Default for Handle {
    fn default() -> Self {
        Self::new()
    }
}

impl Handle {
    pub fn new() -> Self {
        let (_, handle) = RefContext::new();

        let (sender, receiver) = tokio::sync::oneshot::channel();

        let send = Arc::new(sender);

        Self {
            handle,
            _parent: None,
            _sender: send,
            _receiver: receiver,
        }
    }

    pub fn new_from_parent(ctx: &Context, timeout: Option<Duration>) -> Handle {
        let (_, handle) = RefContext::with_parent(&ctx.ctx, timeout);

        let (sender, receiver) = tokio::sync::oneshot::channel();
        let send = Arc::new(sender);

        Handle {
            handle,
            _parent: Some(ctx._sender.clone()),
            _sender: send,
            _receiver: receiver,
        }
    }

    pub fn spawn_timeout(&mut self, timeout: Duration) -> (Context, Self) {
        let (ctx, handle) = TokioContext::with_parent(&self.handle.spawn_ref(), Some(timeout));

        let (sender, receiver) = tokio::sync::oneshot::channel();

        (
            Context {
                ctx: RefContext::from(ctx),
                _sender: self._sender.clone(),
            },
            Self {
                handle,
                _parent: Some(self._sender.clone()),
                _sender: Arc::new(sender),
                _receiver: receiver,
            },
        )
    }

    pub fn spawn(&mut self) -> Context {
        Context {
            ctx: self.handle.spawn_ref(),
            _sender: self._sender.clone(),
        }
    }

    pub fn cancel(self) -> impl Future<Output = ()> {
        self.handle.cancel();
        async move {
            let _ = self._receiver.await;
        }
    }
}
