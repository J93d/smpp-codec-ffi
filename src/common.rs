//! # UniFFI Common Definitions
//!
//! This module contains common bindings for UniFFI
use smpp_codec::common;

/// Common error type for FFI operations.
#[derive(Debug, thiserror::Error, uniffi::Error)]
pub enum SmppFfiError {
    /// Generic error with a message.
    #[error("{msg}")]
    Generic {
        /// The error message.
        msg: String,
    },
}

// --- Command IDs ---
// These constants define the Command ID for each SMPP PDU.

/// Command ID for Bind Receiver.
pub const CMD_BIND_RECEIVER: u32 = 0x00000001;
/// Command ID for Bind Receiver Response.
pub const CMD_BIND_RECEIVER_RESP: u32 = 0x80000001;

/// Command ID for Bind Transmitter.
pub const CMD_BIND_TRANSMITTER: u32 = 0x00000002;
/// Command ID for Bind Transmitter Response.
pub const CMD_BIND_TRANSMITTER_RESP: u32 = 0x80000002;

/// Command ID for Bind Transceiver.
pub const CMD_BIND_TRANSCEIVER: u32 = 0x00000009;
/// Command ID for Bind Transceiver Response.
pub const CMD_BIND_TRANSCEIVER_RESP: u32 = 0x80000009;

/// Command ID for Outbind.
pub const CMD_OUTBIND: u32 = 0x0000000B;

/// Command ID for Enquire Link.
pub const CMD_ENQUIRE_LINK: u32 = 0x00000015;
/// Command ID for Enquire Link Response.
pub const CMD_ENQUIRE_LINK_RESP: u32 = 0x80000015;

/// Command ID for Submit SM.
pub const CMD_SUBMIT_SM: u32 = 0x00000004;
/// Command ID for Submit SM Response.
pub const CMD_SUBMIT_SM_RESP: u32 = 0x80000004;

/// Command ID for Deliver SM.
pub const CMD_DELIVER_SM: u32 = 0x00000005;
/// Command ID for Deliver SM Response.
pub const CMD_DELIVER_SM_RESP: u32 = 0x80000005;

/// Command ID for Unbind.
pub const CMD_UNBIND: u32 = 0x00000006;
/// Command ID for Unbind Response.
pub const CMD_UNBIND_RESP: u32 = 0x80000006;

/// Command ID for Submit Multi SM.
pub const CMD_SUBMIT_MULTI_SM: u32 = 0x00000021;
/// Command ID for Submit Multi SM Response.
pub const CMD_SUBMIT_MULTI_SM_RESP: u32 = 0x80000021;

/// Command ID for Query SM.
pub const CMD_QUERY_SM: u32 = 0x00000003;
/// Command ID for Query SM Response.
pub const CMD_QUERY_SM_RESP: u32 = 0x80000003;

/// Command ID for Cancel SM.
pub const CMD_CANCEL_SM: u32 = 0x00000008;
/// Command ID for Cancel SM Response.
pub const CMD_CANCEL_SM_RESP: u32 = 0x80000008;

/// Command ID for Replace SM.
pub const CMD_REPLACE_SM: u32 = 0x00000007;
/// Command ID for Replace SM Response.
pub const CMD_REPLACE_SM_RESP: u32 = 0x80000007;

/// Command ID for Data SM.
pub const CMD_DATA_SM: u32 = 0x00000103;
/// Command ID for Data SM Response.
pub const CMD_DATA_SM_RESP: u32 = 0x80000103;

/// Command ID for Alert Notification.
pub const CMD_ALERT_NOTIFICATION: u32 = 0x00000102;
/// Command ID for Alert Notification Response (Note: Alert Notification does not have a standard response, but some implementations might use one).
pub const CMD_ALERT_NOTIFICATION_RESP: u32 = 0x80000102;

/// Generic NACK.
pub const GENERIC_NACK: u32 = 0x80000000;

/// Command ID for Broadcast SM.
pub const CMD_BROADCAST_SM: u32 = 0x00000111;
/// Command ID for Broadcast SM Response.
pub const CMD_BROADCAST_SM_RESP: u32 = 0x80000112;

/// Command ID for Query Broadcast SM.
pub const CMD_QUERY_BROADCAST_SM: u32 = 0x00000112;
/// Command ID for Query Broadcast SM Response.
pub const CMD_QUERY_BROADCAST_SM_RESP: u32 = 0x80000112;

/// Command ID for Cancel Broadcast SM.
pub const CMD_CANCEL_BROADCAST_SM: u32 = 0x00000113;
/// Command ID for Cancel Broadcast SM Response.
pub const CMD_CANCEL_BROADCAST_SM_RESP: u32 = 0x80000113;

/// Standard Header Length
pub const HEADER_LEN: usize = 16;

