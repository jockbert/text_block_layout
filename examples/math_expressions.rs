use std::cmp::max;

use text_block_layout::Block;

fn num(n: i64) -> Block {
    n.to_string().into()
}

fn add(a: Block, b: Block) -> Block {
    let operator = " + ".into();
    a.beside_center_bottom(&operator).beside_center_bottom(&b)
}

fn pow(base: Block, exponent: Block) -> Block {
    base.pad_top(exponent.height()).beside_top(&exponent)
}

fn mult(term1: Block, term2: Block) -> Block {
    term1.pad_right(1).beside_center_bottom(&term2)
}

fn div(dividend: Block, divisor: Block) -> Block {
    let width = max(dividend.width(), divisor.width());

    let bar = Block::of_width(width).fill_bottom(1, '─');

    dividend.stack_left(&bar).stack_center_right(&divisor)
}

fn func(name: Block, argument: Block) -> Block {
    name.beside_center_bottom(&paren(argument))
}

fn growing_middle_stack(total_height: usize, top: char, middle: char, bottom: char) -> Block {
    let t: Block = top.into();
    let b = bottom.into();
    let m = Block::of_width(1).fill_bottom(total_height - 2, middle);
    t.stack_left(&m).stack_left(&b)
}

fn integral(expr: Block, differential_over_variable: &str) -> Block {
    let symbol = Block::of_text("⌠").add_text("⎮").add_text("⌡");

    symbol
        .pad_right(1)
        .beside_center_top(&expr)
        .pad_right(1)
        .beside_center_top(&differential_over_variable.into())
}

/// Parenthesises around expression
fn paren(expr: Block) -> Block {
    let left = match expr.height() {
        0..=1 => Block::of_text("("),
        n => growing_middle_stack(n, '⎛', '⎜', '⎝'),
    };

    let right = match expr.height() {
        0..=1 => Block::of_text(")"),
        n => growing_middle_stack(n, '⎞', '⎟', '⎠'),
    };

    left.beside_center_bottom(&expr)
        .beside_center_bottom(&right)
}

fn pow2(base: Block) -> Block {
    pow(base, num(2))
}

fn e() -> Block {
    "e".into()
}

fn equals(l: Block, r: Block) -> Block {
    l.beside_center_bottom(&"  =  ".into())
        .beside_center_bottom(&r)
}

fn main() {
    let expr1 = integral(func(pow2("cos".into()), "x".into()), "dx");

    let expr2 = integral(
        pow2(paren(div(
            add(pow(e(), "ix".into()), pow(e(), "-ix".into())),
            num(2),
        ))),
        "dx",
    );

    let expr3 = mult(
        div(num(1), num(4)),
        integral(
            paren(add(
                pow(e(), "2ix".into()),
                add(num(2), pow(e(), "-2ix".into())),
            )),
            "dx",
        ),
    );

    let expr4 = add(
        mult(
            div(num(1), num(4)),
            paren(add("2x".into(), func("sin".into(), "2x".into()))),
        ),
        "C".into(),
    );

    let left_column = expr1.width();

    let line1 = equals(expr1, expr2);
    let line2 = equals(Block::of_width(left_column), expr3);
    let line3 = equals(Block::of_width(left_column), expr4);

    let calculation = line1
        .pad_bottom(2)
        .stack_left(&line2)
        .pad_bottom(2)
        .stack_left(&line3);

    println!("{}", &calculation.to_string());
}
