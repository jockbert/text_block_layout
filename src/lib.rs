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

/** Adjust the length of string to be exactly the given length. Too long
strings are truncated and to short strings are padded wuth spaces at the end. */
fn adjust_to_len(original: &str, length: usize) -> String {
    let mut result = original.to_string();
    result.truncate(length);

    let suffix = repeat(' ', length - result.len());
    result.push_str(&suffix);
    result
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
        Block::of_width(text.len()).add_line(text)
    }

    /** Return height of block. */
    pub fn height(&self) -> usize {
        self.lines.len()
    }

    /** Return width of block. */
    pub fn width(&self) -> usize {
        self.width
    }

    /** Add given text at bottom of block, incementing the height */
    pub fn add_line(&self, text: &str) -> Block {
        let mut result = self.clone();
        // TODO text is not padded / truncades correctly
        result.lines.push(adjust_to_len(text, result.width));
        result
    }

    /** Fill right side of block with given number of the filler character. */
    pub fn fill_right(&self, width: usize, filler: char) -> Block {
        let suffix = repeat(filler, width);

        let lines = self
            .lines
            .iter()
            .map(|line| line.to_string() + &suffix)
            .collect::<Vec<String>>();

        Block { width, lines }
    }

    /** Pad right side of block with given number of spaces. */
    pub fn pad_right(&self, width: usize) -> Block {
        self.fill_right(width, ' ')
    }

    /** Pad left side of block with given number of spaces. */
    pub fn pad_left(&self, width: usize) -> Block {
        Block::of_width(width).left_of(self)
    }

    /** Pad bottom side of block with given number of empty lines. */
    pub fn pad_bottom(&self, height: usize) -> Block {
        let padding = adjust_to_len("", self.width);

        let mut result = self.clone();
        for _ in 0..height {
            result.lines.push(padding.clone())
        }
        result
    }

    /** Pad right so given width is reached. */
    pub fn pad_right_to_width(&self, width: usize) -> Block {
        if width > self.width {
            self.pad_right(width - self.width)
        } else {
            self.clone()
        }
    }

    /** Pad bottom so given height is reached. */
    pub fn pad_bottom_to_height(&self, height: usize) -> Block {
        if height > self.height() {
            self.pad_bottom(height - self.height())
        } else {
            self.clone()
        }
    }

    /** Glue togeter two blocks horizontally, self to the left and the given
    block to the right. Differences in height will be compensated for by
    padding on bottom of blocks. */
    pub fn left_of(&self, right: &Block) -> Block {
        let left_padded = self.pad_bottom_to_height(right.height());
        let right_padded = right.pad_bottom_to_height(self.height());

        let lines = left_padded
            .lines
            .iter()
            .zip(right_padded.lines.iter())
            .map(|a| a.0.to_string() + a.1)
            .collect::<Vec<String>>();

        Block {
            width: left_padded.width + right_padded.width,
            lines,
        }
    }

    /** Glue together two blocks vertically, self on the top and the given
    block on the bottom. Differences in width will be compensated for by
    padding on right side of blocks. */
    pub fn above(&self, bottom: Block) -> Block {
        let top_padded = self.pad_right_to_width(bottom.width);
        let bottom_padded = bottom.pad_right_to_width(self.width);

        let lines = top_padded
            .lines
            .iter()
            .cloned()
            .chain(bottom_padded.lines.iter().cloned())
            .collect::<Vec<String>>();

        Block {
            width: top_padded.width,
            lines,
        }
    }

    /** Render a string from a block using '\n' as separator between lines */
    pub fn render(&self) -> String {
        self.lines.join("\n")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn above() {
        let a = Block::of_text("aaa");
        let b = Block::of_text("b").add_line("b").pad_left(1);

        assert_eq!("aaa", a.render());
        assert_eq!(" b\n b", b.render());

        assert_eq!("aaa\n b \n b ", a.above(b).render());
    }
}
