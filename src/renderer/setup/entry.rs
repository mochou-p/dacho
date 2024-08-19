// dacho/src/renderer/setup/entry.rs

// crates
use anyhow::Result;

// crate
use crate::{
    renderer::{VulkanObject, LOG_SRC},
    debug
};

pub struct Entry {
    raw: ash::Entry
}

impl Entry {
    pub fn new() -> Result<Self> {
        debug!(LOG_SRC, "Creating Entry");

        let raw = unsafe { ash::Entry::load() }?;

        Ok(Self { raw })
    }
}

impl VulkanObject for Entry {
    type RawType = ash::Entry;

    fn raw(&self) -> &Self::RawType {
        &self.raw
    }
}

