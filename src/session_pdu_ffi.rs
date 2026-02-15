use crate::common::{BindMode, Npi, Ton};
use smpp_codec::common::get_status_description;
use smpp_codec::pdus::{
    BindRequest as InternalBindRequest, BindResponse as InternalBindResponse,
    EnquireLinkRequest as InternalEnquireLinkRequest,
    EnquireLinkResponse as InternalEnquireLinkResponse, GenericNack as InternalGenericNack,
    OutbindRequest as InternalOutbindRequest, UnbindRequest as InternalUnbindRequest,
    UnbindResponse as InternalUnbindResponse,
};

// -- Request ---
/// Represents a Bind request (Receiver, Transmitter, or Transceiver).
#[derive(uniffi::Record)]
pub struct BindRequest {
    /// The sequence number of the PDU.
    pub sequence_number: u32,
    /// The bind mode (Transmitter, Receiver, or Transceiver).
    pub mode: BindMode,
    /// The system_id (username).
    pub system_id: String,
    /// The password.
    pub password: String,
    /// The system type.
    pub system_type: String,
    /// The interface version.
    pub interface_version: u8,
    /// The Type of Number for the address.
    pub addr_ton: Ton,
    /// The Numbering Plan Indicator for the address.
    pub addr_npi: Npi,
    /// The address range (regex).
    pub address_range: String,
}

/// Encodes a `BindRequest` into a byte vector.
///
/// # Panics
///
/// Panics if the internal encoding fails.
#[uniffi::export]
pub fn encode_bind_request(req: &BindRequest) -> Vec<u8> {
    let internal_req = InternalBindRequest {
        sequence_number: req.sequence_number,
        mode: req.mode.into(),
        system_id: req.system_id.clone(),
        password: req.password.clone(),
        system_type: req.system_type.clone(),
        interface_version: req.interface_version,
        addr_ton: req.addr_ton.into(),
        addr_npi: req.addr_npi.into(),
        address_range: req.address_range.clone(),
    };

    let mut buffer = Vec::new();
    internal_req
        .encode(&mut buffer)
        .expect("Failed to encode BindRequest");
    buffer
}

/// Decodes a byte buffer into a `BindRequest`.
///
/// # Errors
///
/// Returns an error string if the decoding fails.
#[uniffi::export]
pub fn decode_bind_request(buffer: &[u8]) -> Result<BindRequest, String> {
    match InternalBindRequest::decode(buffer) {
        Ok(internal_resp) => Ok(BindRequest {
            sequence_number: internal_resp.sequence_number,
            mode: internal_resp.mode.into(),
            system_id: internal_resp.system_id,
            password: internal_resp.password,
            system_type: internal_resp.system_type,
            interface_version: internal_resp.interface_version,
            addr_ton: internal_resp.addr_ton.into(),
            addr_npi: internal_resp.addr_npi.into(),
            address_range: internal_resp.address_range,
        }),
        Err(e) => Err(e.to_string()),
    }
}

/// Represents an Enquire Link request.
#[derive(uniffi::Record)]
pub struct EnquireLink {
    /// The sequence number of the PDU.
    pub sequence_number: u32,
}

/// Encodes an `EnquireLink` request into a byte vector.
///
/// # Panics
///
/// Panics if the internal encoding fails.
#[uniffi::export]
pub fn encode_enquire_link(req: &EnquireLink) -> Vec<u8> {
    let internal_req = InternalEnquireLinkRequest {
        sequence_number: req.sequence_number,
    };

    let mut buffer = Vec::new();
    internal_req
        .encode(&mut buffer)
        .expect("Failed to encode EnquireLink");
    buffer
}

/// Decodes a byte buffer into an `EnquireLink` request.
///
/// # Errors
///
/// Returns an error string if the decoding fails.
#[uniffi::export]
pub fn decode_enquire_link(buffer: &[u8]) -> Result<EnquireLink, String> {
    match InternalEnquireLinkRequest::decode(buffer) {
        Ok(internal_resp) => Ok(EnquireLink {
            sequence_number: internal_resp.sequence_number,
        }),
        Err(e) => Err(e.to_string()),
    }
}

