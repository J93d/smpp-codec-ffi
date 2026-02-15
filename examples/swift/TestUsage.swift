import Foundation
import smpp_codec_ffi

func testSubmitSm() {
    print("Testing Swift Bindings (SubmitSm)...")

    let request = SubmitSmRequest(
        sequenceNumber: 1,
        serviceType: "",
        sourceAddrTon: .unknown,
        sourceAddrNpi: .unknown,
        sourceAddr: "123456",
        destAddrTon: .international,
        destAddrNpi: .isdn,
        destinationAddr: "9876543210",
        esmClass: 0,
        protocolId: 0,
        priorityFlag: 0,
        scheduleDeliveryTime: nil,
        validityPeriod: nil,
        registeredDelivery: 1,
        replaceIfPresentFlag: 0,
        dataCoding: 0,
        smDefaultMsgId: 0,
        shortMessage: Data("Hello, World!".utf8),
        tlvs: []
    )

    print("Created Request: \(request)")

    // Encode
    let encoded = encodeSubmitSmRequest(request: request)
    print("Encoded bytes: \(encoded)")

    // Decode
    do {
        let decoded = try decodeSubmitSmRequest(buffer: encoded)
        print("Decoded Request: \(decoded)")

        if request.sequenceNumber == decoded.sequenceNumber && 
           request.sourceAddr == decoded.sourceAddr {
            print("SubmitSm Test Passed!")
        } else {
            print("SubmitSm Test FAILED!")
        }
    } catch {
        print("Decoding failed: \(error)")
    }
}

testSubmitSm()
