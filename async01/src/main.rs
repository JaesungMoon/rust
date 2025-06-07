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

        tokio::spawn(async move {
            let buffer = Arc::new(Mutex::new(Vec::new()));
            let mut timer_running = false;

            while let Some(log) = rx.recv().await {
                let buffer_clone = buffer.clone();

                {
                    // バッファにログ追加
                    let mut buf = buffer_clone.lock().await;
                    buf.push(log);
                }

                if !timer_running {
                    timer_running = true;
                    let buffer_clone = buffer.clone();

                    tokio::spawn(async move {
                        sleep(Duration::from_secs(1)).await;

                        let mut buf = buffer_clone.lock().await;
                        println!("Flushed logs: {:?}", *buf);
                        buf.clear();
                    });
                }
            }
        });

        Logger { sender: tx }
    }

    fn send_log(&self, msg: impl Into<String>) {
        let _ = self.sender.send(msg.into());
    }
}

#[tokio::main]
async fn main() {
    let logger = Logger::new();

    logger.send_log("Hello");
    tokio::time::sleep(Duration::from_millis(300)).await;

    logger.send_log("World");
    tokio::time::sleep(Duration::from_millis(300)).await;

    logger.send_log("From Rust!");
    tokio::time::sleep(Duration::from_secs(2)).await;
}
