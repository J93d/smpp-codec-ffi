use smpp_codec::common as native_common;
use smpp_codec_ffi::common::*;

#[test]
fn test_ton_conversions() {
    let ton_list = vec![
        (Ton::Unknown, native_common::Ton::Unknown),
        (Ton::International, native_common::Ton::International),
        (Ton::National, native_common::Ton::National),
        (Ton::NetworkSpecific, native_common::Ton::NetworkSpecific),
        (Ton::SubscriberNumber, native_common::Ton::SubscriberNumber),
        (Ton::Alphanumeric, native_common::Ton::Alphanumeric),
        (Ton::Abbreviated, native_common::Ton::Abbreviated),
    ];

    for (ffi, native) in ton_list {
        assert_eq!(Ton::from(native), ffi);
        assert_eq!(native_common::Ton::from(ffi), native);
    }
}

#[test]
fn test_npi_conversions() {
    let npi_list = vec![
        (Npi::Unknown, native_common::Npi::Unknown),
        (Npi::Isdn, native_common::Npi::Isdn),
        (Npi::Data, native_common::Npi::Data),
        (Npi::Telex, native_common::Npi::Telex),
        (Npi::LandMobile, native_common::Npi::LandMobile),
        (Npi::National, native_common::Npi::National),
        (Npi::Private, native_common::Npi::Private),
        (Npi::Ermes, native_common::Npi::Ermes),
        (Npi::Internet, native_common::Npi::Internet),
        (Npi::Wap, native_common::Npi::Wap),
    ];

    for (ffi, native) in npi_list {
        assert_eq!(Npi::from(native), ffi);
        assert_eq!(native_common::Npi::from(ffi), native);
    }
}

#[test]
fn test_bind_mode_conversions() {
    let mode_list = vec![
        (BindMode::Receiver, native_common::BindMode::Receiver),
        (BindMode::Transmitter, native_common::BindMode::Transmitter),
        (BindMode::Transceiver, native_common::BindMode::Transceiver),
    ];

    for (ffi, native) in mode_list {
        assert_eq!(BindMode::from(native), ffi);
        assert_eq!(native_common::BindMode::from(ffi), native);
    }
}

#[test]
fn test_error_display() {
    let err = SmppFfiError::Generic {
        msg: "test error".to_string(),
    };
    assert_eq!(format!("{}", err), "test error");
}

#[test]
fn test_constants() {
    assert_eq!(CMD_BIND_RECEIVER, 0x00000001);
    assert_eq!(CMD_BIND_RECEIVER_RESP, 0x80000001);
    assert_eq!(HEADER_LEN, 16);
    assert_eq!(SMPP_INTERFACE_VERSION_34, 0x34);
    assert_eq!(SMPP_INTERFACE_VERSION_50, 0x50);
}
