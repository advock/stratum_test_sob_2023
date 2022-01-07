//! Traits that implements very basic properties that every implementation should implements
use crate::selectors::{
    DownstreamMiningSelector, DownstreamSelector, NullDownstreamMiningSelector,
};
use common_messages_sv2::{Protocol, SetupConnection};
use std::collections::HashMap;
use std::fmt::Debug as D;

/// What define a mining downstream node at the very basic
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct CommonDownstreamData {
    pub id: u32,
    pub header_only: bool,
    pub work_selection: bool,
    pub version_rolling: bool,
}

/// SetupConnection sugared
#[derive(Debug, Copy, Clone)]
pub struct PairSettings {
    pub protocol: Protocol,
    pub min_v: u16,
    pub max_v: u16,
    pub flags: u32,
}

pub trait IsUpstream<Down: IsDownstream, Sel: DownstreamSelector<Down> + ?Sized> {
    fn get_version(&self) -> u16;
    fn get_flags(&self) -> u32;
    fn get_supported_protocols(&self) -> Vec<Protocol>;
    fn is_pairable(&self, pair_settings: &PairSettings) -> bool {
        let protocol = pair_settings.protocol;
        let min_v = pair_settings.min_v;
        let max_v = pair_settings.max_v;
        let flags = pair_settings.flags;

        let check_version = self.get_version() >= min_v && self.get_version() <= max_v;
        let check_flags = SetupConnection::check_flags(protocol, flags, self.get_flags());
        check_version && check_flags
    }
    fn get_id(&self) -> u32;
    fn get_mapper(&mut self) -> Option<&mut RequestIdMapper>;
    fn get_remote_selector(&mut self) -> &mut Sel;
}

/// General propoerties that each mining upstream that implement the Sv2 protocol should have
pub trait IsMiningUpstream<Down: IsMiningDownstream, Sel: DownstreamMiningSelector<Down> + ?Sized>:
    IsUpstream<Down, Sel>
{
    fn total_hash_rate(&self) -> u64;
    fn add_hash_rate(&mut self, to_add: u64);
}

/// General propoerties that each downstream that implement the Sv2 protocol should have
pub trait IsDownstream {
    fn get_downstream_mining_data(&self) -> CommonDownstreamData;
}

pub trait IsMiningDownstream: IsDownstream {}

/// Implemented for the NullDownstreamMiningSelector
impl<Down: IsDownstream + D> IsUpstream<Down, NullDownstreamMiningSelector> for () {
    fn get_version(&self) -> u16 {
        unreachable!("0");
    }

    fn get_flags(&self) -> u32 {
        unreachable!("1");
    }

    fn get_supported_protocols(&self) -> Vec<Protocol> {
        unreachable!("2");
    }
    fn get_id(&self) -> u32 {
        unreachable!("b");
    }

    fn get_mapper(&mut self) -> Option<&mut RequestIdMapper> {
        todo!()
    }

    fn get_remote_selector(&mut self) -> &mut NullDownstreamMiningSelector {
        todo!()
    }
}

impl<Down: IsMiningDownstream + D> IsMiningUpstream<Down, NullDownstreamMiningSelector> for () {
    fn total_hash_rate(&self) -> u64 {
        todo!()
    }

    fn add_hash_rate(&mut self, _to_add: u64) {
        todo!()
    }
}

/// Implemented for the NullDownstreamMiningSelector
impl IsDownstream for () {
    fn get_downstream_mining_data(&self) -> CommonDownstreamData {
        unreachable!("c");
    }
}

impl IsMiningDownstream for () {}

/// Proxyies likely need to change the request ids of downsteam's messages. They also need to
/// remeber original id to patch the upstream's response with it
#[derive(Debug, Default)]
pub struct RequestIdMapper {
    // upstream id -> downstream id
    request_ids_map: HashMap<u32, u32>,
    next_id: u32,
}

impl RequestIdMapper {
    pub fn new() -> Self {
        Self {
            request_ids_map: HashMap::new(),
            next_id: 0,
        }
    }

    pub fn on_open_channel(&mut self, id: u32) -> u32 {
        let new_id = self.next_id;
        self.next_id += 1;

        //let mut inner = self.request_ids_map.lock().unwrap();
        self.request_ids_map.insert(new_id, id);
        new_id
    }

    pub fn remove(&mut self, upstream_id: u32) -> u32 {
        //let mut inner = self.request_ids_map.lock().unwrap();
        self.request_ids_map.remove(&upstream_id).unwrap()
    }
}
