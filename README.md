# gc-adapter
A Rust library for working with the Nintendo Gamecube Controller Adapter

## Example

Cargo.toml:
```
gc-adapter = { version = "0.1.2", features = ["libusb"] }
```

Code:
```rust
use gc_adapter::GcAdapter;

// get adapter from global context
let mut adapter = GcAdapter::from_usb().unwrap();

// refresh inputs to ensure they are up to date
adapter.refresh_inputs();

// read and display all controller ports
dbg!(adapter.read_controllers());

// enable rumble for only ports 1 and 4
adapter.set_rumble([false, false, false, true]);

std::thread::sleep(std::time::Duration::from_millis(100));

// on drop all rumble will be disabled and the USB connection
// will be cleaned up
let _ = adapter;
```
