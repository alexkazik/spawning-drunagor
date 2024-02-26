#[derive(
    Clone, Copy, Default, PartialEq, enum_tools::EnumTools, serde::Deserialize, serde::Serialize,
)]
#[repr(usize)]
#[enum_tools(as_str, iter, names)]
pub(crate) enum MsgLanguage {
    #[default]
    #[enum_tools(rename = "English")]
    English,
}
