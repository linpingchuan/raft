use crate::storage::Storage;
use crate::log_unstable::Unstable;

use slog::Logger;

/// Raft 日志实现
pub struct RaftLog<T:Storage>{
    /// 保存最新快照中的所有 stable 日志条目
    pub store:T,
    /// 保存所有 ustable 的日志条目与快照
    pub unstable:Unstable,
    /// 最新日志存储位置
    pub committed:u64,
    /// 公式: applied <= committed
    pub applied:u64
}

impl<T> ToString for RaftLog<T>
where 
    T:Storage
    {
        fn to_string(&self)->String{
            format!(
                "commited={}, applied={}, unstable.offset={}, unstable.entries.len()={}",
                self.committed,
                self.applied,
                self.unstable.offset,
                self.unstable.entries.len(),
            )
        }
    }

impl<T:Storage> RaftLog<T>{
    pub fn new(store:T,logger:Logger)->RaftLog<T>{
        let first_index=store.first_index().unwrap();
        let last_index=store.last_index().unwrap();
        RaftLog{
            store,
            committed:first_index-1,
            applied:first_index-1,
            unstable:Unstable::new(last_index+1,logger),
        }
    }
}