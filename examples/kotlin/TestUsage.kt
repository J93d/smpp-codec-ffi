import uniffi.smpp_codec_ffi.*

fun main() {
    println("Testing Kotlin Bindings...")

    try {
        // Create SubmitSmRequest
        val request = SubmitSmRequest(
            sequenceNumber = 1u,
            serviceType = "",
            sourceAddrTon = Ton.UNKNOWN,
            sourceAddrNpi = Npi.UNKNOWN,
            sourceAddr = "123456",
            destAddrTon = Ton.INTERNATIONAL,
            destAddrNpi = Npi.ISDN,
            destinationAddr = "9876543210",
            esmClass = 0u,
            protocolId = 0u,
            priorityFlag = 0u,
            scheduleDeliveryTime = null,
            validityPeriod = null,
            registeredDelivery = 1u,
            replaceIfPresentFlag = 0u,
            dataCoding = 0u,
            smDefaultMsgId = 0u,
            shortMessage = "Hello, World!".encodeToByteArray().toList(),
            tlvs = emptyList()
        )

        println("Created Request: $request")

        // Encode
        val encoded = encodeSubmitSmRequest(request)
        println("Encoded bytes: $encoded")

        // Decode
        val decoded = decodeSubmitSmRequest(encoded)
        println("Decoded Request: $decoded")

        // Simple Assertion
        if (request.sequenceNumber == decoded.sequenceNumber && 
            request.sourceAddr == decoded.sourceAddr) {
            println("SubmitSm Test Passed!")
        } else {
            println("SubmitSm Test FAILED!")
        }

    } catch (e: Exception) {
        println("Error: $e")
    }
}
