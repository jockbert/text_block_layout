use unicode_width::UnicodeWidthStr;

/// Represents a block, i.e. a rectangle, of some width and height containing
/// text.
///
/// The key feature of a [Block] is that it enables you to easily specify
/// how chunks of text should be positioned in relation to other block, by
/// joining blocks together, either vertically or horizontally, and using
/// paddings and fills.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Block {
    width: usize,
    lines: Vec<String>,
}

/// Repeat a character a given ammount of times.
fn repeat(c: char, times: usize) -> String {
    std::iter::repeat(c).take(times).collect::<String>()
}

/// Subract usizes and clamp to positive results.
fn subtract_or_zero(a: usize, b: usize) -> usize {
    if a > b {
        a - b
    } else {
        0
    }
}

/// Join two blocks vertically, requiring blocks to have same width.
fn stack_same_width(top: &Block, bottom: &Block) -> Block {
    assert_eq!(top.width(), bottom.width());

    let lines = top
        .lines
        .iter()
        .cloned()
        .chain(bottom.lines.iter().cloned())
        .collect::<Vec<String>>();

    Block {
        width: top.width,
        lines,
    }
}

/// Join two blocks horizontally, requiring blocks to have same height.
fn beside_same_height(left: &Block, right: &Block) -> Block {
    assert_eq!(left.height(), right.height());

    let lines = left
        .lines
        .iter()
        .zip(right.lines.iter())
        .map(|a| a.0.to_string() + a.1)
        .collect::<Vec<String>>();

    Block {
        width: left.width + right.width,
        lines,
    }
}

impl Block {
    /// Create empty block with width and height zero.
    pub fn empty() -> Self {
        Block {
            width: 0,
            lines: vec![],
        }
    }

    /// Create block of given width and height 0.
    pub fn of_width(width: usize) -> Self {
        Block::empty().pad_right(width)
    }

    /// Create block of given height and width 0.
    pub fn of_height(height: usize) -> Self {
        Block::empty().pad_to_height_bottom(height)
    }

    /// Create block of given text. Uses width of text and height 1.
    fn of_string(text: String) -> Self {
        let width = UnicodeWidthStr::width(text.as_str());
        Block {
            width,
            lines: vec![text],
        }
    }

    /// Creates block of any argument implementing `std::string::ToString`
    /// trait, or implicitly by implementing `std::fmt::Display`. Uses
    /// String representation in block. See `Block::of_string`.
    pub fn of<T: ToString>(t: T) -> Self {
        Block::of_string(t.to_string())
    }

    /// Create block containing given text. Gets width of the text and height 1.
    #[deprecated(since = "1.2.0", note = "please use `Block::of` instead")]
    pub fn of_text(text: &str) -> Self {
        Block::of(text)
    }

    /// Return height of block.
    pub fn height(&self) -> usize {
        self.lines.len()
    }

    /// Return width of block.
    pub fn width(&self) -> usize {
        self.width
    }

    /// Add given text at bottom of block, incementing the height. Width of
    /// block will be increased if needed for added line to fit.
    pub fn add_text(&self, text: &str) -> Self {
        self.stack_left(&Block::of(text))
    }

    /// Add given text lines at bottom of block, incrementing the height
    /// accordingly. Width of block will be increades if needed.
    pub fn add_multiple_texts(&self, texts: &[String]) -> Self {
        let addition = texts
            .iter()
            .fold(Block::empty(), |acc, text| acc.add_text(text));

        self.stack_left(&addition)
    }

    /// Fill right side of block with given number of the filler character.
    pub fn fill_right(&self, width: usize, filler: char) -> Self {
        let suffix = repeat(filler, width);

        let lines = self
            .lines
            .iter()
            .map(|line| line.to_string() + &suffix)
            .collect::<Vec<String>>();

        Block {
            width: self.width + width,
            lines,
        }
    }

    /// Fill bottom side of block with given number of the filler character.
    pub fn fill_bottom(&self, height: usize, filler: char) -> Self {
        let padding = repeat(filler, self.width);

        let mut result = self.clone();
        for _ in 0..height {
            result.lines.push(padding.clone())
        }
        result
    }

    /// Pad right side of block with given number of spaces.
    pub fn pad_right(&self, width: usize) -> Self {
        self.fill_right(width, ' ')
    }

    /// Pad left side of block with given number of spaces.
    pub fn pad_left(&self, width: usize) -> Self {
        Block::of_width(width).beside_top(self)
    }

    pub fn pad_top(&self, height: usize) -> Self {
        Block::of_height(height).stack_left(self)
    }

    /// Pad bottom side of block with given number of empty lines.
    pub fn pad_bottom(&self, height: usize) -> Self {
        self.fill_bottom(height, ' ')
    }

