//! [`Aino.io`](https://aino.io) agent for the Rust programming language.
//!
//! [`Aino.io`](http://aino.io) is an analytics and monitoring tool for integrated enterprise applications and digital
//! business processes. Aino.io can help organizations manage, develop, and run the digital parts of their day-to-day
//! business. Read more from our [web pages](http://aino.io).
//!
//! Aino.io works by analyzing transactions between enterprise applications and other pieces of software.
//! This Agent helps to store data about the transactions to Aino.io platform using Aino.io Data API (version 2.0).
//! See [API documentation](http://www.aino.io/api) for detailed information about the API.
//!
//! #### Example
//! ```no_run
//! use ainoio_agent;
//! use std::time::SystemTime;
//!
//! // Load the configuration
//! let config = ainoio_agent::AinoConfig::new().expect("Failed to load aino configuration");
//!
//! // Start the Aino agent
//! // This must be called exactly once before any transactions are sent
//! ainoio_agent::start(config);
//!
//! let timestamp = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
//!
//! // Create transaction object
//! let mut transaction = ainoio_agent::Transaction::new("SAP".to_string(),
//!     "Card Management".to_string(), "Payment".to_string(), ainoio_agent::Status::Success,
//!     timestamp.as_millis(), "flow id".to_string());
//! transaction.message = Some("Data transfer successful.".to_string());
//! transaction.payload_type = Some("Product Update".to_string());
//!
//! let metadata = ainoio_agent::TransactionMetadata::new("Card API".to_string(), "https://somecardsystem.com".to_string());
//! transaction.add_metadata(metadata);
//!
//! let id = ainoio_agent::TransactionId::new("OrderId".to_string(), vec!["123456".to_string(), "xxasd".to_string()]);
//! transaction.add_id(id);
//!
//! // Add the transaction into the queue, it will be sent after `send_interval' has elapsed at the latests
//! ainoio_agent::add_transaction(transaction).expect("Failed to add transaction to the send queue.");
//! ```

#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;

mod aino_agent;
mod aino_config;
mod status;
mod transaction;

pub use aino_agent::*;
pub use aino_config::*;
pub use status::*;
pub use transaction::*;

use std::error::Error;
use std::fmt;

/// Error object for [`Aino.io`](https://aino.io) agent
#[derive(Debug)]
pub struct AinoError {
    msg: String,
}

impl fmt::Display for AinoError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(&self.msg)
    }
}

impl AinoError {
    /// Construct a new `AinoError`
    pub fn new(msg: String) -> Self {
        AinoError { msg }
    }
}

impl Error for AinoError {}
