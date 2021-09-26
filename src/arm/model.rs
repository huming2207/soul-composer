use serde::{Serialize, Deserialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArmFlashStub {
    pub name: String,
    pub description: String,
    pub default: bool,
    pub instructions: String,
    pub pc_init: u32,
    pub pc_uninit: u32,
    pub pc_program_page: u32,
    pub pc_erase_sector: u32,
    pub pc_erase_all: Option<u32>,
    pub data_section_offset: u32,
    pub flash_start_addr: u32,
    pub flash_end_addr: u32,
    pub flash_page_size: u32,
    pub erased_byte_value: u32,
    pub flash_sector_size: u32,
    pub program_timeout: u32,
    pub erase_timeout: u32,
    pub ram_size: u32,
    pub flash_size: u32,
}
