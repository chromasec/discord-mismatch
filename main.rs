fn uniggerify(corrupted: &str) -> String {
    let utf16_units: Vec<u16> = corrupted.encode_utf16().collect();

    let mut digits = String::new();

    for unit in utf16_units {
        let high = (unit >> 8) as u8;
        let low = (unit & 0xFF) as u8;
        if (0x30..=0x39).contains(&high) {
            digits.push(high as char);
        }
        else if (0x30..=0x39).contains(&low) {
            digits.push(low as char);
        }
        else {
            digits.push('?');
        }
    }

    digits
}

fn main() {
    let corrupted = "嘀漀琀爀攀 挀漀搀攀 搀攀 猀挀甀爀椀琀 䐀椀猀挀漀爀搀 攀猀琀ꀀ㨀 㘀㜀㘀㤀㤀　"; // so yea replace by ur shite
    let decoded = uniggerify(corrupted);
    println!("Decoded digits: {}", decoded);
}
