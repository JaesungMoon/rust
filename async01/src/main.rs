use std::sync::Arc;
use tokio::sync::{Mutex, mpsc};
use tokio::time::{sleep, Duration};

#[derive(Clone)]
struct Logger {
    sender: mpsc::UnboundedSender<String>,
}

impl Logger {
    fn new() -> Self {
        let (tx, mut rx) = mpsc::unbounded_channel::<String>();

        // バッファを保持するためのスレッド内タスク
        tokio::spawn(async move {
            let mut buffer = Vec::new();
            let mut timer = None;

            while let Some(log) = rx.recv().await {
                buffer.push(log);

                // タイマーが未起動なら起動
                if timer.is_none() {
                    timer = Some(tokio::spawn({
                        let mut buf = std::mem::take(&mut buffer);
                        async move {
                            sleep(Duration::from_secs(1)).await;
                            println!("Flushed logs: {:?}", buf);
                        }
                    }));
                } else {
                    // タイマー起動済み：バッファを復元
                    buffer.append(&mut tokio::task::spawn_blocking(move || {
                        buf
                    }).await.unwrap());
                }

                // タイマーが終了したらリセット
                if let Some(handle) = timer.take() {
                    let _ = handle.await;
                }
            }
        });

        Logger { sender: tx }
    }

    fn send_log(&self, msg: String) {
        let _ = self.sender.send(msg);
    }
}

#[tokio::main]
async fn main() {
    let logger = Logger::new();

    logger.send_log("Hello".into());
    tokio::time::sleep(Duration::from_millis(300)).await;

    logger.send_log("World".into());
    tokio::time::sleep(Duration::from_millis(300)).await;

    logger.send_log("From Rust!".into());
    tokio::time::sleep(Duration::from_secs(2)).await;
}