    /// Pad right so given width is reached. Wider block is untouched.
    pub fn pad_to_width_right(&self, width: usize) -> Self {
        self.pad_right(subtract_or_zero(width, self.width))
    }

    /// Pad left so given width is reached. Wider block is untouched.
    pub fn pad_to_width_left(&self, width: usize) -> Self {
        self.pad_left(subtract_or_zero(width, self.width))
    }

    /// Pad both sides so given width is reached. Wider block is untouched.
    /// If padding needs to be uneven, there will be more padding on the
    /// right side.
    pub fn pad_to_width_center_right(&self, width: usize) -> Self {
        let padding = subtract_or_zero(width, self.width);
        let padding_left = padding / 2;
        let padding_right = padding - padding_left;
        self.pad_left(padding_left).pad_right(padding_right)
    }

    /// Pad both sides so given width is reached. Wider block is untouched.
    /// If padding needs to be uneven, there will be more padding on the
    /// left side.
    pub fn pad_to_width_center_left(&self, width: usize) -> Self {
        let padding = subtract_or_zero(width, self.width);
        let padding_right = padding / 2;
        let padding_left = padding - padding_right;
        self.pad_left(padding_left).pad_right(padding_right)
    }

    /// Pad top so given height is reached. Higher block is untouched.
    pub fn pad_to_height_top(&self, height: usize) -> Self {
        self.pad_top(subtract_or_zero(height, self.height()))
    }

    /// Pad bottom so given height is reached. Higher block is untouched.
    pub fn pad_to_height_bottom(&self, height: usize) -> Self {
        self.pad_bottom(subtract_or_zero(height, self.height()))
    }

    /// Pad both top and bottom so given height is reached. Higher block is
    /// untouched. If padding needs to be uneven, there will be more padding
    /// on the top side.
    pub fn pad_to_height_center_top(&self, height: usize) -> Self {
        let padding = subtract_or_zero(height, self.height());
        let padding_bottom = padding / 2;
        let padding_top = padding - padding_bottom;
        self.pad_bottom(padding_bottom).pad_top(padding_top)
    }

    /// Pad both top and bottom so given height is reached. Higher block is
    /// untouched. If padding needs to be uneven, there will be more padding
    /// on the bottom side.
    pub fn pad_to_height_center_bottom(&self, height: usize) -> Self {
        let padding = subtract_or_zero(height, self.height());
        let padding_top = padding / 2;
        let padding_bottom = padding - padding_top;
        self.pad_bottom(padding_bottom).pad_top(padding_top)
    }

    /// Join two blocks horizontally, self to the left and the given
    /// block to the right, aligning the top side of the blocks.
    pub fn beside_top(&self, right: &Block) -> Self {
        beside_same_height(
            &self.pad_to_height_bottom(right.height()),
            &right.pad_to_height_bottom(self.height()),
        )
    }

    /// Join two blocks horizontally, self to the left and the given
    /// block to the right, aligning the bottom side of the blocks.
    pub fn beside_bottom(&self, right: &Block) -> Self {
        beside_same_height(
            &self.pad_to_height_top(right.height()),
            &right.pad_to_height_top(self.height()),
        )
    }

    /// Join two blocks horizontally, self to the left and the given
    /// block to the right, aligning the center of the blocks.
    /// If padding needs to be uneven, there will be more padding on the
    /// top side.
    pub fn beside_center_bottom(&self, right: &Block) -> Self {
        beside_same_height(
            &self.pad_to_height_center_top(right.height()),
            &right.pad_to_height_center_top(self.height()),
        )
    }

    /// Join two blocks horizontally, self to the left and the given
    /// block to the right, aligning the center of the blocks.
    /// If padding needs to be uneven, there will be more padding on the
    /// bottom side.
    pub fn beside_center_top(&self, right: &Block) -> Self {
        beside_same_height(
            &self.pad_to_height_center_bottom(right.height()),
            &right.pad_to_height_center_bottom(self.height()),
        )
    }

    /// Join two blocks vertically, self on the top and the given
    /// block on the bottom, aligning the right side of the blocks.
    pub fn stack_right(&self, bottom: &Block) -> Self {
        stack_same_width(
            &self.pad_to_width_left(bottom.width),
            &bottom.pad_to_width_left(self.width),
        )
    }

    /// Join two blocks vertically, self on the top and the given
    /// block on the bottom, aligning the left side of the blocks.
    pub fn stack_left(&self, bottom: &Block) -> Self {
        stack_same_width(
            &self.pad_to_width_right(bottom.width),
            &bottom.pad_to_width_right(self.width),
        )
    }

