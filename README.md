
# litch
Parser for ITCH 5.0, inspired by [`itchy`](https://github.com/adwhit/itchy-rust/)
but oriented toward live message handling.

### WARNING: Work-in-progress
This crate should be presumed experimental and non-functional until integration testing has been completed.
If you are willing and able to assist with integration testing, please leave a response under 
[this issue](https://github.com/j-stach/litch/issues/1).

## Use
1. Add `litch` to your Rust project (`v2024` or more recent):
```bash
cargo add litch
```
2. Create a UDP socket and connect to the TotalView-ITCH broadcast port. <br>
When you receive data, parse it as an `ItchMessage` enum.
```rust
use std::net::UdpSocket;
use litch::ItchMessage;

let socket = UdpSocket::bind(127.0.0.1:0).unwrap();
let mut buf = [0u8; 1024];

// TODO: Connect to TotalView-ITCH feed

let (len, _origin) = socket.recv_from(&mut buf).unwrap();
let msg = ItchMessage::parse(buf[..len]).unwrap();
```
3. Use `match` to extract message contents. 
All messages have `metadata` and a `body` which contains variant-specific data.
```
use litch::msg::SystemEvent::*;
use ItchMessage::*;

match msg {
    SystemEvent { metadata, body } => {
        match body {
            BeginMessages => {/* Do something */},
            _ => {/* Do something else */}
        }
    },
    _ => {/* Do nothing */}
}
```
4. The message metadata (`ItchMetadata`) can be accessed without matching. 
```
let meta = msg.metadata();
let _time = meta.timestamp;
let _locate = meta.stock_locate;
let _num = meta.tracking_number;
```


## Development
Development history and current tasks are tracked in [TODO.md](TODO.md).

Developer resources:
- [ITCH 5.0 Specification](https://www.nasdaqtrader.com/content/technicalsupport/specifications/dataproducts/NQTVITCHspecification.pdf)

Contributions are welcome! Submit issues and pull requests to this repository.

