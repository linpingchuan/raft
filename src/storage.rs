use crate::eraftpb::*;

pub struct RaftState{
    pub hard_state:HardState,
    pub conf_state:ConfState,
}

pub trait Storage{
    fn initial_state(&self)->Result<RaftState>;
}