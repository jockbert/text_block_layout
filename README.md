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

Example [overlapping_boxes](examples/overlapping_boxes.rs) shows that blocks can
be layered over each other. Transperancy character can be configured.

```text
Blocks can be put on top of each other, with transparency!

OOOOO
O   O
O **O****
O * O   *
OOOOO   *
  *     *
  *     *
  *     *
  *******
```

Example named [invoice](examples/invoice.rs) shows yet another usage example.

```text
                                           INVOICE

  Acme                                        DATE 2020/01/01
  Where customers are billed             INVOICE # 12345678

  Address
  City, State ZIP



     SHIP TO Name                          BILL TO Name
             Address                               Address
             City, State ZIP                       City, State ZIP



  DESCRIPTION                           UNIT PRICE  QUANTITY     AMMOUNT
  ──────────────────────────────────────────────────────────────────────
  Toilet paper, 13-pack                     $ 3.95       200    $ 790.00
  Coffee, medium ground, 3 lbs              $ 6.95         4     $ 27.80
  ──────────────────────────────────────────────────────────────────────
                                                    SUBTOTAL    $ 817.80
                                                  ──────────────────────
                                                    TAX RATE         8 %
                                                  ──────────────────────
                                                   SALES TAX     $ 65.42
                                                  ──────────────────────
                                                       TOTAL    $ 883.22
                                                  ══════════════════════
```
