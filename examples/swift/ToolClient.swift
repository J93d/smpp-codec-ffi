import Foundation
import Network
import smpp_codec_ffi

let HOST = "127.0.0.1"
let PORT: UInt16 = 2775
let HEADER_LEN = 16

// Command IDs
let CMD_BIND_TRANSCEIVER_RESP: UInt32 = 0x80000009
let CMD_SUBMIT_SM_RESP: UInt32 = 0x80000004
let CMD_DELIVER_SM_RESP: UInt32 = 0x80000005
let CMD_UNBIND_RESP: UInt32 = 0x80000006

func runClient() {
    print("Connecting to \(HOST):\(PORT)...")
    
    // Using simple CFStream for synchronous-like blocking example (simpler for script)
    // Or just Network.framework. Given it's a script, let's try to use standard OutputStream.
    
    var readStream: Unmanaged<CFReadStream>?
    var writeStream: Unmanaged<CFWriteStream>?
    
    CFStreamCreatePairWithSocketToHost(kCFAllocatorDefault, HOST as CFString, UInt32(PORT), &readStream, &writeStream)
    
    guard let inputStream = readStream?.takeRetainedValue(),
          let outputStream = writeStream?.takeRetainedValue() else {
        print("Failed to create streams")
        return
    }
    
    inputStream.open()
    outputStream.open()
    
    print("Connected!")
    
    defer {
        inputStream.close()
        outputStream.close()
    }
    
    // Helper to send data
    func sendData(_ data: Data) {
        _ = data.withUnsafeBytes { buffer in
             guard let ptr = buffer.bindMemory(to: UInt8.self).baseAddress else { return }
             outputStream.write(ptr, maxLength: data.count)
        }
        print("Sent \(data.count) bytes")
    }
    
    // Helper to read response
    func readResponse(expectedId: UInt32) {
        var header = [UInt8](repeating: 0, count: HEADER_LEN)
        let bytesRead = inputStream.read(&header, maxLength: HEADER_LEN)
        
        if bytesRead < HEADER_LEN {
            print("Failed to read header")
            return
        }
        
        // Parse Header (Big Endian)
        let commandLen = UInt32(header[0]) << 24 | UInt32(header[1]) << 16 | UInt32(header[2]) << 8 | UInt32(header[3])
        let commandId = UInt32(header[4]) << 24 | UInt32(header[5]) << 16 | UInt32(header[6]) << 8 | UInt32(header[7])
        let commandStatus = UInt32(header[8]) << 24 | UInt32(header[9]) << 16 | UInt32(header[10]) << 8 | UInt32(header[11])
        let sequenceNumber = UInt32(header[12]) << 24 | UInt32(header[13]) << 16 | UInt32(header[14]) << 8 | UInt32(header[15])
        
        print(String(format: "Response: Len=%d, ID=0x%08X, Status=%d, Seq=%d", commandLen, commandId, commandStatus, sequenceNumber))
        
        if commandId != expectedId {
            print(String(format: "WARNING: Expected command ID 0x%08X, got 0x%08X", expectedId, commandId))
        }
        
        let bodyLen = Int(commandLen) - HEADER_LEN
        if bodyLen > 0 {
             var body = [UInt8](repeating: 0, count: bodyLen)
             inputStream.read(&body, maxLength: bodyLen)
        }
    }
    
    // 1. Bind Transceiver
    print("\n--- 1. Bind Transceiver ---")
    let bindReq = BindRequest(
        sequenceNumber: 1,
        systemId: "my_system_id",
        password: "password",
        systemType: "",
        interfaceVersion: 0x34,
        addrTon: .unknown,
        addrNpi: .unknown,
        addressRange: "",
        mode: .transceiver
    )
    sendData(encodeBindRequest(request: bindReq))
    readResponse(expectedId: CMD_BIND_TRANSCEIVER_RESP)
    
    // 2. SubmitSm
    print("\n--- 2. SubmitSm ---")
    let submitReq = SubmitSmRequest(
        sequenceNumber: 2,
        serviceType: "CMT",
        sourceAddrTon: .international,
        sourceAddrNpi: .isdn,
        sourceAddr: "123456",
        destAddrTon: .national,
        destAddrNpi: .isdn,
        destinationAddr: "9876543210",
        esmClass: 0,
        protocolId: 0,
        priorityFlag: 1,
        scheduleDeliveryTime: "231201000000000R",
        validityPeriod: "231202000000000R",
        registeredDelivery: 1,
        replaceIfPresentFlag: 0,
        dataCoding: 0,
        smDefaultMsgId: 0,
        shortMessage: Data("Hello SubmitSm".utf8),
        tlvs: [
            tlvNewU16(tag: Tags.USER_MESSAGE_REFERENCE, value: 1),
            tlvNewU16(tag: Tags.SAR_MSG_REF_NUM, value: 1),
            tlvNewU8(tag: Tags.SAR_TOTAL_SEGMENTS, value: 2),
            tlvNewU8(tag: Tags.SAR_SEGMENT_SEQNUM, value: 1)
        ]
    )
    sendData(encodeSubmitSmRequest(request: submitReq))
    readResponse(expectedId: CMD_SUBMIT_SM_RESP)
    
    // 3. Unbind
    print("\n--- 3. Unbind ---")
    let unbindReq = UnbindRequest(sequenceNumber: 13)
    sendData(encodeUnbindRequest(request: unbindReq))
    readResponse(expectedId: CMD_UNBIND_RESP)
    
    print("\nDone!")
}

runClient()
