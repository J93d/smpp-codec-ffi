import smpp_codec_ffi
import unittest

class TestSmppCodec(unittest.TestCase):
    def test_submit_sm(self):
        print("Testing SubmitSm...")
        
        # Create a SubmitSmRequest
        request = smpp_codec_ffi.SubmitSmRequest(
            sequence_number=1,
            service_type="",
            source_addr_ton=smpp_codec_ffi.Ton.UNKNOWN,
            source_addr_npi=smpp_codec_ffi.Npi.UNKNOWN,
            source_addr="123456",
            dest_addr_ton=smpp_codec_ffi.Ton.INTERNATIONAL,
            dest_addr_npi=smpp_codec_ffi.Npi.ISDN,
            destination_addr="9876543210",
            esm_class=0,
            protocol_id=0,
            priority_flag=0,
            schedule_delivery_time=None,
            validity_period=None,
            registered_delivery=1,
            replace_if_present_flag=0,
            data_coding=0,
            sm_default_msg_id=0,
            short_message=b"Hello, World!",
            tlvs=[]
        )

        # Encode
        encoded_bytes = smpp_codec_ffi.encode_submit_sm_request(request)
        print(f"Encoded bytes: {encoded_bytes.hex()}")

        # Decode
        decoded_request = smpp_codec_ffi.decode_submit_sm_request(encoded_bytes)
        
        # Assert (checking fields match)
        self.assertEqual(request.sequence_number, decoded_request.sequence_number)
        self.assertEqual(request.source_addr, decoded_request.source_addr)
        self.assertEqual(request.destination_addr, decoded_request.destination_addr)
        self.assertEqual(request.short_message, decoded_request.short_message)
        
        print("SubmitSm Test Passed!")

    def test_deliver_sm(self):
        print("\nTesting DeliverSm...")
        
        request = smpp_codec_ffi.DeliverSmRequest(
            sequence_number=2,
            service_type="",
            source_addr_ton=smpp_codec_ffi.Ton.ABBREVIATED,
            source_addr_npi=smpp_codec_ffi.Npi.UNKNOWN,
            source_addr="shortcode",
            dest_addr_ton=smpp_codec_ffi.Ton.UNKNOWN,
            dest_addr_npi=smpp_codec_ffi.Npi.UNKNOWN,
            destination_addr="user",
            esm_class=0,
            protocol_id=0,
            priority_flag=0,
            schedule_delivery_time=None,
            validity_period=None,
            registered_delivery=0,
            replace_if_present_flag=0,
            data_coding=0,
            sm_default_msg_id=0,
            short_message=b"Delivery Receipt",
            tlvs=[]
        )

        encoded = smpp_codec_ffi.encode_deliver_sm(request)
        decoded = smpp_codec_ffi.decode_deliver_sm(encoded)

        self.assertEqual(request.sequence_number, decoded.sequence_number)
        self.assertEqual(request.short_message, decoded.short_message)
        print("DeliverSm Test Passed!")

if __name__ == '__main__':
    unittest.main()
