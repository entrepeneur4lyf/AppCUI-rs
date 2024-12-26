use super::FormatNumber;
use super::FormatRatings;
use super::GlyphParser;
use super::KeyValueParser;
use super::Strategy;
use super::TempBuffer;
use super::TempString;
use super::ValueType;
use super::VectorIndex;
use crate::system::Handle;
use crate::system::HandleSupport;
use crate::utils::HandleManager;

#[test]
fn check_key_value_parser_single() {
    let mut p = KeyValueParser::new("abc=2");
    let k = p.next().unwrap();
    assert_eq!(k.key, "abc");
    assert_eq!(k.numerical_value, 2);
    assert_eq!(k.value_type, ValueType::Number);
    assert_eq!(p.next(), None);
}
#[test]
fn check_key_value_parser_double() {
    let mut p = KeyValueParser::new("abc=2,xyz=10%");
    let k = p.next().unwrap();
    assert_eq!(k.key, "abc");
    assert_eq!(k.numerical_value, 2);
    assert_eq!(k.value_type, ValueType::Number);
    let k = p.next().unwrap();
    assert_eq!(k.key, "xyz");
    assert_eq!(k.numerical_value, 1000);
    assert_eq!(k.value_type, ValueType::Percentage);
    assert_eq!(p.next(), None);
}
#[test]
fn check_key_value_parser_text() {
    let mut p = KeyValueParser::new("  abc  =  2 ,  xyz=10%   , some_value : another_value   ");
    let k = p.next().unwrap();
    assert_eq!(k.key, "abc");
    assert_eq!(k.numerical_value, 2);
    assert_eq!(k.value_type, ValueType::Number);
    let k = p.next().unwrap();
    assert_eq!(k.key, "xyz");
    assert_eq!(k.numerical_value, 1000);
    assert_eq!(k.value_type, ValueType::Percentage);
    let k = p.next().unwrap();
    assert_eq!(k.key, "some_value");
    assert_eq!(k.value_type, ValueType::String);
    assert_eq!(k.value, "another_value");
    assert_eq!(p.next(), None);
}

#[test]
fn check_index() {
    let mut i = VectorIndex::First;
    assert_eq!(i.index(), 0);
    i = VectorIndex::last(6);
    assert_eq!(i.index(), 5);
    i.set(10, 5, true);
    assert_eq!(i.index(), 4);
    i.set(10, 5, false);
    assert!(!i.is_valid());
    i = VectorIndex::with_value(3);
    assert_eq!(i.index(), 3);
    i.sub(1, 10, Strategy::Clamp);
    assert_eq!(i.index(), 2);
    i.sub(1, 10, Strategy::Clamp);
    assert_eq!(i.index(), 1);
    i.sub(1, 10, Strategy::Clamp);
    assert_eq!(i.index(), 0);
    i.sub(1, 10, Strategy::Clamp);
    assert_eq!(i.index(), 0);
    i.sub(1, 10, Strategy::Clamp);
    assert_eq!(i.index(), 0);
    i = VectorIndex::with_value(3);
    i.sub(125, 10, Strategy::Clamp);
    assert_eq!(i.index(), 0);
    i = VectorIndex::with_value(3);
    i.sub(4, 10, Strategy::Rotate);
    assert_eq!(i.index(), 9);
    i.sub(4, 10, Strategy::Rotate);
    assert_eq!(i.index(), 5);
    i.sub(4, 10, Strategy::Rotate);
    assert_eq!(i.index(), 1);
    i.sub(4, 10, Strategy::Rotate);
    assert_eq!(i.index(), 7);
    i.add(1, 9, Strategy::Clamp);
    assert_eq!(i.index(), 8);
    i.add(1, 9, Strategy::Clamp);
    assert_eq!(i.index(), 8);
    i.add(100, 9, Strategy::Clamp);
    assert_eq!(i.index(), 8);
    i.add(3, 9, Strategy::Rotate);
    assert_eq!(i.index(), 2);
    i.add(3, 9, Strategy::Rotate);
    assert_eq!(i.index(), 5);
    i.add(2, 9, Strategy::Rotate);
    assert_eq!(i.index(), 7);

    i = VectorIndex::with_value(5);
    assert!(i.in_range(10));
    assert!(i.in_range(6));
    assert!(!i.in_range(5));
}

