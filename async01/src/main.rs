use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
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
            let is_timer_running = Arc::new(AtomicBool::new(false));

            while let Some(log) = rx.recv().await {
                {
                    let mut buf = buffer.lock().await;
                    buf.push(log);
                }

                // タイマーが動いてなければ起動
                if !is_timer_running.load(Ordering::Relaxed) {
                    is_timer_running.store(true, Ordering::Relaxed);

                    let buffer_clone = Arc::clone(&buffer);
                    let is_timer_running_clone = Arc::clone(&is_timer_running);

                    tokio::spawn(async move {
                        sleep(Duration::from_secs(1)).await;

                        let mut buf = buffer_clone.lock().await;
                        println!("Flushed logs: {:?}", *buf);
                        buf.clear();

                        // タイマー終了
                        is_timer_running_clone.store(false, Ordering::Relaxed);
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

    logger.send_log("Hello1");
    tokio::time::sleep(Duration::from_millis(300)).await;

    logger.send_log("World1");
    tokio::time::sleep(Duration::from_millis(300)).await;

    logger.send_log("From Rust!1");
    tokio::time::sleep(Duration::from_secs(2)).await;
}
