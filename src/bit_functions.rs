pub fn test(value: u8, bit: u8) -> bool {
    return val(value, bit) == 1;
}

pub fn val(value: u8, bit: u8) -> u8 {
    return (value >> bit) & 1;
}

pub fn set(value: u8, bit: u8) -> u8 {
    return value | (1.wrapping_shl(bit))
}

pub fn reset(value: u8, bit: u8) -> u8 {
    return value & !(1.wrapping_shl(bit))
}

pub fn half_carry_add(value_1: u8, value_2: u8) -> bool {
    return (value_1&0xF).wrapping_add(value_2&0xF) > 0xF
}

pub fn b(value: bool) -> u8 {
    if value {
        return 1
    }
    return 0
}
