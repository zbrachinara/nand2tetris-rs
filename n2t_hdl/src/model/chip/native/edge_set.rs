use crate::channel_range::ChannelRange;
use crate::clock_behavior::ClockBehavior;
use derive_more::{Deref, DerefMut};
use petgraph::graph::NodeIndex;
use std::collections::hash_map::{Entry, HashMap};

#[derive(Debug, Deref, DerefMut)]
pub struct EdgeSetMap(HashMap<String, EdgeSet>);

#[derive(Debug)]
pub struct EdgeSet {
    pub input: Option<Endpoint>,
    pub outputs: Vec<Endpoint>,
}

impl EdgeSet {
    pub fn new_with(endpoint: Endpoint, as_input: bool) -> Result<Self, ()> {
        let mut new = EdgeSet {
            input: None,
            outputs: Vec::new(),
        };
        new.add(endpoint, as_input)?;
        Ok(new)
    }

    pub fn add(&mut self, endpoint: Endpoint, as_input: bool) -> Result<(), ()> {
        if as_input {
            if self.input.is_some() {
                return Err(());
            } else {
                self.input = Some(endpoint)
            }
        } else {
            self.outputs.push(endpoint)
        }

        Ok(())
    }

    pub fn iter(&self) -> Result<impl Iterator<Item = (&Endpoint, &Endpoint)>, ()> {
        self.input
            .as_ref()
            .map(|i| self.outputs.iter().map(move |x| (i, x)))
            .ok_or(())
    }
}

impl EdgeSetMap {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn insert(&mut self, k: String, v: Endpoint, input: bool) -> Result<(), ()> {
        match self.entry(k) {
            Entry::Occupied(mut e) => {
                e.get_mut().add(v, input)?;
            }
            Entry::Vacant(e) => {
                e.insert(EdgeSet::new_with(v, input)?);
            }
        };
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Endpoint {
    pub index: NodeIndex,
    pub range: ChannelRange,
    pub clocked: ClockBehavior,
}
