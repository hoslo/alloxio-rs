use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mode {
    /// OwnerBits represents the owner access mode
    pub owner_bits: Bits,
    /// GroupBits represents the group access mode
    pub group_bits: Bits,
    /// OtherBits represents the other access mode
    pub other_bits: Bits,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all="SCREAMING_SNAKE_CASE")]
pub enum Bits {
    None,
    Execute,
    Write,
    WriteExecute,
    Read,
    ReadExecute,
    ReadWrite,
    All,
}

/// BlockInfo represents a block's information.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockInfo {
    /// BlockID is the block id.
    pub block_id: i64,
    /// Length is the block length.
    pub length: i64,
    /// Locations holds the block locations.
    pub locations: Vec<BlockLocation>,
}

/// BlockLocation represents a block's location.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockLocation {
    /// WorkerID is the worker id.
    pub worker_id: i64,
    /// WorkerAddress is the worker network address.
    pub worker_address: WorkerNetAddress,
    /// TierAlias is the tier alias.
    pub tier_alias: String,
}

/// WorkerNetAddress represents a worker's net address.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkerNetAddress {
    /// Host is the hostname.
    pub host: String,
    /// RPCPort is the RPC port.
    pub rpc_port: i32,
    /// DataPort is the data port.
    pub data_port: i32,
    /// WebPort is the web port.
    pub web_port: i32,
}

/// FileBlockInfo represents a file block's information.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileBlockInfo {
    /// BlockInfo is the block information
    pub block_info: BlockInfo,
    /// Offset is the file offset.
    pub offset: i64,
    /// UfsLocations holds the UFS locations.
    pub ufs_locations: Vec<String>,
}

/// FileInfo represents a file's information.
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FileInfo {
    /// BlockIds holds the block ids.
    pub block_ids: Vec<i64>,
    /// BlockSizeBytes is the block size (in bytes).
    pub block_size_bytes: i64,
    /// Cacheable determines whether the file is cacheable.
    pub cacheable: bool,
    /// Completed determines whether the file is completed.
    pub completed: bool,
    /// CreationTimesMs is the creation time (in milliseconds).
    pub creation_time_ms: i64,
    /// FileBlockInfos holds the file block information.
    pub file_block_infos: Vec<FileBlockInfo>,
    /// FileID is the file id.
    pub file_id: i64,
    /// Folder determines whether the file is a folder.
    pub folder: bool,
    /// Group is the group.
    pub group: String,
    /// InMemoryPercentage represents the in-memory percentage.
    pub in_memory_percentage: i32,
    /// LastModificationTimeMs is the last modification time (in milliseconds).
    pub last_modification_time_ms: i64,
    /// Length is the file length.
    pub length: i64,
    /// Name is the file name.
    pub name: String,
    /// Path is the file path.
    pub path: String,
    /// Persisted determines whether file is persisted.
    pub persisted: bool,
    /// PersistenceState represents the persistence state.
    pub persistence_state: String,
    /// Pinned determines whether the file is pinned.
    pub pinned: bool,
    /// Mode is the access mode.
    pub mode: i32,
    /// MountPoint determines whether the file is a mount point.
    pub mount_point: bool,
    /// Owner is the owner.
    pub owner: String,
    /// TTL is the time-to-live window.
    pub ttl: i64,
    /// TTLAction si the time-to-live action.
    pub ttl_action: String,
    /// UfsPath is the UFS path.
    pub ufs_path: String,
}

pub type FileInfos = Vec<FileInfo>;

/// LoadMetadataType represents the load metadata type.
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all="SCREAMING_SNAKE_CASE")]
pub enum LoadMetadataType {
    /// LoadMetadataTypeNever means metadata should never be loaded.
    Never,
    /// LoadMetadataTypeOnce means metadata should be loaded once.
    Once,
    /// LoadMetadataTypeAlways means metadata should always be loaded.
    Always,
}

/// ReadType represents a read type.
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all="SCREAMING_SNAKE_CASE")]
pub enum ReadType {
    /// ReadTypeNoCache means data will be not cached.
    NoCache,
    /// ReadTypeCache means data will be cached.
    Cache,
    /// ReadTypeCachePromote mans data will be cached in the top tier.
    CachePromote,
}


/// TTLAction represents a TTL action.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all="SCREAMING_SNAKE_CASE")]
pub enum TTLAction {
    /// TTLActionDelete represents the action of deleting a path.
    Delete,
    /// TTLActionFree represents the action of freeing a path.
    Free,
}


/// WriteType represents a write type.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all="SCREAMING_SNAKE_CASE")]
pub enum WriteType {
    /// WriteTypeMustCache means the data will be stored in Alluxio.
    MustCache,
    /// WriteTypeCacheThrough means the data will be stored in Alluxio and
    /// synchronously written to UFS.
    CacheThrough,
    /// WriteTypeThrough means the data will be sychrounously written to UFS.
    Through,
    /// WriteTypeAsyncThrough means the data will be stored in Alluxio and
    /// asynchrounously written to UFS.
    AsyncThrough,
    /// WriteTypeNone means the data will no be stored.
    None,
}
