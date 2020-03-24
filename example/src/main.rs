use ainoio_agent;
use std::thread;
use std::time::{Duration, SystemTime};

const FROM_APPLICATION: &str = "FromApplicationName";
const TO_APPLICATION: &str = "ToApplicationName";
const OPERATION: &str = "OperationName";
const FLOW_ID: &str = "FlowId";
const ID_TYPE: &str = "IdType";
const ID1: &str = "ID1";
const ID2: &str = "ID2";

fn main() {
    // Read the configuration
    let config = ainoio_agent::AinoConfig::new().expect("Failed to load aino configuration");

    // Start the Aino.io agent, this must be done only once
    ainoio_agent::start(config);

    // Spawn two threads that start sending transactions to Aino.io
    // Rememeber to update the configuration with your actual API key
    let t = thread::spawn(|| sender());
    let t2 = thread::spawn(|| sender());
    t.join().unwrap();
    t2.join().unwrap();
}

fn sender() {
    loop {
        match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(n) => {
                // Construct the transaction
                let mut trx = ainoio_agent::Transaction::new(
                    FROM_APPLICATION,
                    TO_APPLICATION,
                    OPERATION,
                    ainoio_agent::Status::Success,
                    n.as_millis(),
                    FLOW_ID,
                );

                // Add some more data
                let id_values: Vec<&'static str> = vec![ID1, ID2];
                let id = ainoio_agent::TransactionId::new(ID_TYPE, id_values);
                trx.add_id(id);

                // Add the transaction to the queue to be sent
                ainoio_agent::add_transaction(trx).expect("Error");
            }
            Err(_) => panic!("SystemTime before UNIX EPOCH!"),
        }
        thread::sleep(Duration::from_millis(50));
    }
}
