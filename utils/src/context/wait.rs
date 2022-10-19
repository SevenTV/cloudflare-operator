use std::{future::Future, pin::Pin, sync::Arc, time::Duration};

use super::{Context as TokioContext, Handle as TokioHandle, RefContext};

#[derive(Clone)]
pub struct Context {
    ctx: RefContext,
    _owner: Arc<ConextOwner>,
}

struct ConextOwner {
    _parent: Option<Arc<ConextOwner>>,
    _sender: Arc<tokio::sync::oneshot::Sender<()>>,
}

impl Context {
    pub fn done(&mut self) -> Pin<Box<dyn Future<Output = ()> + '_ + Send>> {
        self.ctx.done()
    }
}

pub struct Handle {
    handle: TokioHandle,
    _owner: Arc<ConextOwner>,
    _receiver: tokio::sync::oneshot::Receiver<()>,
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
            _owner: Arc::new(ConextOwner {
                _parent: None,
                _sender: send,
            }),
            _receiver: receiver,
        }
    }

    pub fn new_from_parent(ctx: &Context, timeout: Option<Duration>) -> (Context, Self) {
        let (_ctx, handle) = RefContext::with_parent(&ctx.ctx, timeout);

        let (sender, receiver) = tokio::sync::oneshot::channel();

        let owner = Arc::new(ConextOwner {
            _parent: Some(ctx._owner.clone()),
            _sender: Arc::new(sender),
        });

        (
            Context {
                ctx: RefContext::from(_ctx),
                _owner: owner.clone(),
            },
            Self {
                handle,
                _owner: owner,
                _receiver: receiver,
            },
        )
    }

    pub fn spawn_timeout(&mut self, timeout: Duration) -> (Context, Self) {
        let (ctx, handle) = TokioContext::with_parent(&self.handle.spawn_ref(), Some(timeout));

        let (sender, receiver) = tokio::sync::oneshot::channel();

        let sender = Arc::new(sender);

        let owner = Arc::new(ConextOwner {
            _parent: Some(self._owner.clone()),
            _sender: sender,
        });

        (
            Context {
                ctx: RefContext::from(ctx),
                _owner: owner.clone(),
            },
            Self {
                handle,
                _owner: owner,
                _receiver: receiver,
            },
        )
    }

    pub fn spawn(&mut self) -> Context {
        Context {
            ctx: self.handle.spawn_ref(),
            _owner: self._owner.clone(),
        }
    }

    pub fn cancel(self) -> impl Future<Output = ()> {
        self.handle.cancel();
        let recv = self._receiver;
        async move {
            let _ = recv.await;
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;
    use tokio::select;

    use super::*;

    #[tokio::test]
    async fn test_cancel_is_held() {
        let mut handle = Handle::new();
        let mut ctx = handle.spawn();

        let done = tokio::spawn(async move {
            tokio::time::sleep(Duration::from_millis(10)).await;
            ctx.done().await;
        });

        select! {
            _ = handle.cancel() => assert!(true),
            _ = tokio::time::sleep(Duration::from_millis(100)) => assert!(false),
        }

        assert!(done.is_finished())
    }

    #[tokio::test]
    async fn test_cancel_is_held_timeout() {
        let (mut ctx, handle) = Handle::default().spawn_timeout(Duration::from_millis(10));

        let done = tokio::spawn(async move {
            tokio::time::sleep(Duration::from_millis(20)).await;
            ctx.done().await;
        });

        select! {
            _ = handle.cancel() => assert!(true),
            _ = tokio::time::sleep(Duration::from_millis(100)) => assert!(false),
        }

        assert!(done.is_finished())
    }

    #[tokio::test]
    async fn test_cancel_is_held_timeout_parent() {
        let (parent_ctx, handle) = Handle::default().spawn_timeout(Duration::from_millis(10));
        let (mut ctx, _) = Handle::new_from_parent(&parent_ctx, Some(Duration::from_millis(10)));

        let done = tokio::spawn(async move {
            tokio::time::sleep(Duration::from_millis(20)).await;
            ctx.done().await;
        });

        drop(parent_ctx);

        select! {
            _ = handle.cancel() => assert!(true),
            _ = tokio::time::sleep(Duration::from_millis(1000)) => assert!(false),
        }

        assert!(done.is_finished())
    }
}
