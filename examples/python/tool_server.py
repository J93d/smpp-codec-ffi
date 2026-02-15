import socket
import struct
import smpp_codec_ffi

HOST = "127.0.0.1"
PORT = 2775
HEADER_LEN = 16

# Command IDs (Requests)
CMD_BIND_RECEIVER = 0x00000001
CMD_BIND_TRANSMITTER = 0x00000002
CMD_QUERY_SM = 0x00000003
CMD_SUBMIT_SM = 0x00000004
CMD_DELIVER_SM = 0x00000005
CMD_UNBIND = 0x00000006
CMD_REPLACE_SM = 0x00000007
CMD_CANCEL_SM = 0x00000008
CMD_BIND_TRANSCEIVER = 0x00000009
CMD_ENQUIRE_LINK = 0x00000015
CMD_SUBMIT_MULTI_SM = 0x00000021
CMD_ALERT_NOTIFICATION = 0x00000102
CMD_DATA_SM = 0x00000103
CMD_BROADCAST_SM = 0x00000111
CMD_QUERY_BROADCAST_SM = 0x00000112
CMD_CANCEL_BROADCAST_SM = 0x00000113

def handle_client(conn, addr):
    print(f"Connected by {addr}")
    try:
        while True:
            header = conn.recv(HEADER_LEN)
            if not header or len(header) < HEADER_LEN:
                break
            
            command_len = struct.unpack(">I", header[0:4])[0]
            command_id = struct.unpack(">I", header[4:8])[0]
            command_status = struct.unpack(">I", header[8:12])[0]
            sequence_number = struct.unpack(">I", header[12:16])[0]
            
            print(f"Header: Len={command_len}, ID=0x{command_id:08X}, Status={command_status}, Seq={sequence_number}")
            
            body_len = command_len - HEADER_LEN
            body = b""
            if body_len > 0:
                body = conn.recv(body_len)
            
            full_pdu = header + body
            
            response_pdu = None
            
            if command_id in [CMD_BIND_RECEIVER, CMD_BIND_TRANSMITTER, CMD_BIND_TRANSCEIVER]:
                req = smpp_codec_ffi.decode_bind_request(full_pdu)
                print(f"Received Bind: {req.system_id}")
                resp = smpp_codec_ffi.BindResponse(
                     sequence_number=sequence_number,
                     command_id=command_id | 0x80000000,
                     system_id=req.system_id,
                     sc_interface_version_tlv=None
                )
                response_pdu = smpp_codec_ffi.encode_bind_response(resp)
                
            elif command_id == CMD_SUBMIT_SM:
                req = smpp_codec_ffi.decode_submit_sm_request(full_pdu)
                print(f"Received SubmitSm: {req.source_addr} -> {req.destination_addr}")
                
                # Concatenation Detection
                is_udh = (req.esm_class & 0x40) == 0x40
                sar_msg_ref = None
                sar_total = None
                sar_seq = None
                
                for tlv in req.tlvs:
                    if tlv.tag == smpp_codec_ffi.Tags.SAR_MSG_REF_NUM:
                        # Extract u16 from value (2 bytes, big endian)
                        sar_msg_ref = struct.unpack(">H", bytes(tlv.value))[0]
                    elif tlv.tag == smpp_codec_ffi.Tags.SAR_TOTAL_SEGMENTS:
                        sar_total = tlv.value[0]
                    elif tlv.tag == smpp_codec_ffi.Tags.SAR_SEGMENT_SEQNUM:
                        sar_seq = tlv.value[0]
                
                if is_udh:
                    print("  [DETECTION] UDH Concatenation detected")
                    # In a real server, we would parse the UDH header from req.short_message[0:6] or [0:7]
                    if len(req.short_message) > 6:
                         print(f"  [DEBUG] UDH Info (Raw): {req.short_message[:6].hex()}")
                
                if sar_msg_ref is not None:
                    print(f"  [DETECTION] SAR Concatenation detected: Ref={sar_msg_ref}, Part={sar_seq}/{sar_total}")

                resp = smpp_codec_ffi.SubmitSmResponse(
                    sequence_number=sequence_number,
                    message_id="MsgID_12345"
                )
                response_pdu = smpp_codec_ffi.encode_submit_sm_response(resp)

            elif command_id == CMD_DELIVER_SM:
                req = smpp_codec_ffi.decode_deliver_sm(full_pdu)
                print(f"Received DeliverSm: {req.source_addr}")
                resp = smpp_codec_ffi.DeliverSmResponse(
                    sequence_number=sequence_number,
                    command_status=0,
                    message_id="MsgID_DLR",
                    status_description=""
                )
                response_pdu = smpp_codec_ffi.encode_delivery_sm_response(resp)

            elif command_id == CMD_BROADCAST_SM:
                req = smpp_codec_ffi.decode_broadcast_sm_request(full_pdu)
                print(f"Received BroadcastSm")
                resp = smpp_codec_ffi.BroadcastSmResponse(
                    sequence_number=sequence_number,
                    message_id="BcastID_999",
                    optional_params=[]
                )
                response_pdu = smpp_codec_ffi.encode_broadcast_sm_response(resp)

            elif command_id == CMD_SUBMIT_MULTI_SM:
                req = smpp_codec_ffi.decode_submit_multi_request(full_pdu)
                print(f"Received SubmitMulti")
                resp = smpp_codec_ffi.SubmitMultiResponse(
                    sequence_number=sequence_number,
                    message_id="MsgID_Multi",
                    unsuccess_smes=[]
                )
                response_pdu = smpp_codec_ffi.encode_submit_multi_response(resp)

            elif command_id == CMD_QUERY_SM:
                req = smpp_codec_ffi.decode_query_sm_request(full_pdu)
                print(f"Received QuerySm")
                resp = smpp_codec_ffi.QuerySmResponse(
                    sequence_number=sequence_number,
                    message_id=req.message_id,
                    final_date="220101000000000R",
                    message_state=2, # Delivered
                    error_code=0
                )
                response_pdu = smpp_codec_ffi.encode_query_sm_response(resp)

            elif command_id == CMD_CANCEL_SM:
                req = smpp_codec_ffi.decode_cancel_sm_request(full_pdu)
                print(f"Received CancelSm")
                resp = smpp_codec_ffi.CancelSmResponse(
                    sequence_number=sequence_number
                )
                response_pdu = smpp_codec_ffi.encode_cancel_sm_response(resp)

            elif command_id == CMD_REPLACE_SM:
                req = smpp_codec_ffi.decode_replace_sm_request(full_pdu)
                print(f"Received ReplaceSm")
                resp = smpp_codec_ffi.ReplaceSmResponse(
                    sequence_number=sequence_number
                )
                response_pdu = smpp_codec_ffi.encode_replace_sm_response(resp)

            elif command_id == CMD_DATA_SM:
                req = smpp_codec_ffi.decode_data_sm(full_pdu)
                print(f"Received DataSm")
                resp = smpp_codec_ffi.DataSmResponse(
                    sequence_number=sequence_number,
                    command_status=0,
                    message_id="MsgID_Data",
                    status_description="",
                    optional_params=[]
                )
                response_pdu = smpp_codec_ffi.encode_data_sm_response(resp)

            elif command_id == CMD_ALERT_NOTIFICATION:
                req = smpp_codec_ffi.decode_alert_notification(full_pdu)
                print(f"Received AlertNotification")
                # No response

            elif command_id == CMD_QUERY_BROADCAST_SM:
                req = smpp_codec_ffi.decode_query_broadcast_sm_request(full_pdu)
                print(f"Received QueryBroadcastSm")
                resp = smpp_codec_ffi.QueryBroadcastSmResponse(
                    sequence_number=sequence_number,
                    message_id=req.message_id,
                    optional_params=[]
                )
                response_pdu = smpp_codec_ffi.encode_query_broadcast_sm_response(resp)

            elif command_id == CMD_CANCEL_BROADCAST_SM:
                req = smpp_codec_ffi.decode_cancel_broadcast_sm_request(full_pdu)
                print(f"Received CancelBroadcastSm")
                resp = smpp_codec_ffi.CancelBroadcastSmResponse(
                    sequence_number=sequence_number
                )
                response_pdu = smpp_codec_ffi.encode_cancel_broadcast_sm_response(resp)

            elif command_id == CMD_UNBIND:
                req = smpp_codec_ffi.decode_unbind_request(full_pdu)
                print(f"Received Unbind")
                resp = smpp_codec_ffi.UnbindResponse(
                    sequence_number=sequence_number
                )
                response_pdu = smpp_codec_ffi.encode_unbind_response(resp)
                if response_pdu:
                    conn.sendall(bytes(response_pdu))
                    print("Sent Unbind Response")
                break

            elif command_id == CMD_ENQUIRE_LINK:
                # Assuming EnquireLink isn't explicitly in FFI top-level decode yet, but it might be
                # If not available, we can just echo success
                print("Received EnquireLink")
                # Need to check if EnquireLink is exposed. If not, manual construct? 
                # Let's assume it is or just skip for now as not in main list above
                # But to be safe, standard response:
                resp_header = struct.pack(">IIII", 16, 0x80000015, 0, sequence_number)
                conn.sendall(resp_header)
                continue

            else:
                print(f"Unknown Command ID: 0x{command_id:08X}")

            if response_pdu:
                conn.sendall(bytes(response_pdu))
                print("Sent Response")

    finally:
        conn.close()

def main():
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
        s.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
        s.bind((HOST, PORT))
        s.listen()
        print(f"Server listening on {HOST}:{PORT}")
        while True:
            conn, addr = s.accept()
            handle_client(conn, addr)

if __name__ == "__main__":
    main()
