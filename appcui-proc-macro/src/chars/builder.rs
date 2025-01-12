use crate::{
    parameter_parser::{self, color::Color, *},
    token_stream_to_string::TokenStreamToString,
};
use proc_macro::*;
use std::str::FromStr;

static mut CHAR_ATTR: FlagsSignature = FlagsSignature::new(&["Bold", "Italic", "Underline"]);

static CHAR_POSILITIONAL_PARAMETERS: &[PositionalParameter] = &[
    PositionalParameter::new("value", ParamType::String),
    PositionalParameter::new("fore", ParamType::Color),
    PositionalParameter::new("back", ParamType::Color),
];
static CHAR_NAMED_PARAMETERS: &[NamedParameter] = &[
    NamedParameter::new("value", "value", ParamType::String),
    NamedParameter::new("char", "value", ParamType::String),
    NamedParameter::new("ch", "value", ParamType::String),
    NamedParameter::new("fore", "fore", ParamType::Color),
    NamedParameter::new("foreground", "fore", ParamType::Color),
    NamedParameter::new("forecolor", "fore", ParamType::Color),
    NamedParameter::new("color", "fore", ParamType::Color),
    NamedParameter::new("back", "back", ParamType::Color),
    NamedParameter::new("background", "back", ParamType::Color),
    NamedParameter::new("backcolor", "back", ParamType::Color),
    NamedParameter::new("attr", "attr", ParamType::Flags),
    NamedParameter::new("attributes", "attr", ParamType::Flags),
    NamedParameter::new("code", "code", ParamType::String),
    NamedParameter::new("unicode", "code", ParamType::String),
];

static CHARATTR_POSILITIONAL_PARAMETERS: &[PositionalParameter] = &[
    PositionalParameter::new("fore", ParamType::Color),
    PositionalParameter::new("back", ParamType::Color),
];
static CHARATTR_NAMED_PARAMETERS: &[NamedParameter] = &[
    NamedParameter::new("fore", "fore", ParamType::Color),
    NamedParameter::new("foreground", "fore", ParamType::Color),
    NamedParameter::new("forecolor", "fore", ParamType::Color),
    NamedParameter::new("color", "fore", ParamType::Color),
    NamedParameter::new("back", "back", ParamType::Color),
    NamedParameter::new("background", "back", ParamType::Color),
    NamedParameter::new("backcolor", "back", ParamType::Color),
    NamedParameter::new("attr", "attr", ParamType::Flags),
    NamedParameter::new("attributes", "attr", ParamType::Flags),
];


fn get_color(param_name: &str, dict: &mut NamedParamsMap) -> Color {
    if !dict.contains(param_name) {
        return Color::Transparent;
    }
    if let Some(color) = dict.get_mut(param_name).unwrap().get_color() {
        return color;
    }
    panic!(
        "Invalid color value {} for parameter '{}'",
        dict.get(param_name).unwrap().get_string(),
        param_name
    );
}
fn unicode_number_to_value(text: &str) -> u32 {
    let mut value = 0;
    for ch in text.chars() {
        if ch.is_ascii_digit() {
            value = value * 16 + (ch as u32 - '0' as u32);
            continue;
        }
        if ('a'..='f').contains(&ch) {
            value = value * 16 + ((ch as u32 - 'a' as u32) + 10);
            continue;
        }
        if ('A'..='F').contains(&ch) {
            value = value * 16 + ((ch as u32 - 'A' as u32) + 10);
            continue;
        }
        panic!("Invalid hexadecimal number: {} for character code !",text);
    }
    value
}
fn add_color(output: &mut String, key: &str, dict: &mut NamedParamsMap) {
    let col = get_color(key, dict);
    output.push_str("Color::");
    output.push_str(col.get_name());
}
fn add_attr(output: &mut String, dict: &mut NamedParamsMap, param_list: &str) {
    if let Some(value) = dict.get_mut("attr") {
        if let Some(list) = value.get_list() {
            if list.is_empty() {
                output.push_str("CharFlags::None)");
            } else {
                let mut add_or_operator = false;
                for name in list {
                    if let Some(flag) = unsafe { CHAR_ATTR.get(name.get_string()) } {
                        if add_or_operator {
                            output.push_str(" | ")
                        }
                        output.push_str("CharFlags::");
                        output.push_str(flag);
                        add_or_operator = true;
                    } else {
                        Error::new(
                            param_list,
                            format!("Unknwon character attribute: {} !", name.get_string()).as_str(),
                            name.get_start_pos(),
                            name.get_end_pos(),
                        )
                        .panic();
                    }
                }
                output.push(')')
            }
        } else {
            panic!("Parameter 'attr' should contain some flags !");
        }
    } else {
        output.push_str("CharFlags::None)");
    }    
}
pub(crate) fn create_from_dict(param_list: &str, dict: &mut NamedParamsMap) -> String {
    dict.validate_positional_parameters(param_list, CHAR_POSILITIONAL_PARAMETERS).unwrap();
    dict.validate_named_parameters(param_list, CHAR_NAMED_PARAMETERS).unwrap();
    let mut res = String::with_capacity(64);
    res.push_str("Character::new(");
    if let Some(value) = dict.get("code") {
        let code_value = unicode_number_to_value(value.get_string());
        res.push_str(format!{"'\\u{{{:x}}}'",code_value}.as_str());
    } else {
        let val = dict
            .get("value")
            .expect("Missing first positional parameter or the parameter 'value' (the character code)");
        let char_value = val.get_string();
        let count = char_value.chars().count();
        match count {
            0 => res.push('0'),
            1 => {
                res.push('\'');
                res.push_str(char_value);
                res.push('\'')
            }
            _ => {
                let hash = crate::utils::compute_hash(char_value);
                if let Some(special_char) = super::SpecialCharacter::from_hash(hash) {
                    res.push_str("SpecialChar::");
                    res.push_str(special_char.get_name());
                } else {
                    panic!("Unknown representation '{}' for a special character !",char_value);
                }
            },
        }
    }
    res.push_str(", ");
    add_color(&mut res, "fore", dict);
    res.push_str(", ");
    add_color(&mut res, "back", dict);
    res.push_str(", ");
    add_attr(&mut res, dict, param_list);

    res
}

pub(crate) fn create_attr_from_dict(param_list: &str, dict: &mut NamedParamsMap) -> String {
    dict.validate_positional_parameters(param_list, CHARATTR_POSILITIONAL_PARAMETERS).unwrap();
    dict.validate_named_parameters(param_list, CHARATTR_NAMED_PARAMETERS).unwrap();
    let mut res = String::with_capacity(64);
    res.push_str("CharAttribute::new(");
    add_color(&mut res, "fore", dict);
    res.push_str(", ");
    add_color(&mut res, "back", dict);
    res.push_str(", ");
    add_attr(&mut res, dict, param_list);

    res
}

pub(crate) fn create(input: TokenStream) -> TokenStream {
    let s = input.validate_one_string_parameter("char");
    let mut d = parameter_parser::parse(&s).unwrap();
    let res = create_from_dict(&s, &mut d);
    TokenStream::from_str(&res).expect("Fail to convert 'char!' macro content to token stream")
}

pub(crate) fn create_attr(input: TokenStream) -> TokenStream {
    let s = input.validate_one_string_parameter("charattr");
    let mut d = parameter_parser::parse(&s).unwrap();
    let res = create_attr_from_dict(&s, &mut d);
    TokenStream::from_str(&res).expect("Fail to convert 'charattr!' macro content to token stream")
}