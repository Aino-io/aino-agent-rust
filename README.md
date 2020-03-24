# Rust Agent for Aino.io

[![Aino-io](https://circleci.com/gh/Aino-io/aino-agent-rust.svg?style=shield&circle-token=560dca0ca6f4535ca07361caaa11809bdbdf85e8)](https://app.circleci.com/pipelines/github/Aino-io/aino-agent-rust)

Rust implementation of Aino.io logging agent.

## What is [Aino.io](http://aino.io) and what does this Agent have to do with it?

[Aino.io](http://aino.io) is an analytics and monitoring tool for integrated enterprise applications and digital
business processes. Aino.io can help organizations manage, develop, and run the digital parts of their day-to-day
business. Read more from our [web pages](http://aino.io).

Aino.io works by analyzing transactions between enterprise applications and other pieces of software.
This Agent helps to store data about the transactions to Aino.io platform using Aino.io Data API (version 2.0).
See [API documentation](http://www.aino.io/api) for detailed information about the API.

## Technical requirements
* Rust 1.39

### 1. Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
ainoio-agent = "1.0"
```

Now, you can use ainoio-agent:

```rust
use ainoio_agent;
```

### 2. Configuring the agent
Agent is configured with an TOML configuration file. Below is an example.

```rust
let config = ainoio_agent::AinoConfig::new().expect("Failed to load aino configuration");
```

##### Configuration file example
```toml
[connection]
url = "https://data.aino.io/rest/v2/transaction"
api_key = "<your api key here>"
send_interval = 1000
```

The configuration files are placed in a config-directory. They are read in the following order:
1. config/default.toml
2. config/<environment>.toml
    * the environmant is read from the RUN_MODE environment variable.
3. config/local.toml
    * This should be used to local testing, and should not be commited.
4. All environment variables prefixed with AINO.

### 3. Send a request to Aino.io:

#### Example
Logging is done by creating a `Transaction` object and passing it to the agent:
```rust
use ainoio_agent;
use std::time::SystemTime;

// Load the configuration
let config = ainoio_agent::AinoConfig::new().expect("Failed to load aino configuration");
// Start the Aino agent
// This must be called exactly once before any transactions are sent
ainoio_agent::start(config);

let timestamp = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();

// Create transaction object
let mut transaction = ainoio_agent::Transaction::new("SAP", "Card Management", "Payment", ainoio_agent::Status::Success, timestamp.as_millis(), "1249F41E55A1123FB");
transaction.message = Some("Data transfer successful.");
transaction.payload_type = Some("Product Update");

let metadata = ainoio_agent::TransactionMetadata::new("Card API", "https://somecardsystem.com");
transaction.add_metadata(metadata);

let id = ainoio_agent::TransactionId::new("OrderId", vec!["123456", "xxasd"]);
transaction.add_id(id);

// Add the transaction into the queue, it will be sent after `send_interval' has elapsed at the latests
ainoio_agent::add_transaction(transaction).expect("Failed to add transaction to the send queue.");
```

## [License](LICENSE)

Copyright &copy; 2020 [Aino.io](http://aino.io). Licensed under the [Apache 2.0 License](LICENSE).
