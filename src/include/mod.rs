pub mod yaml;

pub fn to_ascii(bytes: &[u8]) -> String {
    return bytes
        .iter()
        .map(|&byte| char::from(byte).to_string())
        .collect();
}

pub const CONFIG_FILE: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/include/config.yaml"));
