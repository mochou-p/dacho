// dacho/core/renderer/vulkan/backend/src/setup/entry.rs

use anyhow::Result;

use crate::create_log;


pub struct Entry {
    pub raw: ash::Entry
}

impl Entry {
    pub fn new() -> Result<Self> {
        create_log!(debug);

        let raw = unsafe { ash::Entry::load() }?;

        Ok(Self { raw })
    }
}

