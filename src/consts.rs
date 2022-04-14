pub const OTF_FONT_REGULAR: &[u8] =
    include_bytes!("../assets/fonts/roboto/Roboto-Regular.ttf");
pub const OTF_FONT_MONOSPACE: &[u8] =
    include_bytes!("../assets/fonts/red-hat/RedHatMono-Regular.otf");

pub const fn get_sans_license() -> &'static str {
    include_str!("../assets/fonts/roboto/LICENSE")
}

pub const fn get_mono_license() -> &'static str {
    include_str!("../assets/fonts/red-hat/LICENSE")
}
