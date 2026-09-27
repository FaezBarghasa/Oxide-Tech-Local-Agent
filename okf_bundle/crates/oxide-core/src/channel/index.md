# channel

## Classs

- [TokenReceiver](TokenReceiver.md) — Stream receiver wrapping tokio mpsc for SSE and HTTP response pipelines.
- [TokenSender](TokenSender.md) — Zero-copy token chunk transmitter for the inference pipeline.

## Functions

- [create_token_channel](create_token_channel.md) — Create a bounded token stream channel with pre-allocated buffer slots.
- [new](new.md)
- [new](new_1.md)
- [new](new_2.md)
- [new](new_3.md)
- [poll_next](poll_next.md)
- [poll_next](poll_next_1.md)
- [recv](recv.md) — Asynchronously receive the next token chunk.
- [recv](recv_1.md) — Asynchronously receive the next token chunk.
- [send_chunk](send_chunk.md) — Send a token chunk as `Bytes` without copying.
- [send_chunk](send_chunk_1.md) — Send a token chunk as `Bytes` without copying.
- [send_str](send_str.md) — Send a UTF-8 token string converted directly to static/borrowed bytes.
- [send_str](send_str_1.md) — Send a UTF-8 token string converted directly to static/borrowed bytes.
- [test_token_channel_streaming](test_token_channel_streaming.md) — [tokio::test]
