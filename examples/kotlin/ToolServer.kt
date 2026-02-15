import uniffi.smpp_codec_ffi.*
import java.net.ServerSocket
import java.net.Socket
import java.io.InputStream
import java.io.OutputStream
import java.nio.ByteBuffer
import kotlin.concurrent.thread

const val HOST = "127.0.0.1"
const val PORT = 2775
const val HEADER_LEN = 16

// Command IDs
const val CMD_BIND_RECEIVER = 0x00000001.toInt()
const val CMD_BIND_TRANSMITTER = 0x00000002.toInt()
const val CMD_QUERY_SM = 0x00000003.toInt()
const val CMD_SUBMIT_SM = 0x00000004.toInt()
const val CMD_DELIVER_SM = 0x00000005.toInt()
const val CMD_UNBIND = 0x00000006.toInt()
const val CMD_REPLACE_SM = 0x00000007.toInt()
const val CMD_CANCEL_SM = 0x00000008.toInt()
const val CMD_BIND_TRANSCEIVER = 0x00000009.toInt()
const val CMD_ENQUIRE_LINK = 0x00000015.toInt()
const val CMD_SUBMIT_MULTI_SM = 0x00000021.toInt()
const val CMD_ALERT_NOTIFICATION = 0x00000102.toInt()
const val CMD_DATA_SM = 0x00000103.toInt()
const val CMD_BROADCAST_SM = 0x00000111.toInt()
const val CMD_QUERY_BROADCAST_SM = 0x00000112.toInt()
const val CMD_CANCEL_BROADCAST_SM = 0x00000113.toInt()

fun main() {
    val server = ServerSocket(PORT)
    println("Server listening on port $PORT")

    while (true) {
        val client = server.accept()
        println("Client connected: ${client.inetAddress}")
        thread {
            handleClient(client)
        }
    }
}

fun handleClient(client: Socket) {
    try {
        val input = client.getInputStream()
        val output = client.getOutputStream()

        while (true) {
            val header = ByteArray(HEADER_LEN)
            if (input.read(header) < HEADER_LEN) {
                break
            }

            val buffer = ByteBuffer.wrap(header)
            val commandLen = buffer.getInt()
            val commandId = buffer.getInt()
            val commandStatus = buffer.getInt()
            val sequenceNumber = buffer.getInt()

            println(String.format("Header: Len=%d, ID=0x%08X, Status=%d, Seq=%d", commandLen, commandId, commandStatus, sequenceNumber))

            val bodyLen = commandLen - HEADER_LEN
            val body = ByteArray(bodyLen)
            if (bodyLen > 0) {
                input.read(body)
            }

            val fullPdu = header + body
            // Note: uniffi expects List<Byte> (or UByte).
            // We assume List<Byte> for compatibility with encodeToByteArray().toList()
            val fullPduList = fullPdu.toList() 

            var responsePdu: List<Byte>? = null

            when (commandId) {
                CMD_BIND_RECEIVER, CMD_BIND_TRANSMITTER, CMD_BIND_TRANSCEIVER -> {
                    val req = decodeBindRequest(fullPduList)
                    println("Received Bind: ${req.systemId}")
                    val resp = BindResponse(
                        sequenceNumber = sequenceNumber.toUInt(),
                        commandId = (commandId or -0x80000000).toUInt(), // 0x80000000 as int is negative
                        systemId = req.systemId,
                        scInterfaceVersionTlv = null
                    )
                    responsePdu = encodeBindResponse(resp)
                }
                CMD_SUBMIT_SM -> {
                    val req = decodeSubmitSmRequest(fullPduList)
                    println("Received SubmitSm: ${req.sourceAddr} -> ${req.destinationAddr}")
                    
                    val isUdh = (req.esmClass.toInt() and 0x40) == 0x40
                    var sarRef: Int? = null
                    var sarTotal: Int? = null
                    var sarSeq: Int? = null
                    
                    for (tlv in req.tlvs) {
                        when (tlv.tag.toInt()) {
                            Tags.SAR_MSG_REF_NUM.toInt() -> {
                                // Extract u16 from value (2 bytes, big endian)
                                if (tlv.value.size >= 2) {
                                    sarRef = ((tlv.value[0].toInt() and 0xFF) shl 8) or (tlv.value[1].toInt() and 0xFF)
                                }
                            }
                            Tags.SAR_TOTAL_SEGMENTS.toInt() -> {
                                if (tlv.value.isNotEmpty()) {
                                    sarTotal = tlv.value[0].toInt() and 0xFF
                                }
                            }
                            Tags.SAR_SEGMENT_SEQNUM.toInt() -> {
                                if (tlv.value.isNotEmpty()) {
                                    sarSeq = tlv.value[0].toInt() and 0xFF
                                }
                            }
                        }
                    }
                    
                    if (isUdh) {
                        println("  [DETECTION] UDH Concatenation detected")
                    }
                    if (sarRef != null) {
                        println("  [DETECTION] SAR Concatenation detected: Ref=$sarRef, Part=$sarSeq/$sarTotal")
                    }

                    val resp = SubmitSmResponse(
                        sequenceNumber = sequenceNumber.toUInt(),
                        messageId = "MsgID_12345"
                    )
                    responsePdu = encodeSubmitSmResponse(resp)
                }
                CMD_UNBIND -> {
                    println("Received Unbind")
                    val resp = UnbindResponse(sequenceNumber = sequenceNumber.toUInt())
                    responsePdu = encodeUnbindResponse(resp)
                }
                // Add other cases as needed...
                else -> {
                    println(String.format("Unknown/Unhandled Command ID: 0x%08X", commandId))
                }
            }

            if (responsePdu != null) {
                val respBytes = ByteArray(responsePdu.size)
                for (i in responsePdu.indices) {
                    respBytes[i] = responsePdu[i]
                }
                output.write(respBytes)
                println("Sent Response")
                
                if (commandId == CMD_UNBIND) {
                    break
                }
            }
        }
        client.close()
    } catch (e: Exception) {
        println("Error handling client: $e")
        e.printStackTrace()
    }
}
