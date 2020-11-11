//! # Example: Custom font
//!
//! Shows how to implement the `Font` trait for a custom `SeventSegmentFont` font. This font renders
//! numbers only and emulates a classic 7 segment display.

use embedded_graphics::{fonts::Text, pixelcolor::BinaryColor, prelude::*, style::MonoTextStyle};
use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, Window,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct SevenSegmentFont;

impl MonoFont for SevenSegmentFont {
    const FONT_IMAGE: &'static [u8] = include_bytes!("assets/seven-segment-font.raw");
    const FONT_IMAGE_WIDTH: u32 = 224;

    const CHARACTER_SIZE: Size = Size::new(22, 40);
    const CHARACTER_SPACING: u32 = 4;

    fn char_offset(c: char) -> u32 {
        c.to_digit(10).unwrap_or(0)
    }
}

fn main() -> Result<(), core::convert::Infallible> {
    let mut display: SimulatorDisplay<BinaryColor> = SimulatorDisplay::new(Size::new(128, 128));

    let position = Point::new(27, 22);
    let row_offset = Point::new(0, 44);

    Text::new("123", position)
        .into_styled(MonoTextStyle::new(SevenSegmentFont, BinaryColor::On))
        .draw(&mut display)?;

    Text::new("456", position + row_offset)
        .into_styled(MonoTextStyle::new(SevenSegmentFont, BinaryColor::On))
        .draw(&mut display)?;

    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::OledBlue)
        .build();
    Window::new("Custom font", &output_settings).show_static(&display);

    Ok(())
}
