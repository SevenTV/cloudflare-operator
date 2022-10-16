use anyhow::{anyhow, Result};

pub mod context;
pub mod ip;
pub mod logger;
pub mod macros;

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
