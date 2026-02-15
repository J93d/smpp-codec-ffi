use crate::common::SmppFfiError;
use smpp_codec::splitter as common;

/// Encoding type for message splitting.
#[derive(uniffi::Enum, Clone, Copy, Debug, PartialEq, Eq)]
pub enum EncodingType {
    /// GSM 7-bit encoding.
    Gsm7Bit,
    /// Latin-1 encoding.
    Latin1,
    /// UCS-2 (UTF-16) encoding.
    Ucs2,
}

impl From<common::EncodingType> for EncodingType {
    fn from(value: common::EncodingType) -> Self {
        match value {
            common::EncodingType::Gsm7Bit => EncodingType::Gsm7Bit,
            common::EncodingType::Latin1 => EncodingType::Latin1,
            common::EncodingType::Ucs2 => EncodingType::Ucs2,
        }
    }
}

impl From<EncodingType> for common::EncodingType {
    fn from(value: EncodingType) -> Self {
        match value {
            EncodingType::Gsm7Bit => common::EncodingType::Gsm7Bit,
            EncodingType::Latin1 => common::EncodingType::Latin1,
            EncodingType::Ucs2 => common::EncodingType::Ucs2,
        }
    }
}

/// Message splitting mode.
#[derive(uniffi::Enum, Clone, Copy, Debug, PartialEq, Eq)]
pub enum SplitMode {
    /// Use User Data Header (UDH) for concatenation.
    Udh,
    /// Use SAR (Segmentation and Reassembly) TLVs.
    Sar,
    /// Put everything in payload (no splitting, just payload).
    Payload,
}

impl From<common::SplitMode> for SplitMode {
    fn from(value: common::SplitMode) -> Self {
        match value {
            common::SplitMode::Udh => SplitMode::Udh,
            common::SplitMode::Sar => SplitMode::Sar,
            common::SplitMode::Payload => SplitMode::Payload,
        }
    }
}

impl From<SplitMode> for common::SplitMode {
    fn from(value: SplitMode) -> Self {
        match value {
            SplitMode::Udh => common::SplitMode::Udh,
            SplitMode::Sar => common::SplitMode::Sar,
            SplitMode::Payload => common::SplitMode::Payload,
        }
    }
}

/// Result of message splitting.
#[derive(uniffi::Record, Clone, Debug, PartialEq, Eq)]
pub struct SplitResult {
    /// The parts of the split message.
    pub parts: Vec<Vec<u8>>,
    /// The data coding used.
    pub data_coding: u8,
}

/// Splits a message into multiple parts based on the specified encoding and mode.
///
/// # Errors
///
/// Returns an error string if the splitting fails.
#[uniffi::export]
pub fn split_message(
    text: String,
    encoding: EncodingType,
    mode: SplitMode,
) -> Result<SplitResult, SmppFfiError> {
    let (parts, data_coding) = common::MessageSplitter::split(text, encoding.into(), mode.into())
        .map_err(|e| SmppFfiError::Generic { msg: e.to_string() })?;

    Ok(SplitResult { parts, data_coding })
}