    /// Join two blocks vertically, self on the top and the given
    /// block on the bottom, aligning the center of the blocks.
    /// If padding needs to be uneven, there will be more padding on the
    /// right side.
    pub fn stack_center_left(&self, bottom: &Block) -> Self {
        stack_same_width(
            &self.pad_to_width_center_right(bottom.width),
            &bottom.pad_to_width_center_right(self.width),
        )
    }

    /// Join two blocks vertically, self on the top and the given
    /// block on the bottom, aligning the center of the blocks.
    /// If padding needs to be uneven, there will be more padding on the
    /// left side.
    pub fn stack_center_right(&self, bottom: &Block) -> Self {
        stack_same_width(
            &self.pad_to_width_center_left(bottom.width),
            &bottom.pad_to_width_center_left(self.width),
        )
    }

    /// Overlays self in front of given block. Treats spaces as transparent
    /// characters.
    pub fn in_front_of(&self, behind: &Block) -> Self {
        self.in_front_of_with_transparency(behind, ' ')
    }

    /// Overlays self in front of given block, showing content of the block
    /// behind on the characters defined as transparent.
    pub fn in_front_of_with_transparency(&self, behind: &Block, transparency: char) -> Self {
        // Making sure the blocks is of same size
        let front = self
            .fill_right(subtract_or_zero(behind.width(), self.width()), transparency)
            .fill_bottom(
                subtract_or_zero(behind.height(), self.height()),
                transparency,
            );

        let back = behind
            .pad_to_width_right(self.width)
            .pad_to_height_bottom(self.height());

        // Zip characters and make sure frontmost is shown if not transparent
        let lines = front
            .lines
            .iter()
            .zip(back.lines.iter())
            .map(|(front_line, back_line)| {
                front_line
                    .chars()
                    .zip(back_line.chars())
                    .map(|(front_char, back_char)| {
                        if front_char == transparency {
                            back_char
                        } else {
                            front_char
                        }
                    })
                    .collect::<String>()
            })
            .collect::<Vec<String>>();

        Block {
            width: front.width,
            lines,
        }
    }

    /// Render a string from a block using '\n' as separator between lines.
    /// Trims away whitespace on the right side of each line, just to save on
    /// final string length.
    pub fn render(&self) -> String {
        self.lines
            .iter()
            .map(|line| line.trim_end())
            .collect::<Vec<_>>()
            .join("\n")
    }
}

impl From<char> for Block {
    fn from(c: char) -> Self {
        Block::of(c)
    }
}

impl From<&str> for Block {
    fn from(s: &str) -> Self {
        Block::of(s)
    }
}

impl From<String> for Block {
    fn from(text: String) -> Self {
        Block::of_string(text)
    }
}

impl std::fmt::Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.render())
    }
}

#[cfg(test)]
mod test {
    use std::fmt::Display;

    use super::*;

    #[test]
    fn above() {
        let a = Block::of("aaa");
        let b = Block::of("b").add_text("b").pad_left(1);

        assert_eq!("aaa", a.render());
        assert_eq!(" b\n b", b.render());
        assert_eq!("aaa\n b\n b", a.stack_left(&b).render());
    }

    #[test]
    fn trim_right_side_of_lines() {
        // Do not trim whitespace at left or middle of line
        let b = Block::of(" a a   ")
            // After trimming, these lines has other width than first line
            .add_text("bbbbb  ")
            .add_text("c  ")
            // These two empty lines should be trimmed down to zero lenght
            .pad_bottom(2);

        assert_eq!(" a a\nbbbbb\nc\n\n", b.render());
    }

    #[test]
    fn from_numbers() {
        assert_eq!("2.56", Block::of(2.56_f64).to_string());
        assert_eq!("2.58", Block::of(2.58_f32).to_string());
        assert_eq!("99", Block::of(99).to_string());
        assert_eq!("99", Block::of(99_i32).to_string());
        assert_eq!("99", Block::of(99_u128).to_string());
    }

    #[test]
    fn from_boolean() {
        assert_eq!("true", Block::of(true).to_string());
        assert_eq!("false", Block::of(false).to_string());
    }

    enum SomeEnum {
        Alfa,
        Beta,
    }

    impl Display for SomeEnum {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let text = match self {
                SomeEnum::Alfa => "aaa".to_string(),
                SomeEnum::Beta => "BbBbB".to_string(),
            };
            write!(f, "{}", text)
        }
    }

    #[test]
    fn from_display_trait() {
        assert_eq!("BbBbB", Block::of(SomeEnum::Beta).to_string());
        assert_eq!("aaa", Block::of(SomeEnum::Alfa).to_string());
    }
}
