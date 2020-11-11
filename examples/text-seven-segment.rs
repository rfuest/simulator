use bitflags::bitflags;
use embedded_graphics::{
    fonts::Text, pixelcolor::Rgb888, pixelcolor::WebColors, prelude::*, primitives::Rectangle,
    style::TextStyle,
};
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};
use std::convert::TryFrom;

fn main() -> Result<(), core::convert::Infallible> {
    let mut display = SimulatorDisplay::<Rgb888>::new(Size::new(500, 230));

    Text::new("0123456789", Point::new(20, 20))
        .into_styled(SevenSegmentTextStyle {
            digit_size: Size::new(16, 32),
            digit_spacing: 2,
            segment_width: 1,
            segment_color: Rgb888::CSS_STEEL_BLUE,
            inactive_segment_color: None,
        })
        .draw(&mut display)?;

    Text::new("12:42", Point::new(20, 100))
        .into_styled(SevenSegmentTextStyle {
            digit_size: Size::new(30, 50),
            digit_spacing: 5,
            segment_width: 5,
            segment_color: Rgb888::CSS_LIME_GREEN,
            inactive_segment_color: None,
        })
        .draw(&mut display)?;

    Text::new("123", Point::new(220, 20))
        .into_styled(SevenSegmentTextStyle {
            digit_size: Size::new(75, 150),
            digit_spacing: 20,
            segment_width: 15,
            segment_color: Rgb888::CSS_ORANGE_RED,
            inactive_segment_color: Some(Rgb888::new(20, 20, 20)),
        })
        .draw(&mut display)?;

    let output_settings = OutputSettingsBuilder::new().scale(2).build();
    Window::new("Custom font renderer", &output_settings).show_static(&display);

    Ok(())
}

// -----------------------------------------------------------------------------

pub struct SevenSegmentTextStyle<C> {
    digit_size: Size,
    digit_spacing: u32,
    segment_width: u32,
    segment_color: C,
    inactive_segment_color: Option<C>,
}

impl<C: PixelColor> SevenSegmentTextStyle<C> {
    fn draw_segment<D>(
        &self,
        rectangle: &Rectangle,
        active: bool,
        target: &mut D,
    ) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = C>,
    {
        let color = if active {
            self.segment_color
        } else if let Some(color) = self.inactive_segment_color {
            color
        } else {
            return Ok(());
        };

        let horizontal = rectangle.size.width > rectangle.size.height;
        let major_size = if horizontal {
            rectangle.size.height
        } else {
            rectangle.size.width
        };
        let offset = major_size / 2 + 1 + (major_size - 1) / 2;

        let mut rect = if horizontal {
            Rectangle::new(
                rectangle.top_left + Size::new(offset, 0),
                Size::new(rectangle.size.width - offset * 2, 1),
            )
        } else {
            Rectangle::new(
                rectangle.top_left + Size::new(0, offset),
                Size::new(1, rectangle.size.height - offset * 2),
            )
        };

        for _ in 0..(major_size + 1) / 2 {
            target.fill_solid(&rect, color)?;

            if horizontal {
                rect.top_left += Point::new(-1, 1);
                rect.size.width += 2;
            } else {
                rect.top_left += Point::new(1, -1);
                rect.size.height += 2;
            }
        }

        let delta = if major_size % 2 == 0 { 1 } else { 2 };
        if horizontal {
            rect.top_left.x += delta;
            rect.size.width -= delta as u32 * 2;
        } else {
            rect.top_left.y += delta;
            rect.size.height -= delta as u32 * 2;
        }

        for _ in 0..major_size / 2 {
            target.fill_solid(&rect, color)?;

            rect.top_left += Point::new(1, 1);
            if horizontal {
                rect.size.width -= 2;
            } else {
                rect.size.height -= 2;
            }
        }

        Ok(())
    }
}

impl<C: PixelColor> TextStyle for SevenSegmentTextStyle<C> {
    type Color = C;

    fn render_text<D>(&self, text: &Text<'_>, target: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        let mut position = text.position;

        for c in text.text.chars() {
            if let Ok(segments) = Segments::try_from(c) {
                let dx = self.digit_size.width - self.segment_width;

                let mut rect = Rectangle::new(
                    position,
                    Size::new(self.digit_size.width, self.segment_width),
                );
                self.draw_segment(&rect, segments.contains(Segments::A), target)?;

                rect.top_left += Size::new(0, (self.digit_size.height - self.segment_width) / 2);
                self.draw_segment(&rect, segments.contains(Segments::G), target)?;

                rect.top_left +=
                    Size::new(0, (self.digit_size.height - self.segment_width + 1) / 2);
                self.draw_segment(&rect, segments.contains(Segments::D), target)?;

                rect = Rectangle::new(
                    position,
                    Size::new(
                        self.segment_width,
                        self.digit_size.height / 2 + self.segment_width / 2,
                    ),
                );
                self.draw_segment(&rect, segments.contains(Segments::F), target)?;

                rect.top_left.x += dx as i32;
                self.draw_segment(&rect, segments.contains(Segments::B), target)?;

                rect = Rectangle::new(
                    position
                        + Size::new(0, self.digit_size.height / 2 - (self.segment_width + 1) / 2),
                    Size::new(
                        self.segment_width,
                        self.digit_size.height / 2 + (self.segment_width + 1) / 2,
                    ),
                );
                self.draw_segment(&rect, segments.contains(Segments::E), target)?;

                rect.top_left.x += dx as i32;
                self.draw_segment(&rect, segments.contains(Segments::C), target)?;

                position += self.digit_size.x_axis() + Size::new(self.digit_spacing, 0);
            } else if c == ':' {
                let dy = self.digit_size.height / 3;

                let mut rect = Rectangle::new(
                    position + Size::new(0, dy - self.segment_width / 2),
                    Size::new(self.segment_width, self.segment_width),
                );
                target.fill_solid(&rect, self.segment_color)?;

                rect.top_left += Size::new(0, dy);
                target.fill_solid(&rect, self.segment_color)?;

                position += Size::new(self.segment_width + self.digit_spacing, 0);
            } else {
                // TODO: add '.'
                // TODO: how should other characters be handled?
            }
        }

        Ok(())
    }

    fn bounding_box(&self, _text: &Text<'_>) -> Rectangle {
        todo!()
    }
}

// Segment layout:
//  AAAAA
// F     B
// F     B
// F     B
//  GGGGG
// E     C
// E     C
// E     C
//  DDDDD

bitflags! {
    struct Segments: u8 {
        const A = 0b01000000;
        const B = 0b00100000;
        const C = 0b00010000;
        const D = 0b00001000;
        const E = 0b00000100;
        const F = 0b00000010;
        const G = 0b00000001;
    }
}

impl TryFrom<char> for Segments {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '0' => Self::A | Self::B | Self::C | Self::D | Self::E | Self::F,
            '1' => Self::B | Self::C,
            '2' => Self::A | Self::B | Self::D | Self::E | Self::G,
            '3' => Self::A | Self::B | Self::C | Self::D | Self::G,
            '4' => Self::B | Self::C | Self::F | Self::G,
            '5' => Self::A | Self::C | Self::D | Self::F | Self::G,
            '6' => Self::A | Self::C | Self::D | Self::E | Self::F | Self::G,
            '7' => Self::A | Self::B | Self::C,
            '8' => Self::A | Self::B | Self::C | Self::D | Self::E | Self::F | Self::G,
            '9' => Self::A | Self::B | Self::C | Self::D | Self::F | Self::G,
            // TODO: add hex digits
            _ => return Err(()),
        })
    }
}
