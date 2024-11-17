use text_block_layout::Block;

/// Create a square with given border fill, size and position, as a block.
fn square(border: char, width: usize, offset_left: usize, offset_top: usize) -> Block {
    assert!(width >= 2);

    let top_line = Block::of_height(1).fill_right(width, border);

    let middle_lines = Block::of_height(width - 2)
        .fill_right(1, border)
        .pad_right(width - 2)
        .fill_right(1, border);

    top_line
        .stack_left(&middle_lines)
        .stack_left(&top_line)
        .pad_left(offset_left)
        .pad_top(offset_top)
}

fn main() {
    let frontmost = square('O', 5, 0, 0);
    let backmost = square('*', 7, 2, 2);

    println!("Blocks can be put on top of each other, with transparency!");
    println!();
    println!("{}", frontmost.in_front_of(&backmost));
}
