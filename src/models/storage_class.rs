/// The type of storage that is used. Pertains to availability, performance and cost.
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum StorageClass {
    /// Standard Storage is best for data that is frequently accessed ("hot" data) and/or stored for
    /// only brief periods of time.
    Standard,
    /// Nearline Storage is a low-cost, highly durable storage service for storing infrequently
    /// accessed data.
    Nearline,
    /// Coldline Storage is a very-low-cost, highly durable storage service for data archiving,
    /// online backup, and disaster recovery.
    Coldline,
    /// Equivalent to Standard Storage, except Multi-Regional Storage can only be used for objects
    /// stored in multi-regions or dual-regions.
    MultiRegional,
    /// Equivalent to Standard Storage, except Regional Storage can only be used for objects stored
    /// in regions.
    Regional,
    /// Similar to Standard Storage except:
    ///
    /// DRA has higher pricing for operations.
    /// DRA has lower performance, particularly in terms of availability (DRA has a 99% availability
    /// SLA).
    ///
    /// You can move your data from DRA to other storage classes by performing a storage transfer.
    DurableReducedAvailability,
}