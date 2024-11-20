/*!

This library implements
[Unicode Vertical_Orientation Property](https://www.unicode.org/reports/tr50/tr50-19.html)
(annex #50).

```rust
use unicode_vo::*;

assert_eq!(char_orientation('A'), Orientation::Rotated);
assert_eq!(char_orientation('本'), Orientation::Upright);
```

*/

#![doc(html_root_url = "https://docs.rs/unicode-vo/0.1.0")]

#![forbid(unsafe_code)]

/// Character orientation.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Orientation {
    /// Characters which are displayed upright, with the same orientation that appears in
    /// the code charts.
    ///
    /// Original name: **U**
    Upright,

    /// Characters which are displayed sideways, rotated 90 degrees clockwise compared to
    /// the code charts.
    ///
    /// Original name: **R**
    Rotated,

    /// Characters which are not just upright or sideways, but generally require
    /// a different glyph than in the code charts when used in vertical texts.
    /// In addition, as a fallback, the character can be displayed with
    /// the code chart glyph upright.
    ///
    /// Original name: **Tu**
    TransformedOrUpright,

    /// Same as Tu except that, as a fallback, the character can be displayed with
    /// the code chart glyph rotated 90 degrees clockwise.
    ///
    /// Original name: **Tr**
    TransformedOrRotated,
}

