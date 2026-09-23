#![no_std]
pub const IMAGE_LEN: usize = 512;
pub const IMAGE: [u8; IMAGE_LEN] = [0; IMAGE_LEN];
#[cfg(test)]
extern crate std;
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn exact_zero_image() {
        assert_eq!(IMAGE.len(), 512);
        assert!(IMAGE.iter().all(|&b| b == 0));
    }
}
