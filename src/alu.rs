pub fn add_u8(x: u8, y: u8, carry: bool) -> (u8, bool, bool, bool) {
    let (res, h, c, z) = add(8, x as usize, y as usize, carry, 4, 8);
    (res as u8, h, c, z)
}

pub fn add_u16(x: u16, y: u16, carry: bool) -> (u16, bool, bool, bool) {
    let (res, h, c, z) = add(16, x as usize, y as usize, carry, 12, 16);
    (res as u16, h, c, z)
}

pub fn add_u16_signed(x: u16, y: u8, carry: bool) -> (u16, bool, bool, bool) {
    let (res, h, c, z) = add(16, x as usize, signed(y) as usize, carry, 4, 8);
    (res as u16, h, c, z)
}

pub fn sub_u8(x: u8, y: u8, carry: bool) -> (u8, bool, bool, bool) {
    let (res, h, c, z) = sub(8, x as usize, y as usize, carry, 4, 8);
    (res as u8, h, c, z)
}

fn add(
    bit: usize,
    x: usize,
    y: usize,
    carry: bool,
    half_carry_bit: usize,
    carry_bit: usize,
) -> (usize, bool, bool, bool) {
    let c = carry as usize;
    let mask = (1 << bit) - 1;
    let res = (x + y + c) & mask;
    let h = has_carry(half_carry_bit, x, y, c);
    let c = has_carry(carry_bit, x, y, c);
    let z = res == 0;
    (res, h, c, z)
}

fn sub(
    bit: usize,
    x: usize,
    y: usize,
    carry: bool,
    half_carry_bit: usize,
    carry_bit: usize,
) -> (usize, bool, bool, bool) {
    let c = carry as usize;
    let mask = (1 << bit) - 1;
    let res = (x.wrapping_sub(y).wrapping_sub(c)) & mask;
    let h = has_borrow(half_carry_bit, x, y, c);
    let c = has_borrow(carry_bit, x, y, c);
    let z = res == 0;
    (res, h, c, z)
}

fn has_carry(bit: usize, x: usize, y: usize, carry: usize) -> bool {
    let c = carry as usize;
    let mask = (1 << bit) - 1;
    (x & mask) + (y & mask) + (c & mask) > mask
}

fn has_borrow(bit: usize, x: usize, y: usize, carry: usize) -> bool {
    let mask = (1 << bit) - 1;
    (x & mask) < (y & mask) + (carry & mask)
}

pub fn signed(v: u8) -> u16 {
    if v & 0x80 != 0 {
        0xff00 | v as u16
    } else {
        v as u16
    }
}

#[test]
fn test_add_u8() {
    assert_eq!(add_u8(0x12, 0x22, false), (0x34, false, false, false));
    assert_eq!(add_u8(0x12, 0x22, true), (0x35, false, false, false));
    assert_eq!(add_u8(0x12, 0x2f, false), (0x41, true, false, false));
    assert_eq!(add_u8(0x12, 0x2f, true), (0x42, true, false, false));
    assert_eq!(add_u8(0x12, 0xf0, false), (0x02, false, true, false));
    assert_eq!(add_u8(0x12, 0xf0, true), (0x03, false, true, false));
    assert_eq!(add_u8(0x0a, 0xfa, false), (0x04, true, true, false));
    assert_eq!(add_u8(0x0a, 0xfa, true), (0x05, true, true, false));
    assert_eq!(add_u8(0x00, 0x00, false), (0x00, false, false, true));
    assert_eq!(add_u8(0x20, 0xe0, false), (0x00, false, true, true));
    assert_eq!(add_u8(0x08, 0xf8, false), (0x00, true, true, true));
    assert_eq!(add_u8(0x07, 0xf8, true), (0x00, true, true, true));
}

#[test]
fn test_sub_u8() {
    assert_eq!(sub_u8(0x12, 0x10, false), (0x02, false, false, false));
    assert_eq!(sub_u8(0x34, 0x22, true), (0x11, false, false, false));
    assert_eq!(sub_u8(0x32, 0x2f, false), (0x03, true, false, false));
    assert_eq!(sub_u8(0x32, 0x2e, true), (0x03, true, false, false));
    assert_eq!(sub_u8(0x12, 0xf0, false), (0x22, false, true, false));
    assert_eq!(sub_u8(0x12, 0xe0, true), (0x31, false, true, false));
    assert_eq!(sub_u8(0x0a, 0xef, false), (0x1b, true, true, false));
    assert_eq!(sub_u8(0x20, 0x5a, true), (0xc5, true, true, false));
    assert_eq!(sub_u8(0x12, 0x12, false), (0x00, false, false, true));
    assert_eq!(sub_u8(0x88, 0x87, true), (0x00, false, false, true));
}

#[test]
fn test_add_u16() {
    assert_eq!(
        add_u16(0x1200, 0x1000, false),
        (0x2200, false, false, false)
    );
    assert_eq!(add_u16(0x1134, 0x1222, true), (0x2357, false, false, false));
    assert_eq!(add_u16(0xf231, 0x2a13, false), (0x1c44, false, true, false));
    assert_eq!(add_u16(0xf231, 0x2a13, true), (0x1c45, false, true, false));
    assert_eq!(add_u16(0xf631, 0x2a03, false), (0x2034, true, true, false));
    assert_eq!(add_u16(0xf631, 0x2a03, true), (0x2035, true, true, false));
}

#[test]
fn test_signed() {
    assert_eq!(signed(0x0a), 0x000a);
    assert_eq!(signed(0x8a), 0xff8a);
}
