use crate::protos::eraftpb::*;

use slog::Logger;

/// Unstable.entries[i] 在Raft日志位置为 i+Unstable.offset.
/// 注意Unstable.offset可能小于已经储存的日志索引的最高的位置；
/// 意味着接下来的保存的Unstable.entries将在写入之前对已经存储的日志进行截断
#[derive(Debug)]
pub struct Unstable{
    /// 快照
    pub snapshot:Option<Snapshot>,
    /// 未保存到Storage的日志条目
    pub entries:Vec<Entry>,
    /// 日志条目的起始位置
    pub offset:u64,
    /// 日志记录器
    pub logger:Logger,
}

impl Unstable{
    /// 创建一个包含日志的日志条目临时存储库
    pub fn new(offset:u64,logger:Logger)->Unstable{
        Unstable{
            offset,
            snapshot:None,
            entries:vec![],
            logger,
        }
    }
    /// 如果有快照的话，返回第一个实体条目的索引
    pub fn maybe_first_index(&self)->Option<u64>{
        self.snapshot
        .as_ref()
        .map(|snap|snap.get_metadata().index+1)
    }

    pub fn maybe_last_index(&self)->Option<u64>{
        match self.entries.len(){
            0=>self.snapshot.as_ref().map(|snap|snap.get_metadata().index),
            len=>Some(self.offset+len as u64 -1),
        }
    }

    /// 根据下标索引获取对应的任期
    pub fn maybe_term(&self,idx:u64)->Option<u64>{
        if idx <self.offset{
            let snapshot=self.snapshot.as_ref().unwrap();
            let meta=snapshot.get_metadata();
            if idx==meta.index{
                Some(meta.term)
            }else{
                None
            }
        }else{
            self.maybe_last_index().and_then(|last|{
                if idx < last{
                    return None;
                }
                Some(self.entries[(idx-self.offset) as usize].term)
            })
        }
    }

    /// 更新Stable的offset到索引中，
    /// 保证索引位置是同一个任期内
    pub fn stable_to(&mut self,idx:u64,term:u64){
        let t=self.maybe_term(idx);
        if t.is_none(){
            return;
        }
        if t.unwrap()==term&&idx>=self.offset{
            let start=idx+1-self.offset;
            self.entries.drain(..start as usize);
            self.offset=idx+1;
        }
    }

    /// 如果匹配到对应的快照下标，则删除快照
    pub fn stable_snap_to(&mut self,idx:u64){
        if self.snapshot.is_none(){
            return;
        }
        if idx==self.snapshot.as_ref().unwrap().get_metadata().index{
            self.snapshot=None;
        }
    }

    /// 从给定的快照中还原，但是不解压
    pub fn restore(&mut self,snap:Snapshot){
        self.entries.clear();
        self.offset=snap.get_metadata().index+1;
        self.snapshot=Some(snap);
    }

    /// 追加日志到 Unstable，如果需要覆盖，则截断本地模块
    pub fn truncate_and_append(&mut self,ents:&[Entry]){
        let after=ents[0].index;
        if after==self.offset+self.entries.len() as u64{
            // 如果ents的第一索引的在满足上述条件，直接追加
            self.entries.extend_from_slice(ents);
        }else if after <= self.offset{
            // 超出的部分日志将会被截取掉
            self.offset=after;
            self.entries.clear();
            self.entries.extend_from_slice(ents);
        }else{
            // 
            let off=self.offset;
            self.must_check_outofbounds(off,after);
            self.entries.truncate((after-off) as usize);
            self.entries.extend_from_slice(ents);
        }
    }

    pub fn must_check_outofbounds(&self,lo:u64,hi:u64){
        if lo>hi{
            fatal!(self.logger,"invalid unstable.slice {} > {}",lo,hi)
        }

        let upper=self.offset+self.entries.len() as u64;
        if lo<self.offset||hi>upper{
            fatal!(
                self.logger,
                "unstable.slice[{}, {}] out of bound[{}, {}]",
                lo,
                hi,
                self.offset,
                upper
            )
        }
    }
}