use crate::protos::eraftpb::*;
use crate::errors;
use std::sync::*;

#[derive(Debug,Clone,Default)]
pub struct RaftState{
    pub hard_state:HardState,
    pub conf_state:ConfState,
}

pub struct MemStorageCore{
    raft_state:RaftState,

}

impl Default for MemStorageCore{
    fn default()->MemStorageCore{
        MemStorageCore{
            raft_state:Default::default(),
        }
    }
}

#[derive(Clone,Default)]
pub struct MemStorage{
    core:Arc<RwLock<MemStorageCore>>,
}

pub trait Storage{
    fn initial_state(&self)->errors::Result<RaftState>;
}

// impl Storage for MemStorage{
//     fn initial_state(&self)->errors::Result<RaftState>{
//         Ok
//     }
// }