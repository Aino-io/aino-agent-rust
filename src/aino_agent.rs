use crate::aino_config::AinoConfig;
use crate::{AinoError, Transaction};
use futures::executor::block_on;
use std::cmp::min;
use std::collections::VecDeque;
use std::sync::mpsc;
use std::sync::Mutex;
use std::thread;
use std::time::{Duration, Instant};
use surf;

enum Msg {
    Cancel,
    Trx(Box<Transaction>),
}

#[derive(Deserialize, Debug)]
struct BatchResponse {
    batch: String,
}

#[derive(Serialize)]
struct BatchRequest {
    transactions: Vec<Transaction>,
}

const MAX_BATCH_SIZE: usize = 500;

enum ListenResult {
    Continue,
    Shutdown,
}

struct Agent {
    sender: mpsc::Sender<Msg>,
    receiver: Option<mpsc::Receiver<Msg>>,
}

lazy_static! {
    static ref AGENT: Mutex<Agent> = {
        let (sender, receiver) = mpsc::channel();
        Mutex::new(Agent {
            sender: sender,
            receiver: Some(receiver),
        })
    };
}

/// Starts the [`Aino.io`](https://aino.io) agent. Should only be called once at application startup.
pub fn start(config: AinoConfig) {
    let mut agent = AGENT.lock().unwrap();
    let receiver = agent.receiver.take();
    match receiver {
        Some(receiver) => run(config, receiver),
        None => (),
    };
}

/// Adds the [`Transaction`](struct.Transaction.html) to the queue to be sent later.
pub fn add_transaction(transaction: Transaction) -> Result<(), AinoError> {
    let sender = {
        let agent = AGENT.lock().unwrap();
        agent.sender.clone()
    };

    match sender.send(Msg::Trx(Box::new(transaction))) {
        Ok(_) => Ok(()),
        Err(e) => Err(AinoError::new(format!("Aino error: {}", e))),
    }
}

/// Stops the [`Aino.io`](https://aino.io) agent. Adding any new [`Transaction`](struct.Transaction.html)s will result in an error.
pub fn stop() -> Result<(), AinoError> {
    let agent = AGENT.lock().unwrap();
    match agent.sender.send(Msg::Cancel) {
        Ok(_) => Ok(()),
        Err(e) => Err(AinoError::new(format!("Aino error: {}", e))),
    }
}

fn run(config: AinoConfig, receiver: mpsc::Receiver<Msg>) {
    thread::spawn(move || {
        let mut buffer: VecDeque<Transaction> = VecDeque::new();
        let mut interval_start = Instant::now();

        loop {
            if let ListenResult::Shutdown = listen_messages(&receiver, &mut buffer) {
                break;
            }

            if can_send_batch(&interval_start, &config, buffer.len()) {
                let batch = create_batch_request(&mut buffer);
                interval_start = Instant::now();
                block_on(send_batch(&config, batch));
            }

            thread::sleep(Duration::from_millis(10));
        }
    });
}

fn can_send_batch(interval_start: &Instant, config: &AinoConfig, buffer_len: usize) -> bool {
    buffer_len > 0
        && (interval_start.elapsed().as_millis() >= config.send_interval as u128
            || MAX_BATCH_SIZE < buffer_len)
}

fn listen_messages(
    receiver: &mpsc::Receiver<Msg>,
    buffer: &mut VecDeque<Transaction>,
) -> ListenResult {
    match receiver.try_recv() {
        Ok(msg) => match msg {
            Msg::Cancel => ListenResult::Shutdown,
            Msg::Trx(transaction) => {
                buffer.push_back(*transaction);
                ListenResult::Continue
            }
        },
        Err(e) => match e {
            mpsc::TryRecvError::Empty => ListenResult::Continue,
            mpsc::TryRecvError::Disconnected => ListenResult::Shutdown,
        },
    }
}

fn create_batch_request(buffer: &mut VecDeque<Transaction>) -> BatchRequest {
    let batch_size = min(MAX_BATCH_SIZE, buffer.len());
    BatchRequest {
        transactions: buffer.drain(..batch_size).collect(),
    }
}

async fn send_batch(config: &AinoConfig, batch: BatchRequest) {
    if let Ok(req) = surf::post(&config.url)
        .set_header("Authorization", format!("apikey {}", &config.api_key))
        .body_json(&batch)
    {
        // TODO: Implement resending if sending fails
        if let Err(e) = req.await {
            println!("Aino error: {}", e);
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Status;
    use std::iter::repeat_with;
    use std::time::SystemTime;

    fn create_config(send_interval: u32) -> AinoConfig {
        AinoConfig {
            send_interval,
            url: "".to_string(),
            api_key: "".to_string(),
        }
    }

    fn create_trx() -> Transaction {
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();
        Transaction::new(
            "from",
            "to",
            "operation",
            Status::Success,
            timestamp.as_millis(),
            "flow_id",
        )
    }

    #[test]
    fn test_can_send_batch_empty_buffer() {
        let config = create_config(10);
        let interval_start = Instant::now();
        let buffer: VecDeque<Transaction> = VecDeque::new();
        assert_eq!(
            can_send_batch(&interval_start, &config, buffer.len()),
            false
        );
    }

    #[test]
    fn test_can_send_batch_full_buffer() {
        let config = create_config(10);
        let interval_start = Instant::now();
        let buffer: VecDeque<Transaction> = repeat_with(|| create_trx())
            .take(MAX_BATCH_SIZE + 1)
            .collect();
        assert_eq!(can_send_batch(&interval_start, &config, buffer.len()), true);
    }

    #[test]
    fn test_can_send_batch_timer() {
        let config = create_config(10);
        let interval_start = Instant::now();
        let buffer: VecDeque<Transaction> = repeat_with(|| create_trx())
            .take(MAX_BATCH_SIZE - 1)
            .collect();
        thread::sleep(Duration::from_millis(11));
        assert_eq!(can_send_batch(&interval_start, &config, buffer.len()), true);
    }

    #[test]
    fn test_can_send_batch_timer_with_empty_buffer() {
        let config = create_config(10);
        let interval_start = Instant::now();
        let buffer: VecDeque<Transaction> = VecDeque::new();
        thread::sleep(Duration::from_millis(11));
        assert_eq!(
            can_send_batch(&interval_start, &config, buffer.len()),
            false
        );
    }

    #[test]
    fn test_create_batch_with_zero_transactions() {
        let mut buffer: VecDeque<Transaction> = VecDeque::new();
        assert_eq!(create_batch_request(&mut buffer).transactions.len(), 0);
    }

    #[test]
    fn test_create_batch_with_less_than_max_transactions() {
        let mut buffer: VecDeque<Transaction> = repeat_with(|| create_trx())
            .take(MAX_BATCH_SIZE - 1)
            .collect();
        assert_eq!(
            create_batch_request(&mut buffer).transactions.len(),
            MAX_BATCH_SIZE - 1
        );
        assert_eq!(buffer.len(), 0);
    }

    #[test]
    fn test_create_batch_with_more_than_max_transactions() {
        let mut buffer: VecDeque<Transaction> = repeat_with(|| create_trx())
            .take(MAX_BATCH_SIZE + 1)
            .collect();
        assert_eq!(
            create_batch_request(&mut buffer).transactions.len(),
            MAX_BATCH_SIZE
        );
        assert_eq!(buffer.len(), 1);
    }
}
