package examples.java;

import uniffi.smpp_codec_ffi.*;
import java.net.Socket;
import java.io.InputStream;
import java.io.OutputStream;
import java.nio.ByteBuffer;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;
import java.util.concurrent.TimeUnit;
import java.util.List;
import java.util.ArrayList;

public class ToolClient {
    private static final String HOST = "127.0.0.1";
    private static final int PORT = 2775;
    private static final int HEADER_LEN = 16;

    // Command IDs
    private static final int CMD_BIND_TRANSCEIVER_RESP = 0x80000009;
    private static final int CMD_SUBMIT_SM_RESP = 0x80000004;
    private static final int CMD_UNBIND_RESP = 0x80000006;

    public static void main(String[] args) {
        System.out.println("Connecting to " + HOST + ":" + PORT + "...");
        
        try (Socket socket = new Socket(HOST, PORT)) {
            InputStream input = socket.getInputStream();
            OutputStream output = socket.getOutputStream();
            System.out.println("Connected!");

            // 1. Bind Transceiver
            System.out.println("\n--- 1. Bind Transceiver ---");
            BindRequest bindReq = new BindRequest(
                1, "my_system_id", "password", "", (short)0x34,
                Ton.UNKNOWN, Npi.UNKNOWN, "", BindMode.TRANSCEIVER
            );
            sendPdu(output, SmppCodecFfiKt.encodeBindRequest(bindReq));
            readResponse(input, CMD_BIND_TRANSCEIVER_RESP);

            // 2. Multi-threaded SubmitSm
            System.out.println("\n--- 2. Multi-threaded SubmitSm ---");
            ExecutorService executor = Executors.newFixedThreadPool(5);
            
            for (int i = 0; i < 10; i++) {
                final int seq = 100 + i;
                executor.submit(() -> {
                    try {
                        SubmitSmRequest submitReq = new SubmitSmRequest(
                            seq, "CMT", Ton.INTERNATIONAL, Npi.ISDN, "123456",
                            Ton.NATIONAL, Npi.ISDN, "9876543210", (short)0, (short)0, (short)1,
                            null, null, (short)1, (short)0, (short)0, (short)0,
                            convertBytes(seq + ": Hello from Java Thread"),
                            new ArrayList<>()
                        );
                        
                        synchronized (output) {
                            sendPdu(output, SmppCodecFfiKt.encodeSubmitSmRequest(submitReq));
                        }
                    } catch (Exception e) {
                        e.printStackTrace();
                    }
                });
            }

            executor.shutdown();
            executor.awaitTermination(10, TimeUnit.SECONDS);

            // Wait for responses
            for (int i = 0; i < 10; i++) {
                readResponse(input, CMD_SUBMIT_SM_RESP);
            }

            // 3. Unbind
            System.out.println("\n--- 3. Unbind ---");
            UnbindRequest unbindReq = new UnbindRequest(999);
            sendPdu(output, SmppCodecFfiKt.encodeUnbindRequest(unbindReq));
            readResponse(input, CMD_UNBIND_RESP);

            System.out.println("\nDone!");

        } catch (Exception e) {
            e.printStackTrace();
        }
    }

    private static List<UByte> convertBytes(String s) {
        byte[] bytes = s.getBytes();
        List<UByte> list = new ArrayList<>();
        for (byte b : bytes) {
            list.add(new UByte((short)(b & 0xFF)));
        }
        return list;
    }

    private static void sendPdu(OutputStream output, List<UByte> data) throws Exception {
        byte[] bytes = new byte[data.size()];
        for (int i = 0; i < data.size(); i++) {
            bytes[i] = (byte)data.get(i).getValue();
        }
        output.write(bytes);
        System.out.println("Sent PDU: " + bytes.length + " bytes");
    }

    private static void readResponse(InputStream input, int expectedId) throws Exception {
        byte[] header = new byte[HEADER_LEN];
        int read = input.read(header);
        if (read < HEADER_LEN) throw new Exception("Failed to read header");

        ByteBuffer buffer = ByteBuffer.wrap(header);
        int commandLen = buffer.getInt();
        int commandId = buffer.getInt();
        int commandStatus = buffer.getInt();
        int sequenceNumber = buffer.getInt();

        System.out.printf("Response: Len=%d, ID=0x%08X, Status=%d, Seq=%d%n", 
            commandLen, commandId, commandStatus, sequenceNumber);

        if (commandId != expectedId) {
            System.out.printf("WARNING: Expected command ID 0x%08X, got 0x%08X%n", expectedId, commandId);
        }

        int bodyLen = commandLen - HEADER_LEN;
        if (bodyLen > 0) {
            byte[] body = new byte[bodyLen];
            input.read(body);
        }
    }
}