/// Returns a vertical orientation of a character.
///
/// Based on <https://www.unicode.org/Public/vertical/revision-17/VerticalOrientation-17.txt>.
pub fn char_orientation(c: char) -> Orientation {
    match c as u32 {
        0x00A7 => Orientation::Upright,
        0x00A9 => Orientation::Upright,
        0x00AE => Orientation::Upright,
        0x00B1 => Orientation::Upright,
        0x00BC...0x00BE => Orientation::Upright,
        0x00D7 => Orientation::Upright,
        0x00F7 => Orientation::Upright,
        0x02EA...0x02EB => Orientation::Upright,
        0x1100...0x11FF => Orientation::Upright,
        0x1401...0x167F => Orientation::Upright,
        0x18B0...0x18FF => Orientation::Upright,
        0x2016 => Orientation::Upright,
        0x2020...0x2021 => Orientation::Upright,
        0x2030...0x2031 => Orientation::Upright,
        0x203B...0x203C => Orientation::Upright,
        0x2042 => Orientation::Upright,
        0x2047...0x2049 => Orientation::Upright,
        0x2051 => Orientation::Upright,
        0x2065 => Orientation::Upright,
        0x20DD...0x20E0 => Orientation::Upright,
        0x20E2...0x20E4 => Orientation::Upright,
        0x2100...0x2101 => Orientation::Upright,
        0x2103...0x2109 => Orientation::Upright,
        0x210F => Orientation::Upright,
        0x2113...0x2114 => Orientation::Upright,
        0x2116...0x2117 => Orientation::Upright,
        0x211E...0x2123 => Orientation::Upright,
        0x2125 => Orientation::Upright,
        0x2127 => Orientation::Upright,
        0x2129 => Orientation::Upright,
        0x212E => Orientation::Upright,
        0x2135...0x213F => Orientation::Upright,
        0x2145...0x214A => Orientation::Upright,
        0x214C...0x214D => Orientation::Upright,
        0x214F => Orientation::Upright,
        0x2150...0x2189 => Orientation::Upright,
        0x218C...0x218F => Orientation::Upright,
        0x221E => Orientation::Upright,
        0x2234...0x2235 => Orientation::Upright,
        0x2300...0x2307 => Orientation::Upright,
        0x230C...0x231F => Orientation::Upright,
        0x2324...0x2328 => Orientation::Upright,
        0x2329...0x232A => Orientation::TransformedOrRotated,
        0x232B => Orientation::Upright,
        0x237D...0x239A => Orientation::Upright,
        0x23BE...0x23CD => Orientation::Upright,
        0x23CF => Orientation::Upright,
        0x23D1...0x23DB => Orientation::Upright,
        0x23E2...0x23FF => Orientation::Upright,
        0x2400...0x2422 => Orientation::Upright,
        0x2424...0x243F => Orientation::Upright,
        0x2440...0x245F => Orientation::Upright,
        0x2460...0x24FF => Orientation::Upright,
        0x25A0...0x25FF => Orientation::Upright,
        0x2600...0x2619 => Orientation::Upright,
        0x2620...0x26FF => Orientation::Upright,
        0x2700...0x2767 => Orientation::Upright,
        0x2776...0x2793 => Orientation::Upright,
        0x2B12...0x2B2F => Orientation::Upright,
        0x2B50...0x2B59 => Orientation::Upright,
        0x2BB8...0x2BEB => Orientation::Upright,
        0x2BF0...0x2BFF => Orientation::Upright,
        0x2E80...0x2EFF => Orientation::Upright,
        0x2F00...0x2FDF => Orientation::Upright,
        0x2FE0...0x2FEF => Orientation::Upright,
        0x2FF0...0x2FFF => Orientation::Upright,
        0x3000 => Orientation::Upright,
        0x3001...0x3002 => Orientation::TransformedOrUpright,
        0x3003...0x3007 => Orientation::Upright,
        0x3008...0x3011 => Orientation::TransformedOrRotated,
        0x3012...0x3013 => Orientation::Upright,
        0x3014...0x301F => Orientation::TransformedOrRotated,
        0x3020...0x302F => Orientation::Upright,
        0x3030 => Orientation::TransformedOrRotated,
        0x3031...0x3040 => Orientation::Upright,
        0x3041 => Orientation::TransformedOrUpright,
        0x3042 => Orientation::Upright,
        0x3043 => Orientation::TransformedOrUpright,
        0x3044 => Orientation::Upright,
        0x3045 => Orientation::TransformedOrUpright,
        0x3046 => Orientation::Upright,
        0x3047 => Orientation::TransformedOrUpright,
        0x3048 => Orientation::Upright,
        0x3049 => Orientation::TransformedOrUpright,
        0x304A...0x3062 => Orientation::Upright,
        0x3063 => Orientation::TransformedOrUpright,
        0x3064...0x3082 => Orientation::Upright,
        0x3083 => Orientation::TransformedOrUpright,
        0x3084 => Orientation::Upright,
        0x3085 => Orientation::TransformedOrUpright,
        0x3086 => Orientation::Upright,
        0x3087 => Orientation::TransformedOrUpright,
        0x3088...0x308D => Orientation::Upright,
        0x308E => Orientation::TransformedOrUpright,
        0x308F...0x3094 => Orientation::Upright,
        0x3095...0x3096 => Orientation::TransformedOrUpright,
        0x3097...0x309A => Orientation::Upright,
        0x309B...0x309C => Orientation::TransformedOrUpright,
        0x309D...0x309F => Orientation::Upright,
        0x30A0 => Orientation::TransformedOrRotated,
        0x30A1 => Orientation::TransformedOrUpright,
        0x30A2 => Orientation::Upright,
        0x30A3 => Orientation::TransformedOrUpright,
        0x30A4 => Orientation::Upright,
        0x30A5 => Orientation::TransformedOrUpright,
        0x30A6 => Orientation::Upright,
        0x30A7 => Orientation::TransformedOrUpright,
        0x30A8 => Orientation::Upright,
        0x30A9 => Orientation::TransformedOrUpright,
        0x30AA...0x30C2 => Orientation::Upright,
        0x30C3 => Orientation::TransformedOrUpright,
        0x30C4...0x30E2 => Orientation::Upright,
        0x30E3 => Orientation::TransformedOrUpright,
        0x30E4 => Orientation::Upright,
        0x30E5 => Orientation::TransformedOrUpright,
        0x30E6 => Orientation::Upright,
        0x30E7 => Orientation::TransformedOrUpright,
        0x30E8...0x30ED => Orientation::Upright,
        0x30EE => Orientation::TransformedOrUpright,
        0x30EF...0x30F4 => Orientation::Upright,
        0x30F5...0x30F6 => Orientation::TransformedOrUpright,
        0x30F7...0x30FB => Orientation::Upright,
        0x30FC => Orientation::TransformedOrRotated,
        0x30FD...0x30FF => Orientation::Upright,
        0x3100...0x3126 => Orientation::Upright,
        0x3127 => Orientation::TransformedOrUpright,
        0x3128...0x312F => Orientation::Upright,
        0x3130...0x318F => Orientation::Upright,
        0x3190...0x319F => Orientation::Upright,
        0x31A0...0x31BF => Orientation::Upright,
        0x31C0...0x31EF => Orientation::Upright,
        0x31F0...0x31FF => Orientation::TransformedOrUpright,
        0x3200...0x321E => Orientation::Upright,
        0x321F...0x32FF => Orientation::Upright,
        0x3300...0x3357 => Orientation::TransformedOrUpright,
        0x3358...0x337A => Orientation::Upright,
        0x337B...0x337F => Orientation::TransformedOrUpright,
        0x3380...0x33FF => Orientation::Upright,
        0x3400...0x4DBF => Orientation::Upright,
        0x4DC0...0x4DFF => Orientation::Upright,
        0x4E00...0x9FFF => Orientation::Upright,
        0xA000...0xA48F => Orientation::Upright,
        0xA490...0xA4CF => Orientation::Upright,
        0xA960...0xA97F => Orientation::Upright,
        0xAC00...0xD7AF => Orientation::Upright,
        0xD7B0...0xD7FF => Orientation::Upright,
        0xE000...0xF8FF => Orientation::Upright,
        0xF900...0xFAFF => Orientation::Upright,
        0xFE10...0xFE19 => Orientation::Upright,
        0xFE1A...0xFE1F => Orientation::Upright,
        0xFE30...0xFE48 => Orientation::Upright,
        0xFE50...0xFE52 => Orientation::TransformedOrUpright,
        0xFE53...0xFE57 => Orientation::Upright,
        0xFE59...0xFE5E => Orientation::TransformedOrRotated,
        0xFE5F...0xFE62 => Orientation::Upright,
        0xFE67...0xFE6B => Orientation::Upright,
        0xFE6C...0xFE6F => Orientation::Upright,
        0xFF01 => Orientation::TransformedOrUpright,
        0xFF02...0xFF07 => Orientation::Upright,
        0xFF08...0xFF09 => Orientation::TransformedOrRotated,
        0xFF0A...0xFF0B => Orientation::Upright,
        0xFF0C => Orientation::TransformedOrUpright,
        0xFF0E => Orientation::TransformedOrUpright,
        0xFF0F...0xFF19 => Orientation::Upright,
        0xFF1A...0xFF1B => Orientation::TransformedOrRotated,
        0xFF1F => Orientation::TransformedOrUpright,
        0xFF20 => Orientation::Upright,
        0xFF21...0xFF3A => Orientation::Upright,
        0xFF3B => Orientation::TransformedOrRotated,
        0xFF3C => Orientation::Upright,
        0xFF3D => Orientation::TransformedOrRotated,
        0xFF3E => Orientation::Upright,
        0xFF3F => Orientation::TransformedOrRotated,
        0xFF40 => Orientation::Upright,
        0xFF41...0xFF5A => Orientation::Upright,
        0xFF5B...0xFF60 => Orientation::TransformedOrRotated,
        0xFFE0...0xFFE2 => Orientation::Upright,
        0xFFE3 => Orientation::TransformedOrRotated,
        0xFFE4...0xFFE7 => Orientation::Upright,
        0xFFF0...0xFFF8 => Orientation::Upright,
        0xFFFC...0xFFFD => Orientation::Upright,
        0x10980...0x1099F => Orientation::Upright,
        0x11580...0x115FF => Orientation::Upright,
        0x13000...0x1342F => Orientation::Upright,
        0x14400...0x1467F => Orientation::Upright,
        0x16FE0...0x16FFF => Orientation::Upright,
        0x17000...0x187FF => Orientation::Upright,
        0x18800...0x18AFF => Orientation::Upright,
        0x1B000...0x1B0FF => Orientation::Upright,
        0x1D000...0x1D0FF => Orientation::Upright,
        0x1D100...0x1D1FF => Orientation::Upright,
        0x1D300...0x1D35F => Orientation::Upright,
        0x1D360...0x1D37F => Orientation::Upright,
        0x1D800...0x1DAAF => Orientation::Upright,
        0x1F000...0x1F02F => Orientation::Upright,
        0x1F030...0x1F09F => Orientation::Upright,
        0x1F0A0...0x1F0FF => Orientation::Upright,
        0x1F100...0x1F1FF => Orientation::Upright,
        0x1F200...0x1F201 => Orientation::TransformedOrUpright,
        0x1F202...0x1F2FF => Orientation::Upright,
        0x1F300...0x1F5FF => Orientation::Upright,
        0x1F600...0x1F64F => Orientation::Upright,
        0x1F650...0x1F67F => Orientation::Upright,
        0x1F680...0x1F6FF => Orientation::Upright,
        0x1F700...0x1F77F => Orientation::Upright,
        0x1F780...0x1F7FF => Orientation::Upright,
        0x1F900...0x1F9FF => Orientation::Upright,
        0x20000...0x2A6DF => Orientation::Upright,
        0x2A6E0...0x2A6FF => Orientation::Upright,
        0x2A700...0x2B73F => Orientation::Upright,
        0x2B740...0x2B81F => Orientation::Upright,
        0x2B820...0x2CEAF => Orientation::Upright,
        0x2CEB0...0x2F7FF => Orientation::Upright,
        0x2F800...0x2FA1F => Orientation::Upright,
        0x2FA20...0x2FFFD => Orientation::Upright,
        0x30000...0x3FFFD => Orientation::Upright,
        0xF0000...0xFFFFD => Orientation::Upright,
        0x100000...0x10FFFD => Orientation::Upright,
        _ => Orientation::Rotated,
    }
}