use unicode_width::UnicodeWidthStr;

/** Represents a block of some width an height containing text. */
#[derive(Clone)]
pub struct Block {
    width: usize,
    lines: Vec<String>,
}

/** Repeat a character a given ammount of times. */
fn repeat(c: char, times: usize) -> String {
    std::iter::repeat(c).take(times).collect::<String>()
}

/** Subract usizes and clamp to positive results. */
fn subtract_or_zero(a: usize, b: usize) -> usize {
    if a > b {
        a - b
    } else {
        0
    }
}

/** Join two blocks vertically, requiring blocks to have same width. */
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

/** Join two blocks horizontally, requiring blocks to have same height. */
pub fn beside_same_height(left: &Block, right: &Block) -> Block {
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
    /** Create empty block with width and height zero */
    pub fn empty() -> Block {
        Block {
            width: 0,
            lines: vec![],
        }
    }

    /** Create block of given width and height 0. */
    pub fn of_width(width: usize) -> Block {
        Block::empty().pad_right(width)
    }

    /** Create block of given height and width 0. */
    pub fn of_height(height: usize) -> Block {
        Block::empty().pad_bottom_to_height(height)
    }

    /** Create block containing given text. Gets width of the text and height 1. */
    pub fn of_text(text: &str) -> Block {
        let width = UnicodeWidthStr::width(text);
        Block {
            width,
            lines: vec![text.to_string()],
        }
    }

    /** Return height of block. */
    pub fn height(&self) -> usize {
        self.lines.len()
    }

    /** Return width of block. */
    pub fn width(&self) -> usize {
        self.width
    }

    /** Add given text at bottom of block, incementing the height. Width of
    block will be increased if needed for added line to fit.*/
    pub fn add_text(&self, text: &str) -> Block {
        self.stack_left(&Block::of_text(text))
    }

    /** Fill right side of block with given number of the filler character. */
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

    /** Fill bottom side of block with given number of the filler character. */
    pub fn fill_bottom(&self, height: usize, filler: char) -> Block {
        let padding = repeat(filler, self.width);

        let mut result = self.clone();
        for _ in 0..height {
            result.lines.push(padding.clone())
        }
        result
    }

    /** Pad right side of block with given number of spaces. */
    pub fn pad_right(&self, width: usize) -> Block {
        self.fill_right(width, ' ')
    }

    /** Pad left side of block with given number of spaces. */
    pub fn pad_left(&self, width: usize) -> Block {
        Block::of_width(width).beside_top(self)
    }

    pub fn pad_top(&self, height: usize) -> Block {
        Block::of_height(height).stack_left(self)
    }

    /** Pad bottom side of block with given number of empty lines. */
    pub fn pad_bottom(&self, height: usize) -> Block {
        self.fill_bottom(height, ' ')
    }

    /** Pad right so given width is reached. Wider block is untouched. */
    pub fn pad_right_to_width(&self, width: usize) -> Block {
        self.pad_right(subtract_or_zero(width, self.width))
    }

    /** Pad left so given width is reached. Wider block is untouched. */
    pub fn pad_left_to_width(&self, width: usize) -> Block {
        self.pad_left(subtract_or_zero(width, self.width))
    }

    /** Pad top so given height is reached. Higher block is untouched. */
    pub fn pad_top_to_height(&self, height: usize) -> Block {
        self.pad_top(subtract_or_zero(height, self.height()))
    }

    /** Pad bottom so given height is reached. Higher block is untouched. */
    pub fn pad_bottom_to_height(&self, height: usize) -> Block {
        self.pad_bottom(subtract_or_zero(height, self.height()))
    }

    /** Join two blocks horizontally, self to the left and the given
    block to the right, aligning the top side of the blocks. */
    pub fn beside_top(&self, right: &Block) -> Block {
        beside_same_height(
            &self.pad_bottom_to_height(right.height()),
            &right.pad_bottom_to_height(self.height()),
        )
    }

    /** Join two blocks horizontally, self to the left and the given
    block to the right, aligning the bottom side of the blocks. */
    pub fn beside_bottom(&self, right: &Block) -> Block {
        beside_same_height(
            &self.pad_top_to_height(right.height()),
            &right.pad_top_to_height(self.height()),
        )
    }

    /** Join two blocks vertically, self on the top and the given
    block on the bottom, aligning the right side of the blocks. */
    pub fn stack_right(&self, bottom: &Block) -> Block {
        stack_same_width(
            &self.pad_left_to_width(bottom.width),
            &bottom.pad_left_to_width(self.width),
        )
    }

    /** Join two blocks vertically, self on the top and the given
    block on the bottom, aligning the left side of the blocks. */
    pub fn stack_left(&self, bottom: &Block) -> Block {
        stack_same_width(
            &self.pad_right_to_width(bottom.width),
            &bottom.pad_right_to_width(self.width),
        )
    }

    /** Render a string from a block using '\n' as separator between lines.
    Trims away whitespace on the right side of each line, just to save on final
    string length. */
    pub fn render(&self) -> String {
        self.lines
            .iter()
            .map(|line| line.trim_end())
            .collect::<Vec<_>>()
            .join("\n")
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
