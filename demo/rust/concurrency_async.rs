use std::time::Duration;
use tokio::time::delay_for;

async fn print_message_after_delay(message: &str, delay_ms: u64) {
    delay_for(Duration::from_millis(delay_ms)).await;
    println!("{}", message);
}

#[tokio::main]
async fn main() {
    let tasks = vec![
        tokio::spawn(print_message_after_delay("Hello", 1000)),
        tokio::spawn(print_message_after_delay("world", 500)),
        tokio::spawn(print_message_after_delay("from", 250)),
        tokio::spawn(print_message_after_delay("Rust!", 125)),
    ];

    for task in tasks {
        task.await.unwrap();
    }
}
