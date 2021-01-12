use unicode_width::UnicodeWidthStr;

/// Represents a block of some width an height containing text.
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
    pub fn empty() -> Block {
        Block {
            width: 0,
            lines: vec![],
        }
    }

    /// Create block of given width and height 0.
    pub fn of_width(width: usize) -> Block {
        Block::empty().pad_right(width)
    }

    /// Create block of given height and width 0.
    pub fn of_height(height: usize) -> Block {
        Block::empty().pad_to_height_bottom(height)
    }

    /// Create block containing given text. Gets width of the text and height 1.
    pub fn of_text(text: &str) -> Block {
        let width = UnicodeWidthStr::width(text);
        Block {
            width,
            lines: vec![text.to_string()],
        }
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
    pub fn add_text(&self, text: &str) -> Block {
        self.stack_left(&Block::of_text(text))
    }

    /// Add given text lines at bottom of block, incrementing the height
    /// accordingly. Width of block will be increades if needed.
    pub fn add_multiple_texts(&self, texts: &[String]) -> Block {
        let addition = texts
            .iter()
            .fold(Block::empty(), |acc, text| acc.add_text(text));

        self.stack_left(&addition)
    }

    /// Fill right side of block with given number of the filler character.
    pub fn fill_right(&self, width: usize, filler: char) -> Block {
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
    pub fn fill_bottom(&self, height: usize, filler: char) -> Block {
        let padding = repeat(filler, self.width);

        let mut result = self.clone();
        for _ in 0..height {
            result.lines.push(padding.clone())
        }
        result
    }

    /// Pad right side of block with given number of spaces.
    pub fn pad_right(&self, width: usize) -> Block {
        self.fill_right(width, ' ')
    }

    /// Pad left side of block with given number of spaces.
    pub fn pad_left(&self, width: usize) -> Block {
        Block::of_width(width).beside_top(self)
    }

    pub fn pad_top(&self, height: usize) -> Block {
        Block::of_height(height).stack_left(self)
    }

    /// Pad bottom side of block with given number of empty lines.
    pub fn pad_bottom(&self, height: usize) -> Block {
        self.fill_bottom(height, ' ')
    }

    /// Pad right so given width is reached. Wider block is untouched.
    pub fn pad_to_width_right(&self, width: usize) -> Block {
        self.pad_right(subtract_or_zero(width, self.width))
    }

    /// Pad left so given width is reached. Wider block is untouched.
    pub fn pad_to_width_left(&self, width: usize) -> Block {
        self.pad_left(subtract_or_zero(width, self.width))
    }

    /// Pad both sides so given width is reached. Wider block is untouched.
    /// If padding needs to be uneven, there will be more padding on the
    /// right side.
    pub fn pad_to_width_center_right(&self, width: usize) -> Block {
        let padding = subtract_or_zero(width, self.width);
        let padding_left = padding / 2;
        let padding_right = padding - padding_left;
        self.pad_left(padding_left).pad_right(padding_right)
    }

    /// Pad both sides so given width is reached. Wider block is untouched.
    /// If padding needs to be uneven, there will be more padding on the
    /// left side.
    pub fn pad_to_width_center_left(&self, width: usize) -> Block {
        let padding = subtract_or_zero(width, self.width);
        let padding_right = padding / 2;
        let padding_left = padding - padding_right;
        self.pad_left(padding_left).pad_right(padding_right)
    }

    /// Pad top so given height is reached. Higher block is untouched.
    pub fn pad_to_height_top(&self, height: usize) -> Block {
        self.pad_top(subtract_or_zero(height, self.height()))
    }

    /// Pad bottom so given height is reached. Higher block is untouched.
    pub fn pad_to_height_bottom(&self, height: usize) -> Block {
        self.pad_bottom(subtract_or_zero(height, self.height()))
    }

    /// Pad both top and bottom so given height is reached. Higher block is
    /// untouched. If padding needs to be uneven, there will be more padding
    /// on the top side.
    pub fn pad_to_height_center_top(&self, height: usize) -> Block {
        let padding = subtract_or_zero(height, self.height());
        let padding_bottom = padding / 2;
        let padding_top = padding - padding_bottom;
        self.pad_bottom(padding_bottom).pad_top(padding_top)
    }

    /// Pad both top and bottom so given height is reached. Higher block is
    /// untouched. If padding needs to be uneven, there will be more padding
    /// on the bottom side.
    pub fn pad_to_height_center_bottom(&self, height: usize) -> Block {
        let padding = subtract_or_zero(height, self.height());
        let padding_top = padding / 2;
        let padding_bottom = padding - padding_top;
        self.pad_bottom(padding_bottom).pad_top(padding_top)
    }

    /// Join two blocks horizontally, self to the left and the given
    /// block to the right, aligning the top side of the blocks.
    pub fn beside_top(&self, right: &Block) -> Block {
        beside_same_height(
            &self.pad_to_height_bottom(right.height()),
            &right.pad_to_height_bottom(self.height()),
        )
    }

    /// Join two blocks horizontally, self to the left and the given
    /// block to the right, aligning the bottom side of the blocks.
    pub fn beside_bottom(&self, right: &Block) -> Block {
        beside_same_height(
            &self.pad_to_height_top(right.height()),
            &right.pad_to_height_top(self.height()),
        )
    }

    /// Join two blocks horizontally, self to the left and the given
    /// block to the right, aligning the center of the blocks.
    /// If padding needs to be uneven, there will be more padding on the
    /// top side.
    pub fn beside_center_bottom(&self, right: &Block) -> Block {
        beside_same_height(
            &self.pad_to_height_center_top(right.height()),
            &right.pad_to_height_center_top(self.height()),
        )
    }

    /// Join two blocks horizontally, self to the left and the given
    /// block to the right, aligning the center of the blocks.
    /// If padding needs to be uneven, there will be more padding on the
    /// bottom side.
    pub fn beside_center_top(&self, right: &Block) -> Block {
        beside_same_height(
            &self.pad_to_height_center_bottom(right.height()),
            &right.pad_to_height_center_bottom(self.height()),
        )
    }

    /// Join two blocks vertically, self on the top and the given
    /// block on the bottom, aligning the right side of the blocks.
    pub fn stack_right(&self, bottom: &Block) -> Block {
        stack_same_width(
            &self.pad_to_width_left(bottom.width),
            &bottom.pad_to_width_left(self.width),
        )
    }

    /// Join two blocks vertically, self on the top and the given
    /// block on the bottom, aligning the left side of the blocks.
    pub fn stack_left(&self, bottom: &Block) -> Block {
        stack_same_width(
            &self.pad_to_width_right(bottom.width),
            &bottom.pad_to_width_right(self.width),
        )
    }

    /// Join two blocks vertically, self on the top and the given
    /// block on the bottom, aligning the center of the blocks.
    /// If padding needs to be uneven, there will be more padding on the
    /// right side.
    pub fn stack_center_left(&self, bottom: &Block) -> Block {
        stack_same_width(
            &self.pad_to_width_center_right(bottom.width),
            &bottom.pad_to_width_center_right(self.width),
        )
    }

    /// Join two blocks vertically, self on the top and the given
    /// block on the bottom, aligning the center of the blocks.
    /// If padding needs to be uneven, there will be more padding on the
    /// left side.
    pub fn stack_center_right(&self, bottom: &Block) -> Block {
        stack_same_width(
            &self.pad_to_width_center_left(bottom.width),
            &bottom.pad_to_width_center_left(self.width),
        )
    }

    /// Overlays self in front of given block. Treats spaces as transparent
    /// characters.
    pub fn in_front_of(&self, behind: &Block) -> Block {
        self.in_front_of_with_transparency(behind, ' ')
    }

    /// Overlays self in front of given block, showing content of the block
    /// behind on the characters defined as transparent.
    pub fn in_front_of_with_transparency(&self, behind: &Block, transparency: char) -> Block {
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
    fn from(text: char) -> Self {
        Block::of_text(&text.to_string())
    }
}

impl From<&str> for Block {
    fn from(text: &str) -> Self {
        Block::of_text(text)
    }
}

impl From<String> for Block {
    fn from(text: String) -> Self {
        Block::of_text(&text)
    }
}

impl ToString for Block {
    fn to_string(&self) -> String {
        self.render()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn above() {
        let a = Block::of_text("aaa");
        let b = Block::of_text("b").add_text("b").pad_left(1);

        assert_eq!("aaa", a.render());
        assert_eq!(" b\n b", b.render());

        assert_eq!("aaa\n b\n b", a.stack_left(&b).render());
    }

    #[test]
    fn trim_right_side_of_lines() {
        // Do not trim whitespace at left or middle of line
        let b = Block::of_text(" a a   ")
            // After trimming, these lines has other width than first line
            .add_text("bbbbb  ")
            .add_text("c  ")
            // These two empty lines should be trimmed down to zero lenght
            .pad_bottom(2);

        assert_eq!(" a a\nbbbbb\nc\n\n", b.render());
    }
}
