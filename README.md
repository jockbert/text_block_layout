# text_block_layout

Rust library for joining together blocks of text characters, in for example a
TUI or text document.

The key feature of the library is that it enables you to easily specify how
blocks of text should be positioned in relation to other block, by joining
blocks together, either vertically or horizontally, and using paddings and
fills.

## Examples

See Examples directory for example uses.

The text block layout is used in the
[math_expressions](examples/math_expressions.rs) example to generates the
following output:

```text
                               2
⌠    2           ⌠ ⎛ ix    -ix⎞
⎮ cos (x) dx  =  ⎮ ⎜e   + e   ⎟  dx
⌡                ⌡ ⎜──────────⎟
                   ⎝     2    ⎠


                 1 ⌠ ⎛ 2ix        -2ix⎞
              =  ─ ⎮ ⎝e    + 2 + e    ⎠ dx
                 4 ⌡


                 1
              =  ─ (2x + sin(2x)) + C
                 4
```
