package examples.java;

import uniffi.smpp_codec_ffi.*;
import java.net.ServerSocket;
import java.net.Socket;
import java.io.InputStream;
import java.io.OutputStream;
import java.nio.ByteBuffer;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;
import java.util.List;
import java.util.ArrayList;

public class ToolServer {
    private static final int PORT = 2775;
    private static final int HEADER_LEN = 16;

    // Command IDs
    private static final int CMD_BIND_RECEIVER = 0x00000001;
    private static final int CMD_BIND_TRANSMITTER = 0x00000002;
    private static final int CMD_SUBMIT_SM = 0x00000004;
    private static final int CMD_UNBIND = 0x00000006;
    private static final int CMD_BIND_TRANSCEIVER = 0x00000009;

    public static void main(String[] args) {
        ExecutorService threadPool = Executors.newCachedThreadPool();
        
        try (ServerSocket server = new ServerSocket(PORT)) {
            System.out.println("Java Multi-threaded Server listening on port " + PORT);

            while (true) {
                Socket client = server.accept();
                System.out.println("Client connected: " + client.getInetAddress());
                threadPool.submit(() -> handleClient(client));
            }
        } catch (Exception e) {
            e.printStackTrace();
        }
    }

    private static void handleClient(Socket client) {
        try (InputStream input = client.getInputStream();
             OutputStream output = client.getOutputStream()) {

            while (true) {
                byte[] header = new byte[HEADER_LEN];
                int read = input.read(header);
                if (read < HEADER_LEN) break;

                ByteBuffer buffer = ByteBuffer.wrap(header);
                int commandLen = buffer.getInt();
                int commandId = buffer.getInt();
                int commandStatus = buffer.getInt();
                int sequenceNumber = buffer.getInt();

                System.out.printf("Header: Len=%d, ID=0x%08X, Seq=%d%n", commandLen, commandId, sequenceNumber);

                int bodyLen = commandLen - HEADER_LEN;
                byte[] body = new byte[bodyLen];
                if (bodyLen > 0) {
                    input.read(body);
                }

                byte[] fullPdu = new byte[commandLen];
                System.arraycopy(header, 0, fullPdu, 0, HEADER_LEN);
                System.arraycopy(body, 0, fullPdu, HEADER_LEN, bodyLen);

                List<UByte> fullPduList = convertToUByteList(fullPdu);
                List<UByte> responsePdu = null;

                if (commandId == CMD_BIND_RECEIVER || commandId == CMD_BIND_TRANSMITTER || commandId == CMD_BIND_TRANSCEIVER) {
                    BindRequest req = SmppCodecFfiKt.decodeBindRequest(fullPduList);
                    System.out.println("Received Bind: " + req.getSystemId());
                    BindResponse resp = new BindResponse(
                        sequenceNumber, commandId | 0x80000000, req.getSystemId(), null
                    );
                    responsePdu = SmppCodecFfiKt.encodeBindResponse(resp);
                } else if (commandId == CMD_SUBMIT_SM) {
                    SubmitSmRequest req = SmppCodecFfiKt.decodeSubmitSmRequest(fullPduList);
                    System.out.println("Received SubmitSm");
                    
                    // Concatenation Detection (Example)
                    boolean isUdh = (req.getEsmClass() & 0x40) == 0x40;
                    if (isUdh) System.out.println("  [DETECTION] UDH Detected");
                    
                    SubmitSmResponse resp = new SubmitSmResponse(sequenceNumber, "MsgID_12345");
                    responsePdu = SmppCodecFfiKt.encodeSubmitSmResponse(resp);
                } else if (commandId == CMD_UNBIND) {
                    System.out.println("Received Unbind");
                    UnbindResponse resp = new UnbindResponse(sequenceNumber);
                    responsePdu = SmppCodecFfiKt.encodeUnbindResponse(resp);
                }

                if (responsePdu != null) {
                    byte[] respBytes = new byte[responsePdu.size()];
                    for (int i = 0; i < responsePdu.size(); i++) {
                        respBytes[i] = (byte)responsePdu.get(i).getValue();
                    }
                    output.write(respBytes);
                    System.out.println("Sent Response");
                    if (commandId == CMD_UNBIND) break;
                }
            }
        } catch (Exception e) {
            System.err.println("Error handling client: " + e.getMessage());
        } finally {
            try { client.close(); } catch (Exception ignored) {}
        }
    }

    private static List<UByte> convertToUByteList(byte[] bytes) {
        List<UByte> list = new ArrayList<>();
        for (byte b : bytes) {
            list.add(new UByte((short)(b & 0xFF)));
        }
        return list;
    }
}
