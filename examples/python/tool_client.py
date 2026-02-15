import socket
import struct
import time
import smpp_codec_ffi

# Constants
HOST = "127.0.0.1"
PORT = 2775
HEADER_LEN = 16

# Command IDs
CMD_BIND_RECEIVER_RESP = 0x80000001
CMD_BIND_TRANSMITTER_RESP = 0x80000002
CMD_QUERY_SM_RESP = 0x80000003
CMD_SUBMIT_SM_RESP = 0x80000004
CMD_DELIVER_SM_RESP = 0x80000005
CMD_UNBIND_RESP = 0x80000006
CMD_REPLACE_SM_RESP = 0x80000007
CMD_CANCEL_SM_RESP = 0x80000008
CMD_BIND_TRANSCEIVER_RESP = 0x80000009
CMD_ENQUIRE_LINK_RESP = 0x80000015
CMD_SUBMIT_MULTI_SM_RESP = 0x80000021
CMD_DATA_SM_RESP = 0x80000103
CMD_BROADCAST_SM_RESP = 0x80000112 # Note: Using 0x80000112 based on common.rs standard, but check if different
CMD_QUERY_BROADCAST_SM_RESP = 0x80000112
CMD_CANCEL_BROADCAST_SM_RESP = 0x80000113

def send_pdu(sock, encoded_pdu):
    sock.sendall(bytes(encoded_pdu))
    print(f"Sent {len(encoded_pdu)} bytes")

def read_response(sock, expected_id):
    header = sock.recv(HEADER_LEN)
    if len(header) < HEADER_LEN:
        raise Exception("Failed to read header")
    
    command_len = struct.unpack(">I", header[0:4])[0]
    command_id = struct.unpack(">I", header[4:8])[0]
    command_status = struct.unpack(">I", header[8:12])[0]
    sequence_number = struct.unpack(">I", header[12:16])[0]
    
    print(f"Response: Len={command_len}, ID=0x{command_id:08X}, Status={command_status}, Seq={sequence_number}")
    
    if command_id != expected_id:
        print(f"WARNING: Expected command ID 0x{expected_id:08X}, got 0x{command_id:08X}")
        
    body_len = command_len - HEADER_LEN
    if body_len > 0:
        body = sock.recv(body_len)
        # We can decode specific responses here if needed
    
    return command_id, command_status, sequence_number