#[test]
fn check_hanlde_manager() {
    struct MyData {
        text: String,
        value: i32,
        handle: Handle<MyData>,
    }
    impl MyData {
        fn new(text: &str, value: i32) -> Self {
            Self {
                text: text.to_string(),
                value,
                handle: Handle::None,
            }
        }
    }
    impl HandleSupport<MyData> for MyData {
        fn handle(&self) -> crate::system::Handle<MyData> {
            self.handle
        }

        fn set_handle(&mut self, handle: crate::system::Handle<MyData>) {
            self.handle = handle;
        }
    }

    let mut man: HandleManager<MyData> = HandleManager::with_capacity(16);
    let h1_123 = man.add(MyData::new("handle1", 123));
    let habc_1 = man.add(MyData::new("abc", 1));
    let hgdt_2345 = man.add(MyData::new("GDT", 2345));
    assert!(man.allocated_objects() == 3);
    assert!(man.free_spaces() == 0);
    let o1 = man.get(h1_123).unwrap();
    assert!(o1.handle == h1_123);
    assert!(o1.text == "handle1");
    assert!(o1.value == 123);
    let o2 = man.get(habc_1).unwrap();
    assert!(o2.handle == habc_1);
    assert!(o2.text == "abc");
    assert!(o2.value == 1);
    let o3 = man.get(hgdt_2345).unwrap();
    assert!(o3.handle == hgdt_2345);
    assert!(o3.text == "GDT");
    assert!(o3.value == 2345);
    // delete first element
    assert!(man.remove(h1_123));
    // second time it shoudl return false
    assert!(!man.remove(h1_123));
    assert!(man.get(h1_123).is_none());
    // check o3 again
    let o3 = man.get(hgdt_2345).unwrap();
    assert!(o3.handle == hgdt_2345);
    assert!(o3.text == "GDT");
    assert!(o3.value == 2345);
    // validate o2
    assert!(man.get(habc_1).is_some());
    // we have deleted one element it should be on the free list
    assert!(man.free_spaces() == 1);
    assert!(man.allocated_objects() == 3);
    // add a new element
    let h_new = man.add(MyData::new("new_handle", 1234));
    assert!(h_new != h1_123);
    assert!(h_new.index() == 0);
    assert!(h1_123.index() == 0);
    // no free spacess stored
    assert!(man.free_spaces() == 0);
    assert!(man.allocated_objects() == 3);
}

#[test]
fn check_glyph_char_and_size() {
    let g = String::from("123❤️╬▶-〓GDT");
    assert_eq!(g.chars().count(), 12);
    assert_eq!(g.count_glyphs(), 11);
    assert_eq!(g.len(), 22);

    assert_eq!(g.glyph(0), Some(('1', 1)));
    assert_eq!(g.glyph(2), Some(('3', 1)));
    assert_eq!(g.glyph(3), Some(('❤', 6)));
    assert_eq!(g.glyph(9), Some(('╬', 3)));
    assert_eq!(g.glyph(12), Some(('▶', 3)));
    assert_eq!(g.glyph(15), Some(('-', 1)));
    assert_eq!(g.glyph(16), Some(('〓', 3)));
    assert_eq!(g.glyph(19), Some(('G', 1)));
    assert_eq!(g.glyph(20), Some(('D', 1)));
    assert_eq!(g.glyph(21), Some(('T', 1)));
    assert_eq!(g.glyph(22), None);

    let poz = g.len();
    assert_eq!(g.previous_glyph(poz), Some(('T', 1)));
    assert_eq!(g.previous_glyph(poz - 1), Some(('D', 1)));
    assert_eq!(g.previous_glyph(poz - 2), Some(('G', 1)));
    assert_eq!(g.previous_glyph(poz - 3), Some(('〓', 3)));
    assert_eq!(g.previous_glyph(poz - 6), Some(('-', 1)));
    assert_eq!(g.previous_glyph(poz - 7), Some(('▶', 3)));
    assert_eq!(g.previous_glyph(poz - 10), Some(('╬', 3)));
    assert_eq!(g.previous_glyph(poz - 13), Some(('❤', 6)));
    assert_eq!(g.previous_glyph(poz - 19), Some(('3', 1)));
    assert_eq!(g.previous_glyph(poz - 20), Some(('2', 1)));
    assert_eq!(g.previous_glyph(poz - 21), Some(('1', 1)));
    assert_eq!(g.previous_glyph(poz - 22), None);
    assert_eq!(g.previous_glyph(0), None);
}

