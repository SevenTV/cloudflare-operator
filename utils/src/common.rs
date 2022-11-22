use anyhow::{anyhow, Result};

pub fn handle_errors(
    errors: Vec<Result<Result<(), anyhow::Error>, tokio::task::JoinError>>,
) -> anyhow::Result<()> {
    let errors = errors
        .into_iter()
        .filter_map(|r| match r {
            Ok(r) => r.err(),
            Err(err) => Some(anyhow!("Task failed: {}", err)),
        })
        .collect::<Vec<_>>();

    if errors.is_empty() {
        Ok(())
    } else {
        Err(anyhow!("{:?}", errors))
    }
}

pub async fn detect_finish<T: Send>(
    tx: tokio::sync::mpsc::Sender<()>,
    f: impl std::future::Future<Output = T> + Send + 'static,
) -> T {
    let r = f.await;

    let _ = tx.try_send(());

    r
}

pub async fn debounce_notify(
    tx: tokio::sync::mpsc::Sender<()>,
    mut rx: tokio::sync::mpsc::Receiver<()>,
    duration: std::time::Duration,
) {
    let mut resend = false;
    let mut delta = duration;
    let mut last_cycle = tokio::time::Instant::now();

    loop {
        tokio::select! {
            _ = tokio::time::sleep(delta) => {
                if resend {
                    let _ = tx.try_send(());
                    resend = false;
                    delta = duration;
                }
            }
            r = rx.recv() => {
                if r.is_none() {
                    break;
                }
                if resend {
                    resend = true;
                    delta = duration - last_cycle.elapsed();
                    last_cycle = tokio::time::Instant::now();
                } else {
                    resend = true;
                    delta = duration;
                    last_cycle = tokio::time::Instant::now();
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_debounce_notify() {
        let (undebounced_send, undebounced_recv) = tokio::sync::mpsc::channel::<()>(50);
        let (debounced_send, mut debounced_recv) = tokio::sync::mpsc::channel::<()>(1);

        let handle = tokio::spawn(debounce_notify(
            debounced_send,
            undebounced_recv,
            std::time::Duration::from_millis(100),
        ));

        for _ in 0..50 {
            undebounced_send.send(()).await.unwrap();
        }

        tokio::time::sleep(std::time::Duration::from_millis(200)).await;

        assert_eq!(debounced_recv.recv().await, Some(()));

        tokio::time::sleep(std::time::Duration::from_millis(200)).await;

        assert!(debounced_recv.try_recv().is_err());

        // dropping the send should resolve the task handle
        drop(undebounced_send);

        tokio::time::sleep(std::time::Duration::from_millis(200)).await;

        assert!(handle.is_finished());
    }

    #[tokio::test]
    async fn test_detect_finish() {
        let (tx, mut rx) = tokio::sync::mpsc::channel::<()>(1);

        let handle = tokio::spawn(detect_finish(tx, async {
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;

            1
        }));

        tokio::time::sleep(std::time::Duration::from_millis(200)).await;

        assert_eq!(rx.recv().await, Some(()));

        tokio::time::sleep(std::time::Duration::from_millis(200)).await;

        assert!(handle.is_finished());

        assert_eq!(handle.await.unwrap(), 1);
    }

    #[tokio::test]
    async fn test_handle_errors() {
        let errors = vec![
            Ok(Ok(())),
            Ok(Err(anyhow!("error 1"))),
            tokio::spawn(async {
                panic!("error");
            })
            .await,
            Ok(Err(anyhow!("error 2"))),
        ];

        let r = handle_errors(errors);

        assert!(r.is_err());

        let errors = vec![Ok(Ok(())), Ok(Ok(()))];

        let r = handle_errors(errors);

        assert!(r.is_ok());
    }
}