/// SMPP Interface Version
pub const SMPP_INTERFACE_VERSION_34: u8 = 0x34;
/// SMPP Interface Version 5.0
pub const SMPP_INTERFACE_VERSION_50: u8 = 0x50;

/// Address Type of Number (TON)
#[derive(uniffi::Enum, Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ton {
    /// Unknown
    Unknown = 0x00,
    /// International
    International = 0x01,
    /// National
    National = 0x02,
    /// Network Specific
    NetworkSpecific = 0x03,
    /// Subscriber Number
    SubscriberNumber = 0x04,
    /// Alphanumeric
    Alphanumeric = 0x05,
    /// Abbreviated
    Abbreviated = 0x06,
}

impl From<common::Ton> for Ton {
    fn from(value: common::Ton) -> Self {
        match value {
            common::Ton::Unknown => Ton::Unknown,
            common::Ton::International => Ton::International,
            common::Ton::National => Ton::National,
            common::Ton::NetworkSpecific => Ton::NetworkSpecific,
            common::Ton::SubscriberNumber => Ton::SubscriberNumber,
            common::Ton::Alphanumeric => Ton::Alphanumeric,
            common::Ton::Abbreviated => Ton::Abbreviated,
        }
    }
}

impl From<Ton> for common::Ton {
    fn from(value: Ton) -> Self {
        match value {
            Ton::Unknown => common::Ton::Unknown,
            Ton::International => common::Ton::International,
            Ton::National => common::Ton::National,
            Ton::NetworkSpecific => common::Ton::NetworkSpecific,
            Ton::SubscriberNumber => common::Ton::SubscriberNumber,
            Ton::Alphanumeric => common::Ton::Alphanumeric,
            Ton::Abbreviated => common::Ton::Abbreviated,
        }
    }
}

/// Address Numbering Plan Indicator (NPI)
#[derive(uniffi::Enum, Clone, Copy, Debug, PartialEq, Eq)]
pub enum Npi {
    /// Unknown
    Unknown = 0x00,
    /// ISDN
    Isdn = 0x01,
    /// Data
    Data = 0x03,
    /// Telex
    Telex = 0x04,
    /// Land Mobile
    LandMobile = 0x06,
    /// National
    National = 0x08,
    /// Private
    Private = 0x09,
    /// ERMES
    Ermes = 0x0A,
    /// Internet
    Internet = 0x0E,
    /// WAP
    Wap = 0x12,
}

impl From<common::Npi> for Npi {
    fn from(value: common::Npi) -> Self {
        match value {
            common::Npi::Unknown => Npi::Unknown,
            common::Npi::Isdn => Npi::Isdn,
            common::Npi::Data => Npi::Data,
            common::Npi::Telex => Npi::Telex,
            common::Npi::LandMobile => Npi::LandMobile,
            common::Npi::National => Npi::National,
            common::Npi::Private => Npi::Private,
            common::Npi::Ermes => Npi::Ermes,
            common::Npi::Internet => Npi::Internet,
            common::Npi::Wap => Npi::Wap,
        }
    }
}

impl From<Npi> for common::Npi {
    fn from(value: Npi) -> Self {
        match value {
            Npi::Unknown => common::Npi::Unknown,
            Npi::Isdn => common::Npi::Isdn,
            Npi::Data => common::Npi::Data,
            Npi::Telex => common::Npi::Telex,
            Npi::LandMobile => common::Npi::LandMobile,
            Npi::National => common::Npi::National,
            Npi::Private => common::Npi::Private,
            Npi::Ermes => common::Npi::Ermes,
            Npi::Internet => common::Npi::Internet,
            Npi::Wap => common::Npi::Wap,
        }
    }
}

/// Command Status
pub const COMMAND_STATUS_OK: u32 = 0x00000000;

/// Bind Mode (Receiver, Transmitter, Transceiver)
#[derive(uniffi::Enum, Clone, Copy, Debug, PartialEq, Eq)]
pub enum BindMode {
    /// Receiver Mode
    Receiver,
    /// Transmitter Mode
    Transmitter,
    /// Transceiver Mode
    Transceiver,
}

impl From<common::BindMode> for BindMode {
    fn from(value: common::BindMode) -> Self {
        match value {
            common::BindMode::Receiver => BindMode::Receiver,
            common::BindMode::Transmitter => BindMode::Transmitter,
            common::BindMode::Transceiver => BindMode::Transceiver,
        }
    }
}

impl From<BindMode> for common::BindMode {
    fn from(value: BindMode) -> Self {
        match value {
            BindMode::Receiver => common::BindMode::Receiver,
            BindMode::Transmitter => common::BindMode::Transmitter,
            BindMode::Transceiver => common::BindMode::Transceiver,
        }
    }
}
