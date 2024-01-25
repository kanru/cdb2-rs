use std::u32;

pub(crate) fn unpack(data: &[u8]) -> u32 {
    u32::from_le_bytes(data.try_into().unwrap())
}

pub(crate) fn unpack2(buf: &[u8]) -> (u32, u32) {
    assert!(buf.len() >= 8);
    (unpack(&buf[0..4]), unpack(&buf[4..8]))
}

fn pack(data: &mut [u8], src: u32) {
    data[..4].copy_from_slice(&src.to_le_bytes());
}

pub(crate) fn pack2(data: &mut [u8], src0: u32, src1: u32) {
    assert!(data.len() >= 8);
    pack(&mut data[0..4], src0);
    pack(&mut data[4..8], src1);
}