/// Represents an Outbind request.
#[derive(uniffi::Record)]
pub struct Outbind {
    /// The sequence number of the PDU.
    pub sequence_number: u32,
    /// The system_id.
    pub system_id: String,
}

/// Encodes an `Outbind` request into a byte vector.
///
/// # Panics
///
/// Panics if the internal encoding fails.
#[uniffi::export]
pub fn encode_outbind(req: &Outbind) -> Vec<u8> {
    let internal_req = InternalOutbindRequest {
        sequence_number: req.sequence_number,
        system_id: req.system_id.clone(),
        password: String::new(), // FFI struct misses password, defaulting to empty or needs update later
    };

    let mut buffer = Vec::new();
    internal_req
        .encode(&mut buffer)
        .expect("Failed to encode Outbind");
    buffer
}

/// Decodes a byte buffer into an `Outbind` request.
///
/// # Errors
///
/// Returns an error string if the decoding fails.
#[uniffi::export]
pub fn decode_outbind(buffer: &[u8]) -> Result<Outbind, String> {
    match InternalOutbindRequest::decode(buffer) {
        Ok(internal_resp) => Ok(Outbind {
            sequence_number: internal_resp.sequence_number,
            system_id: internal_resp.system_id,
        }),
        Err(e) => Err(e.to_string()),
    }
}

/// Represents an Unbind request.
#[derive(uniffi::Record)]
pub struct Unbind {
    /// The sequence number of the PDU.
    pub sequence_number: u32,
}

/// Encodes an `Unbind` request into a byte vector.
///
/// # Panics
///
/// Panics if the internal encoding fails.
#[uniffi::export]
pub fn encode_unbind(req: &Unbind) -> Vec<u8> {
    let internal_req = InternalUnbindRequest {
        sequence_number: req.sequence_number,
    };
    let mut buffer = Vec::new();
    internal_req
        .encode(&mut buffer)
        .expect("Failed to encode Unbind");
    buffer
}

/// Decodes a byte buffer into an `Unbind` request.
///
/// # Errors
///
/// Returns an error string if the decoding fails.
#[uniffi::export]
pub fn decode_unbind(buffer: &[u8]) -> Result<Unbind, String> {
    match InternalUnbindRequest::decode(buffer) {
        Ok(internal_resp) => Ok(Unbind {
            sequence_number: internal_resp.sequence_number,
        }),
        Err(e) => Err(e.to_string()),
    }
}

/// Represents a Generic Nack response.
#[derive(uniffi::Record)]
pub struct GenericNack {
    /// The sequence number of the PDU.
    pub sequence_number: u32,
    /// The command status.
    pub command_status: u32,
}

/// Encodes a `GenericNack` into a byte vector.
///
/// # Panics
///
/// Panics if the internal encoding fails.
#[uniffi::export]
pub fn encode_generic_nack(req: &GenericNack) -> Vec<u8> {
    let internal_req = InternalGenericNack {
        sequence_number: req.sequence_number,
        command_status: req.command_status,
        status_name: get_status_description(req.command_status),
    };
    let mut buffer = Vec::new();
    internal_req
        .encode(&mut buffer)
        .expect("Failed to encode GenericNack");
    buffer
}

/// Decodes a byte buffer into a `GenericNack`.
///
/// # Errors
///
/// Returns an error string if the decoding fails.
#[uniffi::export]
pub fn decode_generic_nack(buffer: &[u8]) -> Result<GenericNack, String> {
    match InternalGenericNack::decode(buffer) {
        Ok(internal_resp) => Ok(GenericNack {
            sequence_number: internal_resp.sequence_number,
            command_status: internal_resp.command_status,
        }),
        Err(e) => Err(e.to_string()),
    }
}

// --- Response ---

/// Represents a Bind response.
#[derive(uniffi::Record)]
pub struct BindResponse {
    /// The sequence number of the PDU.
    pub sequence_number: u32,
    /// The command status.
    pub command_status: u32,
    /// The system_id.
    pub system_id: String,
}

