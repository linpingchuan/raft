/// The role of the node.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum StateRole {
    /// The node is a follower of the leader.
    Follower,
    /// The node could become a leader.
    Candidate,
    /// The node is a leader.
    Leader,
    /// The node could become a candidate, if `prevote` is enabled.
    PreCandidate,
}

/// 默认状态角色为 跟随者
impl Default for StateRole{
    fn default()->StateRole{
        StateRole::Follower
    }
}

/// 此结构体对于日志跟调试非常有用
/// 这个结构体为原子性的并且不用持久化为WAL
#[derive(Default,PartialEq,Debug)]
pub struct SoftState{
    /// 默认的领导者
    pub leader_id:u64,
    /// 节点的角色
    pub raft_state:StateRole,
}

/// 此结构体用于表示Raft 一致性。
/// 存储这个系统中当前以及存在的状态的可能性
#[derive(Getters)]
pub struct Raft<T:Storage>{
    /// 当前的任期
    pub term:u64,
    /// 当前投票给对等节点的
    pub vote:u64,
    /// 当前节点的ID
    pub id:u64,
    /// 当前节点可读取的状态
    pub read_states:Vec<ReadState>,
    /// 当前持久化的日志
    pub raft_log:RaftLog<T>,
    /// 当前保存的信息
    pub max_inflight:usize,
    /// 所有信息条目最大的长度
    pub max_msg_size:u64,
    /// 对等节点获取快照
    pub pending_request_snapshot:u64,
}

/// 表示非法ID
pub const INVALID_ID:u64=0;
/// 表示日志中非法索引
pub const INVALID_INDEX:u64=0;

