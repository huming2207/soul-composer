use thiserror::Error;

#[derive(Debug, Error)]
pub enum ArmError {
    #[error("Section {0} not found, which is required to be present.")]
    StubSectionNotFound(String),

    #[error("Failed to read binary data for flash device, {0}")]
    ReadBinaryInfoFail(String),

    #[error("Failed to parse ELF file")]
    ElfParse,

    #[error("FlashDevice information not found")]
    FlashDeviceInfoNotFound,
}
