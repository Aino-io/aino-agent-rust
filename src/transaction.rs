use crate::status::Status;
use std::fmt;

/// A log entry for a single `Transaction` between two applications.
#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    /// The name of originating application
    pub from: String,

    /// The name of the target application
    pub to: String,

    /// A [`Status`](enum.Status.html) flag indicating the whether the `Transaction` was successful or not.
    pub status: Status,

    /// The timestamp when the `Transaction` took place, in milliseconds.
    pub timestamp: u128,

    /// The operation of the `Transaction`.
    pub operation: String,

    /// The integration segment of the `Transaction`.
    pub integration_segment: String,

    /// The ID for the whole logical flow if `Transaction`s.
    pub flow_id: String,

    /// The type of payload in the `Transaction` (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload_type: Option<String>,

    /// A possible log message (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    /// All IDs related to this `Transaction` (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<TransactionId>>,

    /// All metadata related to this `Transaction` (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Vec<TransactionMetadata>>,
}

/// Container for IDs of a single type.
#[derive(Serialize, Clone, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TransactionId {
    /// The type of the ID.
    pub id_type: String,

    /// The actual ID values for this type.
    pub values: Vec<String>,
}

/// A name/value pair for generic metadata.
#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TransactionMetadata {
    /// The name of the metadata.
    pub name: String,

    /// The value of the metadata.
    pub value: String,
}

impl Transaction {
    /// Constructs a single `Transaction` with the mandatory values.
    pub fn new(
        from: String,
        to: String,
        operation: String,
        status: Status,
        timestamp: u128,
        flow_id: String,
        integration_segment: String,
    ) -> Self {
        Transaction {
            from,
            to,
            status,
            timestamp,
            operation,
            flow_id,
            integration_segment,
            payload_type: None,
            message: None,
            ids: None,
            metadata: None,
        }
    }

    /// Adds a metadata to the `Transaction`
    pub fn add_metadata(&mut self, metadata: TransactionMetadata) -> &mut Self {
        match &mut self.metadata {
            Some(m) => m.push(metadata),
            None => self.metadata = Some(vec![metadata]),
        };

        self
    }

    /// Add an ID to the `Transaction`
    pub fn add_id(&mut self, id: TransactionId) -> &mut Self {
        match &mut self.ids {
            Some(ids) => ids.push(id),
            None => self.ids = Some(vec![id]),
        };

        self
    }
}

impl TransactionMetadata {
    /// Constructs a new [`TransactionMetadata`](struct.TransactionMetadata.html).
    pub fn new(name: String, value: String) -> Self {
        TransactionMetadata { name, value }
    }
}

impl TransactionId {
    /// Constructs a new [`TransactionId`](struct.TransactionId.html).
    pub fn new(id_type: String, values: Vec<String>) -> Self {
        TransactionId { id_type, values }
    }
}

impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match serde_json::to_string_pretty(self) {
            Ok(text) => write!(f, "{}", text),
            Err(_) => Err(fmt::Error {}),
        }
    }
}

impl fmt::Display for TransactionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match serde_json::to_string_pretty(self) {
            Ok(text) => write!(f, "{}", text),
            Err(_) => Err(fmt::Error {}),
        }
    }
}

impl fmt::Display for TransactionMetadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match serde_json::to_string_pretty(self) {
            Ok(text) => write!(f, "{}", text),
            Err(_) => Err(fmt::Error {}),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::SystemTime;

    #[test]
    fn test_add_metadata() {
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();
        let mut trx = Transaction::new(
            "from".to_string(),
            "to".to_string(),
            "operation".to_string(),
            Status::Success,
            timestamp.as_millis(),
            "flow_id".to_string(),
            "integration_segment".to_string(),
        );
        assert_eq!(trx.metadata.is_none(), true);

        let metadata = TransactionMetadata::new("name".to_string(), "value".to_string());
        trx.add_metadata(metadata);
        assert_eq!(trx.metadata.is_some(), true);
    }

    #[test]
    fn test_add_id() {
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();
        let mut trx = Transaction::new(
            "from".to_string(),
            "to".to_string(),
            "operation".to_string(),
            Status::Success,
            timestamp.as_millis(),
            "flow_id".to_string(),
            "integration_segment".to_string(),
        );
        assert_eq!(trx.ids.is_none(), true);

        let id = TransactionId::new("id_type".to_string(), vec!["value".to_string()]);
        trx.add_id(id);
        assert_eq!(trx.ids.is_some(), true);
        if let Some(ids) = &trx.ids {
            assert_eq!(ids[0].id_type, "id_type".to_string());
            assert_eq!(ids[0].values[0], "value".to_string());
        }
    }
}
