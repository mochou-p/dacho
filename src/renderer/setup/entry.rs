// dacho/src/renderer/setup/entry.rs

use anyhow::Result;

use crate::renderer::VulkanObject;

#[cfg(debug_assertions)]
use crate::{
    application::logger::Logger,
    log
};

pub struct Entry {
    raw: ash::Entry
}

impl Entry {
    pub fn new() -> Result<Self> {
        #[cfg(debug_assertions)]
        log!(info, "Creating Entry");

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

