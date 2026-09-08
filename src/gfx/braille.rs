pub const DOTS: [(i32, i32); 8] = [
    (0, 0), (1, 0), (2, 0), (0, 1),
    (1, 1), (2, 1), (3, 0), (3, 1),
];

pub fn enc(bits: u8) -> char {
    std::char::from_u32(0x2800 + bits as u32).unwrap_or(' ')
}
