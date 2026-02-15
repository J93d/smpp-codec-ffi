import uniffi.smpp_codec_ffi.*
import java.net.Socket
import java.io.InputStream
import java.io.OutputStream
import java.nio.ByteBuffer

const val HOST = "127.0.0.1"
const val PORT = 2775
const val HEADER_LEN = 16

// Command IDs
const val CMD_BIND_TRANSCEIVER_RESP = 0x80000009.toInt()
const val CMD_SUBMIT_SM_RESP = 0x80000004.toInt()
const val CMD_DELIVER_SM_RESP = 0x80000005.toInt()
const val CMD_UNBIND_RESP = 0x80000006.toInt()
// ... add others as needed

fun main() {
    println("Connecting to $HOST:$PORT...")
    try {
        val socket = Socket(HOST, PORT)
        val input = socket.getInputStream()
        val output = socket.getOutputStream()
        println("Connected!")

        // 1. Bind Transceiver
        println("\n--- 1. Bind Transceiver ---")
        val bindReq = BindRequest(
            sequenceNumber = 1u,
            systemId = "my_system_id",
            password = "password",
            systemType = "",
            interfaceVersion = 0x34u,
            addrTon = Ton.UNKNOWN,
            addrNpi = Npi.UNKNOWN,
            addressRange = "",
            mode = BindMode.TRANSCEIVER
        )
        sendPdu(output, encodeBindRequest(bindReq))
        readResponse(input, CMD_BIND_TRANSCEIVER_RESP)

        // 2a. Message Splitting Example (Concatenated SMS)
        println("\n--- 2a. Message Splitting (UDH) ---")
        val longMessage = "This is a very long message that needs to be split into multiple parts because it exceeds the standard SMPP short message length limit of 140-160 characters depending on the encoding used."
        
        val splitResult = splitMessage(
            text = longMessage,
            encoding = EncodingType.GSM7_BIT,
            mode = SplitMode.UDH
        )
        
        println("Split message into ${splitResult.parts.size} parts (Data Coding: ${splitResult.dataCoding})")
        
        for (i in splitResult.parts.indices) {
            val part = splitResult.parts[i]
            val partReq = SubmitSmRequest(
                sequenceNumber = (100 + i).toUInt(),
                serviceType = "CMT",
                sourceAddrTon = Ton.INTERNATIONAL,
                sourceAddrNpi = Npi.ISDN,
                sourceAddr = "123456",
                destAddrTon = Ton.NATIONAL,
                destAddrNpi = Npi.ISDN,
                destinationAddr = "9876543210",
                esmClass = 0x40u, // Indicating UDH is present
                protocolId = 0u,
                priorityFlag = 0u,
                scheduleDeliveryTime = null,
                validityPeriod = null,
                registeredDelivery = 0u,
                replaceIfPresentFlag = 0u,
                dataCoding = splitResult.dataCoding,
                smDefaultMsgId = 0u,
                shortMessage = part.toList(), // Convert to List<UByte>
                tlvs = emptyList()
            )
            println("Sending part ${i + 1}...")
            sendPdu(output, encodeSubmitSmRequest(partReq))
            readResponse(input, CMD_SUBMIT_SM_RESP)
        }

        println("\n--- 2b. Message Splitting (SAR) ---")
        val splitResultSar = splitMessage(
            text = longMessage,
            encoding = EncodingType.GSM7_BIT,
            mode = SplitMode.SAR
        )
        
        val refNum = 123u
        val totalParts = splitResultSar.parts.size.toUByte()
        
        for (i in splitResultSar.parts.indices) {
            val part = splitResultSar.parts[i]
            val sarReq = SubmitSmRequest(
                sequenceNumber = (200 + i).toUInt(),
                serviceType = "CMT",
                sourceAddrTon = Ton.INTERNATIONAL,
                sourceAddrNpi = Npi.ISDN,
                sourceAddr = "123456",
                destAddrTon = Ton.NATIONAL,
                destAddrNpi = Npi.ISDN,
                destinationAddr = "9876543210",
                esmClass = 0u,
                protocolId = 0u,
                priorityFlag = 0u,
                scheduleDeliveryTime = null,
                validityPeriod = null,
                registeredDelivery = 0u,
                replaceIfPresentFlag = 0u,
                dataCoding = splitResultSar.dataCoding,
                smDefaultMsgId = 0u,
                shortMessage = part.toList(), // Convert to List<UByte>
                tlvs = listOf(
                    tlvNewU16(Tags.SAR_MSG_REF_NUM, refNum.toUShort()),
                    tlvNewU8(Tags.SAR_TOTAL_SEGMENTS, totalParts),
                    tlvNewU8(Tags.SAR_SEGMENT_SEQNUM, (i + 1).toUByte())
                )
            )
            println("Sending part ${i + 1} (SAR)...")
            sendPdu(output, encodeSubmitSmRequest(sarReq))
            readResponse(input, CMD_SUBMIT_SM_RESP)
        }

        // 3. Unbind
        println("\n--- 3. Unbind ---")
        val unbindReq = UnbindRequest(sequenceNumber = 13u)
        sendPdu(output, encodeUnbindRequest(unbindReq))
        readResponse(input, CMD_UNBIND_RESP)

        println("\nDone!")
        socket.close()

    } catch (e: Exception) {
        println("Error: $e")
        e.printStackTrace()
    }
}

fun sendPdu(output: OutputStream, data: List<UByte>) {
    val bytes = ByteArray(data.size)
    for (i in data.indices) {
        bytes[i] = data[i].toByte()
    }
    output.write(bytes)
    println("Sent ${bytes.size} bytes")
}

fun readResponse(input: InputStream, expectedId: Int) {
    val header = ByteArray(HEADER_LEN)
    if (input.read(header) < HEADER_LEN) {
        throw Exception("Failed to read header")
    }

    val buffer = ByteBuffer.wrap(header)
    val commandLen = buffer.getInt()
    val commandId = buffer.getInt()
    val commandStatus = buffer.getInt()
    val sequenceNumber = buffer.getInt()

    println(String.format("Response: Len=%d, ID=0x%08X, Status=%d, Seq=%d", commandLen, commandId, commandStatus, sequenceNumber))

    if (commandId != expectedId) {
        println(String.format("WARNING: Expected command ID 0x%08X, got 0x%08X", expectedId, commandId))
    }

    val bodyLen = commandLen - HEADER_LEN
    if (bodyLen > 0) {
        val body = ByteArray(bodyLen)
        input.read(body)
    }
}