#[test]
fn check_glyph_next_pos() {
    let g = String::from("123❤️╬▶-〓GDT");
    assert_eq!(g.next_pos(0, 3), 3);
    assert_eq!(g.next_pos(2, 2), 9);
    assert_eq!(g.next_pos(9, 100), 22);
    assert_eq!(g.next_pos(9, 1), 12);
    assert_eq!(g.next_pos(9, 4), 19);
    assert_eq!(&g[9..g.next_pos(9, 4)], "╬▶-〓");
}

#[test]
fn check_glyph_previous_pos() {
    let g = String::from("123❤️╬▶-〓GDT");
    assert_eq!(g.previous_pos(22, 3), 19);
    assert_eq!(g.previous_pos(19, 3), 12);
    assert_eq!(&g[2..g.previous_pos(12, 1)], "3❤️");
    assert_eq!(g.previous_pos(19, 1000), 0);
}

#[test]
fn check_glyph_ignore_case() {
    let t = String::from("aBcDeFgH");
    assert_eq!(t.index_ignoring_case("abcdefgh"), Some(0));
    assert_eq!(t.index_ignoring_case("bcdefgh"), Some(1));
    assert_eq!(t.index_ignoring_case("cdefgh"), Some(2));
    assert_eq!(t.index_ignoring_case("defgh"), Some(3));
    assert_eq!(t.index_ignoring_case("efgh"), Some(4));
    assert_eq!(t.index_ignoring_case("fgh"), Some(5));
    assert_eq!(t.index_ignoring_case("gh"), Some(6));
    assert_eq!(t.index_ignoring_case("h"), Some(7));
    assert_eq!(t.index_ignoring_case("a"), Some(0));
    assert_eq!(t.index_ignoring_case("b"), Some(1));
    assert_eq!(t.index_ignoring_case("c"), Some(2));
    assert_eq!(t.index_ignoring_case("d"), Some(3));
    assert_eq!(t.index_ignoring_case("XXX"), None);
    assert_eq!(t.index_ignoring_case("aBcDeFgH"), Some(0));
    assert_eq!(t.index_ignoring_case("AbCdEfGh"), Some(0));
    assert_eq!(t.index_ignoring_case("AbCdEfE"), None);
}