def main():
    print(f"Connecting to {HOST}:{PORT}...")
    sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    sock.connect((HOST, PORT))
    print("Connected!")

    try:
        # 1. Bind Transceiver
        print("\n--- 1. Bind Transceiver ---")
        bind_req = smpp_codec_ffi.BindRequest(
            sequence_number=1,
            system_id="my_system_id",
            password="password",
            system_type="",
            interface_version=0x34,
            addr_ton=smpp_codec_ffi.Ton.UNKNOWN,
            addr_npi=smpp_codec_ffi.Npi.UNKNOWN,
            address_range="",
            mode=smpp_codec_ffi.BindMode.TRANSCEIVER
        )
        send_pdu(sock, smpp_codec_ffi.encode_bind_request(bind_req))
        read_response(sock, CMD_BIND_TRANSCEIVER_RESP)

        # 2a. Message Splitting Example (Concatenated SMS)
        print("\n--- 2a. Message Splitting (UDH) ---")
        long_message = "This is a very long message that needs to be split into multiple parts because it exceeds the standard SMPP short message length limit of 140-160 characters depending on the encoding used."
        
        split_result = smpp_codec_ffi.split_message(
            text=long_message,
            encoding=smpp_codec_ffi.EncodingType.GSM7_BIT,
            mode=smpp_codec_ffi.SplitMode.UDH
        )
        
        print(f"Split message into {len(split_result.parts)} parts (Data Coding: {split_result.data_coding})")
        
        for i, part in enumerate(split_result.parts):
            part_req = smpp_codec_ffi.SubmitSmRequest(
                sequence_number=100 + i,
                service_type="CMT",
                source_addr_ton=smpp_codec_ffi.Ton.INTERNATIONAL,
                source_addr_npi=smpp_codec_ffi.Npi.ISDN,
                source_addr="123456",
                dest_addr_ton=smpp_codec_ffi.Ton.NATIONAL,
                dest_addr_npi=smpp_codec_ffi.Npi.ISDN,
                destination_addr="9876543210",
                esm_class=0x40, # Indicating UDH is present
                protocol_id=0,
                priority_flag=0,
                schedule_delivery_time=None,
                validity_period=None,
                registered_delivery=0,
                replace_if_present_flag=0,
                data_coding=split_result.data_coding,
                sm_default_msg_id=0,
                short_message=part,
                tlvs=[]
            )
            print(f"Sending part {i+1}...")
            send_pdu(sock, smpp_codec_ffi.encode_submit_sm_request(part_req))
            read_response(sock, CMD_SUBMIT_SM_RESP)

        print("\n--- 2b. Message Splitting (SAR) ---")
        split_result_sar = smpp_codec_ffi.split_message(
            text=long_message,
            encoding=smpp_codec_ffi.EncodingType.GSM7_BIT,
            mode=smpp_codec_ffi.SplitMode.SAR
        )
        
        # In SAR mode, the parts are just the payloads. We need to add SAR TLVs.
        # sar_msg_ref_num would typically be a random value for the whole message
        ref_num = 123
        total_parts = len(split_result_sar.parts)
        
        for i, part in enumerate(split_result_sar.parts):
            sar_req = smpp_codec_ffi.SubmitSmRequest(
                sequence_number=200 + i,
                service_type="CMT",
                source_addr_ton=smpp_codec_ffi.Ton.INTERNATIONAL,
                source_addr_npi=smpp_codec_ffi.Npi.ISDN,
                source_addr="123456",
                dest_addr_ton=smpp_codec_ffi.Ton.NATIONAL,
                dest_addr_npi=smpp_codec_ffi.Npi.ISDN,
                destination_addr="9876543210",
                esm_class=0x00, # No UDH
                protocol_id=0,
                priority_flag=0,
                schedule_delivery_time=None,
                validity_period=None,
                registered_delivery=0,
                replace_if_present_flag=0,
                data_coding=split_result_sar.data_coding,
                sm_default_msg_id=0,
                short_message=part,
                tlvs=[
                    smpp_codec_ffi.tlv_new_u16(smpp_codec_ffi.Tags.SAR_MSG_REF_NUM, ref_num),
                    smpp_codec_ffi.tlv_new_u8(smpp_codec_ffi.Tags.SAR_TOTAL_SEGMENTS, total_parts),
                    smpp_codec_ffi.tlv_new_u8(smpp_codec_ffi.Tags.SAR_SEGMENT_SEQNUM, i + 1)
                ]
            )
            print(f"Sending part {i+1} (SAR)...")
            send_pdu(sock, smpp_codec_ffi.encode_submit_sm_request(sar_req))
            read_response(sock, CMD_SUBMIT_SM_RESP)

        # 3. DeliverSm
        print("\n--- 3. DeliverSm ---")
        deliver_req = smpp_codec_ffi.DeliverSmRequest(
            sequence_number=3,
            service_type="CMT",
            source_addr_ton=smpp_codec_ffi.Ton.INTERNATIONAL,
            source_addr_npi=smpp_codec_ffi.Npi.ISDN,
            source_addr="123456",
            dest_addr_ton=smpp_codec_ffi.Ton.NATIONAL,
            dest_addr_npi=smpp_codec_ffi.Npi.ISDN,
            destination_addr="9876543210",
            esm_class=0,
            protocol_id=0,
            priority_flag=0,
            schedule_delivery_time="",
            validity_period="",
            registered_delivery=0,
            replace_if_present_flag=0,
            data_coding=0,
            sm_default_msg_id=0,
            short_message=b"Hello DeliverSm",
            tlvs=[
                 smpp_codec_ffi.tlv_new(smpp_codec_ffi.Tags.NETWORK_ERROR_CODE, [3, 0, 0])
            ]
        )
        send_pdu(sock, smpp_codec_ffi.encode_deliver_sm(deliver_req))
        read_response(sock, CMD_DELIVER_SM_RESP)

        # 4. BroadcastSm
        print("\n--- 4. BroadcastSm ---")
        broadcast_req = smpp_codec_ffi.BroadcastSmRequest(
            sequence_number=4,
            service_type="CMT",
            source_addr_ton=smpp_codec_ffi.Ton.INTERNATIONAL,
            source_addr_npi=smpp_codec_ffi.Npi.ISDN,
            source_addr="123456",
            message_id="",
            priority_flag=2,
            schedule_delivery_time="",
            validity_period="",
            replace_if_present_flag=0,
            data_coding=0,
            sm_default_msg_id=0,
            broadcast_area_identifier=smpp_codec_ffi.tlv_new(smpp_codec_ffi.Tags.BROADCAST_AREA_IDENTIFIER, [1, 2, 3]),
            broadcast_content_type=smpp_codec_ffi.tlv_new(smpp_codec_ffi.Tags.BROADCAST_CONTENT_TYPE, [0, 0, 0]), # Dummy
            broadcast_rep_num=smpp_codec_ffi.tlv_new_u16(smpp_codec_ffi.Tags.BROADCAST_REP_NUM, 1),
            broadcast_frequency_interval=smpp_codec_ffi.tlv_new_u16(smpp_codec_ffi.Tags.BROADCAST_FREQUENCY_INTERVAL, 0),
            short_message=b"Hello Broadcast",
            tlvs=[]
        )
        send_pdu(sock, smpp_codec_ffi.encode_broadcast_sm_request(broadcast_req))
        read_response(sock, CMD_BROADCAST_SM_RESP)

        # 5. SubmitMulti
        print("\n--- 5. SubmitMulti ---")
        dest1 = smpp_codec_ffi.Destination.SME_ADDRESS(
            ton=smpp_codec_ffi.Ton.INTERNATIONAL,
            npi=smpp_codec_ffi.Npi.ISDN,
            address="111111"
        )
        dest2 = smpp_codec_ffi.Destination.DISTRIBUTION_LIST(name="MyList")
        multi_req = smpp_codec_ffi.SubmitMultiRequest(
            sequence_number=5,
            service_type="CMT",
            source_addr_ton=smpp_codec_ffi.Ton.INTERNATIONAL,
            source_addr_npi=smpp_codec_ffi.Npi.ISDN,
            source_addr="source_addr",
            dest_address=[dest1, dest2],
            esm_class=0,
            protocol_id=0,
            priority_flag=0,
            schedule_delivery_time="",
            validity_period="",
            registered_delivery=0,
            replace_if_present_flag=0,
            data_coding=0,
            sm_default_msg_id=0,
            short_message=b"Hello SubmitMulti",
            tlvs=[]
        )
        send_pdu(sock, smpp_codec_ffi.encode_submit_multi_request(multi_req))
        read_response(sock, CMD_SUBMIT_MULTI_SM_RESP)

        # 6. QuerySm
        print("\n--- 6. QuerySm ---")
        query_req = smpp_codec_ffi.QuerySmRequest(
            sequence_number=6,
            message_id="MsgID_12345",
            source_addr_ton=smpp_codec_ffi.Ton.UNKNOWN,
            source_addr_npi=smpp_codec_ffi.Npi.UNKNOWN,
            source_addr="source_addr"
        )
        send_pdu(sock, smpp_codec_ffi.encode_query_sm_request(query_req))
        read_response(sock, CMD_QUERY_SM_RESP)

        # 7. CancelSm
        print("\n--- 7. CancelSm ---")
        cancel_req = smpp_codec_ffi.CancelSmRequest(
            sequence_number=7,
            service_type="",
            message_id="MsgID_12345",
            source_addr_ton=smpp_codec_ffi.Ton.UNKNOWN,
            source_addr_npi=smpp_codec_ffi.Npi.UNKNOWN,
            source_addr="source_addr",
            dest_addr_ton=smpp_codec_ffi.Ton.UNKNOWN,
            dest_addr_npi=smpp_codec_ffi.Npi.UNKNOWN,
            destination_addr="dest_addr"
        )
        send_pdu(sock, smpp_codec_ffi.encode_cancel_sm_request(cancel_req))
        read_response(sock, CMD_CANCEL_SM_RESP)

        # 8. ReplaceSm
        print("\n--- 8. ReplaceSm ---")
        replace_req = smpp_codec_ffi.ReplaceSmRequest(
            sequence_number=8,
            message_id="MsgID_12345",
            source_addr_ton=smpp_codec_ffi.Ton.UNKNOWN,
            source_addr_npi=smpp_codec_ffi.Npi.UNKNOWN,
            source_addr="source_addr",
            schedule_delivery_time="",
            validity_period="",
            registered_delivery=0,
            sm_default_msg_id=0,
            short_message=b"New Content"
        )
        send_pdu(sock, smpp_codec_ffi.encode_replace_sm_request(replace_req))
        read_response(sock, CMD_REPLACE_SM_RESP)

        # 9. DataSm
        print("\n--- 9. DataSm ---")
        data_req = smpp_codec_ffi.DataSm(
            sequence_number=9,
            service_type="",
            source_addr_ton=smpp_codec_ffi.Ton.UNKNOWN,
            source_addr_npi=smpp_codec_ffi.Npi.UNKNOWN,
            source_addr="source_addr",
            dest_addr_ton=smpp_codec_ffi.Ton.UNKNOWN,
            dest_addr_npi=smpp_codec_ffi.Npi.UNKNOWN,
            dest_addr="dest_addr",
            esm_class=0,
            registered_delivery=0,
            data_coding=0,
            optional_params=[]
        )
        send_pdu(sock, smpp_codec_ffi.encode_data_sm(data_req))
        read_response(sock, CMD_DATA_SM_RESP)

        # 10. AlertNotification
        print("\n--- 10. AlertNotification ---")
        alert_req = smpp_codec_ffi.AlertNotification(
            sequence_number=10,
            source_addr_ton=smpp_codec_ffi.Ton.UNKNOWN,
            source_addr_npi=smpp_codec_ffi.Npi.UNKNOWN,
            source_addr="source_addr",
            esme_addr_ton=smpp_codec_ffi.Ton.UNKNOWN,
            esme_addr_npi=smpp_codec_ffi.Npi.UNKNOWN,
            esme_addr="esme_addr",
            optional_params=[]
        )
        send_pdu(sock, smpp_codec_ffi.encode_alert_notification(alert_req))
        # No response expected

        # 11. QueryBroadcastSm
        print("\n--- 11. QueryBroadcastSm ---")
        query_bc_req = smpp_codec_ffi.QueryBroadcastSmRequest(
             sequence_number=11,
             message_id="BcastID_999",
             source_addr_ton=smpp_codec_ffi.Ton.UNKNOWN,
             source_addr_npi=smpp_codec_ffi.Npi.UNKNOWN,
             source_addr="source_addr",
             optional_params=[]
        )
        send_pdu(sock, smpp_codec_ffi.encode_query_broadcast_sm_request(query_bc_req))
        read_response(sock, CMD_QUERY_BROADCAST_SM_RESP)

        # 12. CancelBroadcastSm
        print("\n--- 12. CancelBroadcastSm ---")
        cancel_bc_req = smpp_codec_ffi.CancelBroadcastSmRequest(
            sequence_number=12,
            service_type="CMT",
            message_id="BcastID_999",
            source_addr_ton=smpp_codec_ffi.Ton.UNKNOWN,
            source_addr_npi=smpp_codec_ffi.Npi.UNKNOWN,
            source_addr="source_addr",
            optional_params=[]
        )
        send_pdu(sock, smpp_codec_ffi.encode_cancel_broadcast_sm_request(cancel_bc_req))
        read_response(sock, CMD_CANCEL_BROADCAST_SM_RESP)

        # 13. Unbind
        print("\n--- 13. Unbind ---")
        unbind_req = smpp_codec_ffi.UnbindRequest(
            sequence_number=13
        )
        send_pdu(sock, smpp_codec_ffi.encode_unbind_request(unbind_req))
        read_response(sock, CMD_UNBIND_RESP)

        print("\nAll tests completed successfully!")

    finally:
        sock.close()

if __name__ == "__main__":
    main()
