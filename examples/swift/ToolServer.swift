import Foundation
import smpp_codec_ffi

// Basic single-threaded blocking server for demonstration
let PORT: UInt16 = 2775
let HEADER_LEN = 16

// Command IDs
let CMD_BIND_RECEIVER: UInt32 = 0x00000001
let CMD_BIND_TRANSMITTER: UInt32 = 0x00000002
let CMD_SUBMIT_SM: UInt32 = 0x00000004
let CMD_UNBIND: UInt32 = 0x00000006
let CMD_BIND_TRANSCEIVER: UInt32 = 0x00000009

import Network

@available(macOS 10.14, *)
func startNetworkServer() {
    let listener = try! NWListener(using: .tcp, on: NWEndpoint.Port(integerLiteral: PORT))
    
    listener.newConnectionHandler = { connection in
        print("New connection")
        connection.start(queue: .main)
        handleConnection(connection)
    }
    
    listener.start(queue: .main)
    print("Server listening on port \(PORT)")
    dispatchMain()
}

@available(macOS 10.14, *)
func handleConnection(_ connection: NWConnection) {
    func receiveHeader() {
        connection.receive(minimumIncompleteLength: HEADER_LEN, maximumLength: HEADER_LEN) { data, _, isComplete, error in
            if let data = data, data.count == HEADER_LEN {
                 // Parse Header
                 let header = [UInt8](data)
                 let commandLen = UInt32(header[0]) << 24 | UInt32(header[1]) << 16 | UInt32(header[2]) << 8 | UInt32(header[3])
                 let commandId = UInt32(header[4]) << 24 | UInt32(header[5]) << 16 | UInt32(header[6]) << 8 | UInt32(header[7])
                 let sequenceNumber = UInt32(header[12]) << 24 | UInt32(header[13]) << 16 | UInt32(header[14]) << 8 | UInt32(header[15])
                 
                 print(String(format: "Header: Len=%d, ID=0x%08X", commandLen, commandId))
                 
                 let bodyLen = Int(commandLen) - HEADER_LEN
                 if bodyLen > 0 {
                     connection.receive(minimumIncompleteLength: bodyLen, maximumLength: bodyLen) { bodyData, _, _, _ in
                        if let bodyData = bodyData {
                             let fullPdu = data + bodyData
                             processPdu(connection, commandId, sequenceNumber, fullPdu)
                        }
                        receiveHeader() // Next PDU
                     }
                 } else {
                     processPdu(connection, commandId, sequenceNumber, data)
                     receiveHeader() // Next PDU
                 }
            } else {
                connection.cancel()
            }
        }
    }
    receiveHeader()
}

@available(macOS 10.14, *)
func processPdu(_ connection: NWConnection, _ commandId: UInt32, _ sequenceNumber: UInt32, _ data: Data) {
    var responseData: Data?
    
    if commandId == CMD_BIND_TRANSCEIVER {
        let req = try! decodeBindRequest(buffer: data)
        print("Received Bind: \(req.systemId)")
         let resp = BindResponse(
            sequenceNumber: sequenceNumber,
            commandId: commandId | 0x80000000,
            systemId: req.systemId,
            scInterfaceVersionTlv: nil
        )
        responseData = encodeBindResponse(response: resp)
    } else if commandId == CMD_SUBMIT_SM {
         let req = try! decodeSubmitSmRequest(buffer: data)
         print("Received SubmitSm: \(req.sourceAddr) -> \(req.destinationAddr)")
         
         // Concatenation Detection
         let isUdh = (req.esmClass & 0x40) == 0x40
         var sarRef: UInt16?
         var sarTotal: UInt8?
         var sarSeq: UInt8?
         
         for tlv in req.tlvs {
             if tlv.tag == Tags.SAR_MSG_REF_NUM {
                 if tlv.value.count >= 2 {
                     sarRef = UInt16(tlv.value[0]) << 8 | UInt16(tlv.value[1])
                 }
             } else if tlv.tag == Tags.SAR_TOTAL_SEGMENTS {
                 if !tlv.value.isEmpty {
                     sarTotal = tlv.value[0]
                 }
             } else if tlv.tag == Tags.SAR_SEGMENT_SEQNUM {
                 if !tlv.value.isEmpty {
                     sarSeq = tlv.value[0]
                 }
             }
         }
         
         if isUdh {
             print("  [DETECTION] UDH Concatenation detected")
         }
         if let ref = sarRef {
             print(String(format: "  [DETECTION] SAR Concatenation detected: Ref=%d, Part=%d/%d", ref, sarSeq ?? 0, sarTotal ?? 0))
         }

         let resp = SubmitSmResponse(
             sequenceNumber: sequenceNumber,
             messageId: "MsgID_12345"
         )
         responseData = encodeSubmitSmResponse(response: resp)
    } else if commandId == CMD_UNBIND {
        print("Received Unbind")
        let resp = UnbindResponse(sequenceNumber: sequenceNumber)
        responseData = encodeUnbindResponse(response: resp)
    }
    
    if let resp = responseData {
        connection.send(content: resp, completion: .contentProcessed({ _ in
            print("Sent Response")
        }))
    }
}

if #available(macOS 10.14, *) {
    startNetworkServer()
} else {
    print("Requires macOS 10.14+")
}
