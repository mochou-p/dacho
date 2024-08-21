// dacho/src/renderer/setup/entry.rs

// crates
use anyhow::Result;

// crate
use crate::{
    renderer::VulkanObject,
    create_log
};

pub struct Entry {
    raw: ash::Entry
}

impl Entry {
    pub fn new() -> Result<Self> {
        create_log!(debug);

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