#[test]
fn check_format_number_decimal_unsigned() {
    let mut output: [u8; 64] = [0; 64];
    const F1: FormatNumber = FormatNumber::new(10);
    assert_eq!(F1.write_number(123, &mut output), Some("123"));
    const F2: FormatNumber = FormatNumber::new(10).group(3, b',');
    let data: &[(u32, &'static str)] = &[
        (1234, "1,234"),
        (123456, "123,456"),
        (123, "123"),
        (12345, "12,345"),
        (0, "0"),
        (9, "9"),
        (10, "10"),
        (100, "100"),
        (1000, "1,000"),
        (1234567890, "1,234,567,890"),
    ];
    for (value, expect) in data.iter() {
        assert_eq!(F2.write_number(*value as u128, &mut output), Some(*expect));
    }
    const F3: FormatNumber = FormatNumber::new(10).fill(10, b'#');
    let data: &[(u32, &'static str)] = &[
        (1234, "######1234"),
        (123456, "####123456"),
        (0, "#########0"),
        (9, "#########9"),
        (10, "########10"),
        (1234567890, "1234567890"),
    ];
    for (value, expect) in data.iter() {
        assert_eq!(F3.write_number(*value as u128, &mut output), Some(*expect));
    }
    const F4: FormatNumber = FormatNumber::new(10).group(3, b',').fill(10, b'*');
    let data: &[(u32, &'static str)] = &[
        (1234, "*****1,234"),
        (123456, "***123,456"),
        (123, "*******123"),
        (12345, "****12,345"),
        (0, "*********0"),
        (9, "*********9"),
        (10, "********10"),
        (100, "*******100"),
        (1000, "*****1,000"),
        (1234567890, "1,234,567,890"),
    ];
    for (value, expect) in data.iter() {
        assert_eq!(F4.write_number(*value as u128, &mut output), Some(*expect));
    }
}

#[test]
fn check_format_number_decimal_signed() {
    let mut output: [u8; 64] = [0; 64];
    const F1: FormatNumber = FormatNumber::new(10);
    assert_eq!(F1.write_number(123i32, &mut output), Some("123"));
    assert_eq!(F1.write_number(-123i32, &mut output), Some("-123"));
    const F2: FormatNumber = FormatNumber::new(10).group(3, b',');
    let data: &[(i64, &'static str)] = &[
        (-1234, "-1,234"),
        (-123456, "-123,456"),
        (-123, "-123"),
        (-12345, "-12,345"),
        (0, "0"),
        (-9, "-9"),
        (-10, "-10"),
        (-100, "-100"),
        (-1000, "-1,000"),
        (-1234567890, "-1,234,567,890"),
    ];
    for (value, expect) in data.iter() {
        assert_eq!(F2.write_number(*value, &mut output), Some(*expect));
    }
    const F3: FormatNumber = FormatNumber::new(10).fill(10, b'#');
    let data: &[(i32, &'static str)] = &[
        (-1234, "#####-1234"),
        (-123456, "###-123456"),
        (0, "#########0"),
        (-9, "########-9"),
        (-10, "#######-10"),
        (-1234567890, "-1234567890"),
    ];
    for (value, expect) in data.iter() {
        assert_eq!(F3.write_number(*value, &mut output), Some(*expect));
    }
    const F4: FormatNumber = FormatNumber::new(10).group(3, b',').fill(10, b'*');
    let data: &[(i64, &'static str)] = &[
        (1234, "*****1,234"),
        (-123456, "**-123,456"),
        (123, "*******123"),
        (-12345, "***-12,345"),
        (0, "*********0"),
        (-9, "********-9"),
        (10, "********10"),
        (-100, "******-100"),
        (1000, "*****1,000"),
        (-1234567890, "-1,234,567,890"),
    ];
    for (value, expect) in data.iter() {
        assert_eq!(F4.write_number(*value, &mut output), Some(*expect));
    }
}

#[test]
fn check_format_number_hex_unsigned() {
    let mut output: [u8; 64] = [0; 64];
    const F1: FormatNumber = FormatNumber::new(16).prefix("0x");
    assert_eq!(F1.write_number(0x123, &mut output), Some("0x123"));
    const F2: FormatNumber = FormatNumber::new(16).group(4, b'_').prefix("0x");
    let data: &[(u64, &'static str)] = &[
        (0x1234, "0x1234"),
        (0x123456, "0x12_3456"),
        (0x123, "0x123"),
        (0, "0x0"),
        (9, "0x9"),
        (10, "0xA"),
        (0xFFFFFFFF, "0xFFFF_FFFF"),
    ];
    for (value, expect) in data.iter() {
        assert_eq!(F2.write_number(*value as u128, &mut output), Some(*expect));
    }
    const F3: FormatNumber = FormatNumber::new(16).fill(10, b'#').prefix("0x");
    let data: &[(u64, &'static str)] = &[
        (0x1234, "####0x1234"),
        (0x123456, "##0x123456"),
        (0, "#######0x0"),
        (9, "#######0x9"),
        (10, "#######0xA"),
        (0x1234567890, "0x1234567890"),
    ];
    for (value, expect) in data.iter() {
        assert_eq!(F3.write_number(*value as u128, &mut output), Some(*expect));
    }
}

#[test]
fn check_format_number_hex() {
    let mut output: [u8; 256] = [0; 256];
    const F1: FormatNumber = FormatNumber::new(16).representation_digits(8).prefix("0x");
    let data: &[(u64, &'static str)] = &[
        (0x1234, "0x00001234"),
        (0x123456, "0x00123456"),
        (0x123, "0x00000123"),
        (0, "0x00000000"),
        (9, "0x00000009"),
        (10, "0x0000000A"),
        (0xFFFFFFFF, "0xFFFFFFFF"),
    ];
    for (value, expect) in data.iter() {
        assert_eq!(F1.write_number(*value as u128, &mut output), Some(*expect));
    }
}

#[test]
fn check_format_number_bin() {
    let mut output: [u8; 256] = [0; 256];
    const F1: FormatNumber = FormatNumber::new(2).representation_digits(8).prefix("0b");
    let data: &[(u64, &'static str)] = &[
        (0b10, "0b00000010"),
        (0b1010, "0b00001010"),
        (0b101010, "0b00101010"),
        (0b10101010, "0b10101010"),
        (0b1010101010101010, "0b1010101010101010"),
        (0b10101010101010101010101010101010, "0b10101010101010101010101010101010"),
        (
            0b1010101010101010101010101010101010101010101010101010101010101010,
            "0b1010101010101010101010101010101010101010101010101010101010101010",
        ),
    ];
    for (value, expect) in data.iter() {
        assert_eq!(F1.write_number(*value as u128, &mut output), Some(*expect));
    }
}

#[test]
fn check_format_number_float() {
    let mut ouput: [u8; 64] = [0; 64];
    const F1: FormatNumber = FormatNumber::new(10).decimals(3);
    let data: &[(f64, &'static str)] = &[
        (1.0, "1.000"),
        (1.2301, "1.230"),
        (0.625, "0.625"),
        (0.6251, "0.625"),
        (0.6259, "0.625"),
        (0.6255, "0.625"),
        (0.6254, "0.625"),
        (0.6256, "0.625"),
        (0.625, "0.625"),
        (0.6250000000000001, "0.625"),
        (0.6250000000000009, "0.625"),
        (0.6250000000000005, "0.625"),
        (0.6250000000000004, "0.625"),
        (0.6250000000000006, "0.625"),
        (0.6250000000000007, "0.625"),
        (1.20001, "1.200"),
        (-1.2341, "-1.234"),
        (-0.625, "-0.625"),
        (-0.6251, "-0.625"),
        (-0.6259, "-0.625"),
        (-0.6255, "-0.625"),
        (-0.6254, "-0.625"),
        (-0.6256, "-0.625"),
        (-0.625, "-0.625"),
        (-0.6250000000000001, "-0.625"),
        (-0.6250000000000009, "-0.625"),
        (-0.6250000000000005, "-0.625"),
        (-0.6250000000000004, "-0.625"),
        (-0.6250000000000006, "-0.625"),
        (-0.6250000000000007, "-0.625"),
        (0.0, "0.000"),
        (0.0000000000000001, "0.000"),
        (0.0000000000000009, "0.000"),
        (0.0000000000000005, "0.000"),
        (0.0000000000000004, "0.000"),
        (0.0000000000000006, "0.000"),
        (0.0000000000000000, "0.000"),
    ];
    for (value, expect) in data.iter() {
        assert_eq!(F1.write_float(*value, &mut ouput), Some(*expect));
    }
}

#[test]
fn check_write_number_to_string() {
    let mut output: [u8; 64] = [0; 64];
    assert_eq!(FormatNumber::new(10).group(3, b',').write_number(123u64, &mut output), Some("123"));
    assert_eq!(FormatNumber::new(10).group(3, b',').write_number(1234u64, &mut output), Some("1,234"));
    assert_eq!(FormatNumber::new(10).group(4, b'-').write_number(1234u64, &mut output), Some("1234"));
    assert_eq!(
        FormatNumber::new(10).group(4, b'-').write_number(123456u128, &mut output),
        Some("12-3456")
    );
    assert_eq!(
        FormatNumber::new(10).group(4, b'-').write_number(12345678u64, &mut output),
        Some("1234-5678")
    );
    assert_eq!(
        FormatNumber::new(10).group(3, b':').write_number(12345678u64, &mut output),
        Some("12:345:678")
    );
    assert_eq!(
        FormatNumber::new(10)
            .group(3, b':')
            .representation_digits(8)
            .write_number(123u64, &mut output),
        Some("00:000:123")
    );
    assert_eq!(
        FormatNumber::new(10)
            .group(4, b'=')
            .representation_digits(8)
            .write_number(123456u128, &mut output),
        Some("0012=3456")
    );
    assert_eq!(
        FormatNumber::new(10)
            .group(4, b'=')
            .representation_digits(8)
            .prefix("PFX")
            .suffix("ABCD")
            .write_number(123456u64, &mut output),
        Some("PFX0012=3456ABCD")
    );
    assert_eq!(
        FormatNumber::new(10)
            .group(3, b'=')
            .representation_digits(8)
            .prefix("PFX")
            .suffix("ABCD")
            .fill(20, b'*')
            .write_number(123456u32, &mut output),
        Some("***PFX00=123=456ABCD")
    );
    assert_eq!(
        FormatNumber::new(16)
            .group(4, b' ')
            .representation_digits(8)
            .prefix("0x")
            .write_number(0x123456u32, &mut output),
        Some("0x0012 3456")
    );
    assert_eq!(
        FormatNumber::new(16)
            .representation_digits(8)
            .suffix("h")
            .write_number(0xC0FFEEu32, &mut output),
        Some("00C0FFEEh")
    );
}

#[test]
fn check_write_number_to_string_positive() {
    let mut output: [u8; 64] = [0; 64];
    assert_eq!(FormatNumber::new(10).group(3, b',').write_number(123, &mut output), Some("123"));
    assert_eq!(FormatNumber::new(10).group(3, b',').write_number(1234, &mut output), Some("1,234"));
    assert_eq!(FormatNumber::new(10).group(4, b'-').write_number(1234, &mut output), Some("1234"));
    assert_eq!(FormatNumber::new(10).group(4, b'-').write_number(123456, &mut output), Some("12-3456"));
    assert_eq!(
        FormatNumber::new(10).group(4, b'-').write_number(12345678, &mut output),
        Some("1234-5678")
    );
    assert_eq!(
        FormatNumber::new(10).group(3, b':').write_number(12345678, &mut output),
        Some("12:345:678")
    );
    assert_eq!(
        FormatNumber::new(10)
            .group(3, b':')
            .representation_digits(8)
            .write_number(123, &mut output),
        Some("00:000:123")
    );
    assert_eq!(
        FormatNumber::new(10)
            .group(4, b'=')
            .representation_digits(8)
            .write_number(123456, &mut output),
        Some("0012=3456")
    );
    assert_eq!(
        FormatNumber::new(10)
            .group(4, b'=')
            .representation_digits(8)
            .prefix("PFX")
            .suffix("ABCD")
            .write_number(123456, &mut output),
        Some("PFX0012=3456ABCD")
    );
    assert_eq!(
        FormatNumber::new(10)
            .group(3, b'=')
            .representation_digits(8)
            .prefix("PFX")
            .suffix("ABCD")
            .fill(20, b'*')
            .write_number(123456, &mut output),
        Some("***PFX00=123=456ABCD")
    );
    assert_eq!(
        FormatNumber::new(16)
            .group(4, b' ')
            .representation_digits(8)
            .prefix("0x")
            .write_number(0x123456, &mut output),
        Some("0x0012 3456")
    );
    assert_eq!(
        FormatNumber::new(16)
            .representation_digits(8)
            .suffix("h")
            .write_number(0xC0FFEE, &mut output),
        Some("00C0FFEEh")
    );
}

#[test]
fn check_write_number_to_string_negative() {
    let mut output: [u8; 64] = [0; 64];
    assert_eq!(FormatNumber::new(10).group(3, b',').write_number(-123, &mut output), Some("-123"));
    assert_eq!(FormatNumber::new(10).group(3, b',').write_number(-1234, &mut output), Some("-1,234"));
    assert_eq!(FormatNumber::new(10).group(4, b'-').write_number(-1234, &mut output), Some("-1234"));
    assert_eq!(FormatNumber::new(10).group(4, b'-').write_number(-123456, &mut output), Some("-12-3456"));
    assert_eq!(
        FormatNumber::new(10).group(4, b'-').write_number(-12345678, &mut output),
        Some("-1234-5678")
    );
    assert_eq!(
        FormatNumber::new(10).group(3, b':').write_number(-12345678, &mut output),
        Some("-12:345:678")
    );
    assert_eq!(
        FormatNumber::new(10)
            .group(3, b':')
            .representation_digits(8)
            .write_number(-123, &mut output),
        Some("-00:000:123")
    );
    assert_eq!(
        FormatNumber::new(10)
            .group(4, b'=')
            .representation_digits(8)
            .write_number(-123456, &mut output),
        Some("-0012=3456")
    );
    assert_eq!(
        FormatNumber::new(10)
            .group(4, b'=')
            .representation_digits(8)
            .prefix("PFX")
            .suffix("ABCD")
            .write_number(-123456, &mut output),
        Some("-PFX0012=3456ABCD")
    );
    assert_eq!(
        FormatNumber::new(10)
            .group(3, b'=')
            .representation_digits(8)
            .prefix("PFX")
            .suffix("ABCD")
            .fill(20, b'*')
            .write_number(-123456, &mut output),
        Some("**-PFX00=123=456ABCD")
    );
    assert_eq!(
        FormatNumber::new(16)
            .group(4, b' ')
            .representation_digits(8)
            .prefix("0x")
            .write_number(-0x123456, &mut output),
        Some("-0x0012 3456")
    );
    assert_eq!(
        FormatNumber::new(16)
            .representation_digits(8)
            .suffix("h")
            .write_number(-0xC0FFEE, &mut output),
        Some("-00C0FFEEh")
    );
}

#[test]
fn check_fraction() {
    let mut output: [u8; 64] = [0; 64];
    assert_eq!(FormatNumber::new(10).decimals(3).write_fraction(123, 1000, &mut output), Some("0.123"));
    assert_eq!(FormatNumber::new(10).decimals(3).write_fraction(12345, 1000, &mut output), Some("12.345"));
    assert_eq!(FormatNumber::new(10).decimals(1).write_fraction(1234, 1000, &mut output), Some("1.2"));
    assert_eq!(FormatNumber::new(10).decimals(3).write_fraction(-123, 1000, &mut output), Some("-0.123"));
    assert_eq!(
        FormatNumber::new(10).decimals(3).write_fraction(-12345, 1000, &mut output),
        Some("-12.345")
    );
    assert_eq!(FormatNumber::new(10).decimals(1).write_fraction(-1234, 1000, &mut output), Some("-1.2"));
    assert_eq!(FormatNumber::new(10).decimals(3).write_fraction(-123, -1000, &mut output), Some("0.123"));
    assert_eq!(
        FormatNumber::new(10).decimals(3).write_fraction(-12345, -1000, &mut output),
        Some("12.345")
    );
    assert_eq!(FormatNumber::new(10).decimals(1).write_fraction(-1234, -1000, &mut output), Some("1.2"));
    assert_eq!(
        FormatNumber::new(10)
            .decimals(2)
            .suffix(" KB")
            .write_fraction(1234u128, 1024u128, &mut output),
        Some("1.20 KB")
    );
    assert_eq!(
        FormatNumber::new(10)
            .decimals(2)
            .suffix(" KB")
            .write_fraction(1024u64, 1024u64, &mut output),
        Some("1.00 KB")
    );
    // no decimals
    assert_eq!(FormatNumber::new(10).write_fraction(123, 1000, &mut output), Some("0"));
    assert_eq!(FormatNumber::new(10).write_fraction(123, 100, &mut output), Some("1"));
    assert_eq!(FormatNumber::new(10).write_fraction(123, 10, &mut output), Some("12"));
}

#[test]
fn check_rating_report() {
    let mut buf_3: [u8; 3] = [0; 3];
    let mut buf_4: [u8; 4] = [0; 4];
    let mut buf_5: [u8; 5] = [0; 5];
    let mut buf: [u8; 20] = [0; 20];

    assert_eq!(FormatRatings::raport(0, 3, &mut buf_3), Some("0/3"));
    assert_eq!(FormatRatings::raport(2, 3, &mut buf_3), Some("2/3"));
    assert_eq!(FormatRatings::raport(2, 3, &mut buf_4), Some("2/3"));
    assert_eq!(FormatRatings::raport(2, 3, &mut buf_5), Some("2/3"));
    assert_eq!(FormatRatings::raport(22, 33, &mut buf_5), Some("22/33"));

    assert_eq!(FormatRatings::raport(1234, 12345, &mut buf), Some("1234/12345"));
    assert_eq!(FormatRatings::raport(0, 0, &mut buf_3), Some("0/0"));
    assert_eq!(FormatRatings::raport(10, 10, &mut buf_3), None);
    assert_eq!(FormatRatings::raport(5, 10, &mut buf_3), None);
    assert_eq!(FormatRatings::raport(5, 10, &mut buf_4), Some("5/10"));
}

#[test]
fn check_rating_two_chars() {
    let mut buf_3: [u8; 3] = [0; 3];
    let mut buf_4: [u8; 4] = [0; 4];
    let mut buf_5: [u8; 5] = [0; 5];
    let mut buf: [u8; 20] = [0; 20];

    assert_eq!(FormatRatings::two_chars(' ', '+', 0, 3, 3, &mut buf), Some("   "));
    assert_eq!(FormatRatings::two_chars(' ', '+', 0, 3, 3, &mut buf_3), Some("   "));
    assert_eq!(FormatRatings::two_chars('-', '+', 1, 3, 3, &mut buf_3), Some("+--"));
    assert_eq!(FormatRatings::two_chars('-', '+', 3, 3, 3, &mut buf_3), Some("+++"));
    assert_eq!(FormatRatings::two_chars('-', '+', 3, 4, 4, &mut buf_4), Some("+++-"));
    assert_eq!(FormatRatings::two_chars('-', '+', 20, 100, 5, &mut buf_5), Some("+----"));

    assert_eq!(FormatRatings::two_chars(' ', '+', 0, 3, 5, &mut buf_3), None);
}

#[test]
fn check_temp_string() {
    let t: TempString<10> = TempString::new("1234567890");
    assert_eq!(t.as_str(), "1234567890");
    assert!(!t.is_on_heap());
    let t: TempString<10> = TempString::new("1234567890A");
    assert_eq!(t.as_str(), "1234567890A");
    assert!(t.is_on_heap());
}

#[test]
fn check_temp_buffer() {
    let buf: TempBuffer<10> = TempBuffer::new(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    assert_eq!(buf.as_slice(), &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    assert!(!buf.is_on_heap());
    let buf: TempBuffer<10> = TempBuffer::new(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]);
    assert_eq!(buf.as_slice(), &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]);
    assert!(buf.is_on_heap());
}
