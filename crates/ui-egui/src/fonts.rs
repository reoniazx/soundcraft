//! Fonts: egui's bundled defaults, plus the operating system's own UI fonts loaded at runtime when
//! present (never redistributed). A "bold" family is always defined so the UI can use it.

use egui::{FontData, FontDefinitions, FontFamily};
use std::sync::Arc;

/// (path, ttc index) candidates for regular and bold UI text, by platform.
const REGULAR: &[(&str, u32)] = &[
    ("/System/Library/Fonts/HelveticaNeue.ttc", 0),
    ("/System/Library/Fonts/Helvetica.ttc", 0),
    ("C:\\Windows\\Fonts\\segoeui.ttf", 0),
    ("/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf", 0),
    ("/usr/share/fonts/TTF/DejaVuSans.ttf", 0),
    ("/usr/share/fonts/dejavu/DejaVuSans.ttf", 0),
    ("/usr/local/share/fonts/dejavu/DejaVuSans.ttf", 0),
    ("/usr/share/fonts/truetype/liberation/LiberationSans-Regular.ttf", 0),
];
const BOLD: &[(&str, u32)] = &[
    ("/System/Library/Fonts/HelveticaNeue.ttc", 1),
    ("/System/Library/Fonts/Helvetica.ttc", 1),
    ("C:\\Windows\\Fonts\\segoeuib.ttf", 0),
    ("/usr/share/fonts/truetype/dejavu/DejaVuSans-Bold.ttf", 0),
    ("/usr/share/fonts/TTF/DejaVuSans-Bold.ttf", 0),
    ("/usr/share/fonts/dejavu/DejaVuSans-Bold.ttf", 0),
    ("/usr/local/share/fonts/dejavu/DejaVuSans-Bold.ttf", 0),
    ("/usr/share/fonts/truetype/liberation/LiberationSans-Bold.ttf", 0),
];

#[cfg(not(target_arch = "wasm32"))]
#[cfg(target_os = "linux")]
fn load_from_fontconfig(bold: bool) -> Option<FontData> {
    let pattern = if bold { "sans-serif:style=Bold" } else { "sans-serif" };
    let output = std::process::Command::new("fc-match").args(["--format=%{file}\\n%{index}\\n", pattern]).output().ok()?;
    if !output.status.success() {
        return None;
    }
    let stdout = String::from_utf8(output.stdout).ok()?;
    let mut lines = stdout.lines();
    let path = lines.next()?.trim();
    let index = lines.next()?.trim().parse().ok()?;
    let bytes = std::fs::read(path).ok()?;
    let mut font = FontData::from_owned(bytes);
    font.index = index;
    Some(font)
}

#[cfg(not(target_arch = "wasm32"))]
fn load(cands: &[(&str, u32)], bold: bool) -> Option<FontData> {
    if std::env::var_os("SOUNDCRAFT_NO_SYSTEM_FONTS").is_some() {
        return None;
    }
    #[cfg(target_os = "linux")]
    if let Some(font) = load_from_fontconfig(bold) {
        return Some(font);
    }
    for (p, idx) in cands {
        if let Ok(bytes) = std::fs::read(p) {
            let mut fd = FontData::from_owned(bytes);
            fd.index = *idx;
            return Some(fd);
        }
    }
    None
}

#[cfg(target_arch = "wasm32")]
fn load(_: &[(&str, u32)], _: bool) -> Option<FontData> {
    None
}

pub fn definitions() -> FontDefinitions {
    let mut fonts = FontDefinitions::default();
    let base: Vec<String> = fonts.families.get(&FontFamily::Proportional).cloned().unwrap_or_default();
    let mut bold_family = base.clone();
    if let Some(reg) = load(REGULAR, false) {
        fonts.font_data.insert("system-regular".into(), Arc::new(reg));
        if let Some(f) = fonts.families.get_mut(&FontFamily::Proportional) {
            f.insert(0, "system-regular".into());
        }
    }
    if let Some(b) = load(BOLD, true) {
        fonts.font_data.insert("system-bold".into(), Arc::new(b));
        bold_family.insert(0, "system-bold".into());
    } else if fonts.font_data.contains_key("system-regular") {
        bold_family.insert(0, "system-regular".into());
    }
    fonts.families.insert(FontFamily::Name("bold".into()), bold_family);
    fonts
}

pub fn install(ctx: &egui::Context) {
    ctx.set_fonts(definitions());
}
