use strum_macros::{Display, EnumString};

/// An enumeration of the different `Status` values.
#[derive(Serialize, Clone, Copy, EnumString, Display, Debug)]
#[serde(rename_all = "camelCase")]
pub enum Status {
    /// Indicates a successful [`Transaction`](struct.Transaction.html).
    #[strum(serialize = "success")]
    Success,

    /// Indicates that the [`Transaction`](struct.Transaction.html) failed.
    #[strum(serialize = "failure")]
    Failure,

    /// Indicates that something happened. This is rarely used.
    #[strum(serialize = "unknown")]
    Unknown,
}
