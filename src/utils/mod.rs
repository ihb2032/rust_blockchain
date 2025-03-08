/// A module for handling conversions between byte arrays and hexadecimal strings.
///
/// This module provides two primary functions:
///
/// - `bytes_to_hex_string`: Converts a byte slice into a hexadecimal string representation.
/// - `hex_string_to_bytes`: Converts a hexadecimal string back into a byte vector.
///
/// # Examples
///
/// ```rust
/// let bytes = [0x12, 0x34, 0x56];
/// let hex_string = hash::bytes_to_hex_string(&bytes);
/// assert_eq!(hex_string, "123456");
///
/// let hex = "123456";
/// let bytes = hash::hex_string_to_bytes(hex);
/// assert_eq!(bytes, vec![0x12, 0x34, 0x56]);
/// ```
/// Additional tests
/// ```rust
/// // Test with an empty byte array
/// let bytes: &[u8] = &[];
/// let hex_string = hash::bytes_to_hex_string(bytes);
/// assert_eq!(hex_string, "");
///
/// // Test with an empty hex string
/// let hex = "";
/// let bytes = hash::hex_string_to_bytes(hex);
/// assert_eq!(bytes, vec![]);
///
/// // Test with a single byte
/// let bytes = [0xAB];
/// let hex_string = hash::bytes_to_hex_string(&bytes);
/// assert_eq!(hex_string, "ab");
///
/// let hex = "ab";
/// let bytes = hash::hex_string_to_bytes(hex);
/// assert_eq!(bytes, vec![0xAB]);
///
/// // Test with invalid hex string
/// let hex = "xyz";
/// let result = std::panic::catch_unwind(|| {
///     hash::hex_string_to_bytes(hex);
/// });
/// assert!(result.is_err());
/// ```
pub mod hash {
    pub fn bytes_to_hex_string(bytes: &[u8]) -> String {
            bytes.iter().fold(String::new(), |mut acc, b| {
        use std::fmt::Write;
        write!(&mut acc, "{:02x}", b).unwrap();
        acc
    })
    }

    pub fn hex_string_to_bytes(hex: &str) -> Vec<u8> {
        (0..hex.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).expect("Invalid hex string"))
            .collect()
    }
}
