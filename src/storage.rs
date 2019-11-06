use crate::protos::eraftpb::*;
use crate::errors;
use crate::errors::*;
use std::sync::*;

#[derive(Debug,Clone,Default)]
pub struct RaftState{
    /// 保存最新的状态元信息，元信息包括已经提交的索引，投票的领导以及投票的任期
    pub hard_state:HardState,
    /// 记录当前节点的ID。在集群中，每个Raft节点必须要有一个唯一的ID。
    pub conf_state:ConfState,
    /// 如果节点中有成员变动，将会保存最后的状态
    pending_conf_state:Option<ConfState>,
    /// 如果 pending_conf_state 存在，将会保存 `BeginMembershipChange` 的入口下标
    pending_conf_state_start_index:Option<u64>,
}

impl RaftState{
    /// 创建一个新的 RaftState
    pub fn new(hard_state:HardState,conf_state:ConfState)->RaftState{
        RaftState{
            hard_state,
            conf_state,
            pending_conf_state:None,
            pending_conf_state_start_index:None,
        }
    }
    /// 判断 RaftState 是否已经初始化
    pub fn initialized(&self)->bool{
        self.conf_state!=ConfState::default()
    }
}

/// 该接口保存当前关于Raft实现的所有信息，
/// 包括：
/// 1.Raft 日志信息
/// 2.提交索引信息
/// 3.领导人
/// 4.投票信息等等
pub trait Storage{
    /// 调用该方法，将会初始化Raft节点，将会返回一个包含HardState和ConfState的RaftState
    /// 如果RaftState节点被初始化了，将会创建带有配置信息的RaftState节点
    /// 他的最新索引信息跟任期将会大于0
    fn initial_state(&self)->errors::Result<RaftState>;

    /// 将会返回一组区间为[low,high)日志的入口
    /// max_size 限制整个返回结果的最大长度.
    /// # Panics
    /// Panics 如果 high 大于 Storage::last_index(&self) + 1 
    fn entries(&self,low:u64,high:u64,max_size:impl Into<Option<u64>>) -> errors::Result<Vec<Entry>>;

    /// 返回任期的下标
    fn term(&self,idx:u64)->errors::Result<u64>;

    /// 返回日志索引的第一个下标，一般为下标地址+1
    /// 新创建的 Storage 将会返回1
    fn first_index(&self)->errors::Result<u64>;

    /// 将会返回最后一个下标地址
    fn last_index(&self)->errors::Result<u64>;

    /// 返回最近的一个快照
    /// 如果快照暂时不可以用，他将会返回 SnapshotTemporarilyUnavailable
    /// 一个快照的索引不能小于要求的索引
    fn snapshot(&self,request_index:u64)->errors::Result<Snapshot>;
}
/// 该结构体实例保存当前真正的状态
/// 为了使用该值，使用 `rl` 和 `wl` 函数
pub struct MemStorageCore{
    raft_state:RaftState,
    /// entries[i] = i + snapshot.get_metadata().index
    entries:Vec<Entry>,
    /// 接收到的最新快照数据的元数据
    snapshot_metadata:SnapshotMetadata,
    /// 如果为 true，下一快照将会返回 SnapshotTemporarilyUnavailable 错误.
    trigger_snap_unavailable:bool,
}

impl Default for MemStorageCore{
    fn default()->MemStorageCore{
        MemStorageCore{
            raft_state:Default::default(),
            entries:vec![],
            /// 元数据将会保存在这里
            snapshot_metadata:Default::default(),
            trigger_snap_unavailable:false,
        }
    }
}

impl MemStorageCore{
    pub fn set_hardstate(&mut self,hs:HardState){
        self.raft_state.hard_state=hs;
    }

    pub fn hard_state(&self)->&HardState{
        &self.raft_state.hard_state
    }

    pub fn mut_hard_state(&mut self)->&mut HardState{
        &mut self.raft_state.hard_state
    }

    /// 提交下标索引
    /// # Panics
    /// 如果日志中没有该条目
    pub fn commit_to(&mut self,index:u64)->errors::Result<()>{
        assert!(self.has_entry_at(index),"commit_to {} but the entry not exists",index);

        let diff=(index-self.entries[0].index)as usize;
        self.raft_state.hard_state.commit=index;
        self.raft_state.hard_state.term=self.entries[diff].term;
        Ok(())
    }

    pub fn set_conf_state(&mut self,cs:ConfState,pending_membership_change:Option<(ConfState,u64)>){
        self.raft_state.conf_state=cs;
        if let Some((cs,idx))=pending_membership_change{
            self.raft_state.pending_conf_state=Some(cs);
            self.raft_state.pending_conf_state_start_index=Some(idx);
        }
    }

    #[inline]
    fn has_entry_at(&self,index:u64)->bool{
        !self.entries.is_empty()&&index>=self.first_index()&&index<=self.last_index()
    }

    fn first_index(&self)->u64{
        match self.entries.first(){
            Some(e)=>e.index,
            None=>self.snapshot_metadata.index+1,
        }
    }

    fn last_index(&self)->u64{
        match self.entries.last(){
            Some(e)=>e.index,
            None=>self.snapshot_metadata.index,
        }
    }

    /// 使用给定的快照覆盖存储对象的内容
    /// # Panics
    /// 
    /// 如果快照索引小于存储对象的位置下标，将会导致Panics
    pub fn apply_snapshot(&mut self,mut snapshot:Snapshot)->errors::Result<()>{
        let mut meta=snapshot.take_metadata();
        let term=meta.term;
        let index=meta.index;

        if self.first_index()>index{
            return Err(Error::Store(StorageError::SnapshotOutOfDate));
        }
        Ok(())
    }
}


#[derive(Clone,Default)]
pub struct MemStorage{
    core:Arc<RwLock<MemStorageCore>>,
}

// impl Storage for MemStorage{
//     fn initial_state(&self)->errors::Result<RaftState>{
//         Ok
//     }
// }