/// Encodes a `BindResponse` into a byte vector.
///
/// # Panics
///
/// Panics if the internal encoding fails.
#[uniffi::export]
pub fn encode_bind_response(req: &BindResponse) -> Vec<u8> {
    let internal_req = InternalBindResponse {
        sequence_number: req.sequence_number,
        command_status: req.command_status,
        system_id: req.system_id.clone(),
        status_description: get_status_description(req.command_status),
        command_id: 0,
        optional_params: Vec::new(),
    };
    let mut buffer = Vec::new();
    internal_req
        .encode(&mut buffer)
        .expect("Failed to encode BindResponse");
    buffer
}

/// Decodes a byte buffer into a `BindResponse`.
///
/// # Errors
///
/// Returns an error string if the decoding fails.
#[uniffi::export]
pub fn decode_bind_response(buffer: &[u8]) -> Result<BindResponse, String> {
    match InternalBindResponse::decode(buffer) {
        Ok(internal_resp) => Ok(BindResponse {
            sequence_number: internal_resp.sequence_number,
            command_status: internal_resp.command_status,
            system_id: internal_resp.system_id,
        }),
        Err(e) => Err(e.to_string()),
    }
}

/// Represents an Enquire Link response.
#[derive(uniffi::Record)]
pub struct EnquireLinkResponse {
    /// The sequence number of the PDU.
    pub sequence_number: u32,
    /// The command status.
    pub command_status: u32,
}

/// Encodes an `EnquireLinkResponse` into a byte vector.
///
/// # Panics
///
/// Panics if the internal encoding fails.
#[uniffi::export]
pub fn encode_enquire_link_response(req: &EnquireLinkResponse) -> Vec<u8> {
    let internal_req = InternalEnquireLinkResponse {
        sequence_number: req.sequence_number,
        command_status: req.command_status,
        status_description: get_status_description(req.command_status),
    };
    let mut buffer = Vec::new();
    internal_req
        .encode(&mut buffer)
        .expect("Failed to encode EnquireLinkResponse");
    buffer
}

/// Decodes a byte buffer into an `EnquireLinkResponse`.
///
/// # Errors
///
/// Returns an error string if the decoding fails.
#[uniffi::export]
pub fn decode_enquire_link_response(buffer: &[u8]) -> Result<EnquireLinkResponse, String> {
    match InternalEnquireLinkResponse::decode(buffer) {
        Ok(internal_resp) => Ok(EnquireLinkResponse {
            sequence_number: internal_resp.sequence_number,
            command_status: internal_resp.command_status,
        }),
        Err(e) => Err(e.to_string()),
    }
}

/// Represents an Unbind response.
#[derive(uniffi::Record)]
pub struct UnbindResponse {
    /// The sequence number of the PDU.
    pub sequence_number: u32,
    /// The command status.
    pub command_status: u32,
}

/// Encodes an `UnbindResponse` into a byte vector.
///
/// # Panics
///
/// Panics if the internal encoding fails.
#[uniffi::export]
pub fn encode_unbind_response(req: &UnbindResponse) -> Vec<u8> {
    let internal_req = InternalUnbindResponse {
        sequence_number: req.sequence_number,
        command_status: req.command_status,
        status_description: get_status_description(req.command_status),
    };
    let mut buffer = Vec::new();
    internal_req
        .encode(&mut buffer)
        .expect("Failed to encode UnbindResponse");
    buffer
}

/// Decodes a byte buffer into an `UnbindResponse`.
///
/// # Errors
///
/// Returns an error string if the decoding fails.
#[uniffi::export]
pub fn decode_unbind_response(buffer: &[u8]) -> Result<UnbindResponse, String> {
    match InternalUnbindResponse::decode(buffer) {
        Ok(internal_resp) => Ok(UnbindResponse {
            sequence_number: internal_resp.sequence_number,
            command_status: internal_resp.command_status,
        }),
        Err(e) => Err(e.to_string()),
    }
}
