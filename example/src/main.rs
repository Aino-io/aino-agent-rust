use ainoio_agent;
use std::thread;
use std::time::{Duration, SystemTime};

fn main() -> Result<(), ainoio_agent::AinoError> {
    // Read the configuration
    let config = ainoio_agent::AinoConfig::new()?;

    // Start the Aino.io agent, this must be done only once
    ainoio_agent::start(config)?;

    // Spawn two threads that start sending transactions to Aino.io
    // Remember to update the configuration with your actual API key
    let t = thread::spawn(|| sender());
    let t2 = thread::spawn(|| sender());
    t.join().unwrap();
    t2.join().unwrap();

    Ok(())
}

fn sender() {
    let from_application: &str = "FromApplicationName";
    let to_application: &str = "ToApplicationName";
    let operation: &str = "OperationName";
    let integration_segment: &str = "IntegrationSegmentName";
    let flow_id: &str = "FlowId";
    let id_type: &str = "IdType";
    let id1: &str = "ID1";
    let id2: &str = "ID2";

    loop {
        match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(n) => {
                // Construct the transaction
                let mut trx = ainoio_agent::Transaction::new(
                    from_application.to_string(),
                    to_application.to_string(),
                    operation.to_string(),
                    ainoio_agent::Status::Success,
                    n.as_millis(),
                    flow_id.to_string(),
                    integration_segment.to_string(),
                );

                // Add some more data
                let id_values: Vec<String> = vec![id1.to_string(), id2.to_string()];
                let id = ainoio_agent::TransactionId::new(id_type.to_string(), id_values);
                trx.add_id(id);

                // Add the transaction to the queue to be sent
                ainoio_agent::add_transaction(trx).expect("Error");
            }
            Err(_) => panic!("SystemTime before UNIX EPOCH!"),
        }
        thread::sleep(Duration::from_millis(50));
    }
}
