use std::usize;
use text_block_layout::Block;

struct Item {
    description: String,
    unit_price: f64,
    quantity: u32,
}

impl Item {
    fn new(description: &str, unit_price: f64, quantity: u32) -> Item {
        Item {
            description: description.into(),
            unit_price,
            quantity,
        }
    }

    fn ammount(&self) -> f64 {
        self.unit_price * self.quantity as f64
    }
}

struct Invoice {
    date: String,
    invoice_no: String,
    company_name: String,
    company_slogan: String,
    company_addres: Vec<String>,
    bill_to: Vec<String>,
    ship_to: Vec<String>,
    items: Vec<Item>,
    tax_rate: f64,
}

impl Invoice {
    fn subtotal(&self) -> f64 {
        self.items
            .iter()
            .fold(0.0, |acc, item| acc + item.ammount())
    }

    fn sales_tax(&self) -> f64 {
        self.tax_rate * self.subtotal()
    }

    fn total(&self) -> f64 {
        self.sales_tax() + self.subtotal()
    }
}

fn info_single(left_column: usize, title: &str, content_line: &str) -> Block {
    info(left_column, title, &[content_line.to_string()])
}

fn info(left_column: usize, title: &str, content_lines: &[String]) -> Block {
    let left = Block::of(title).pad_to_width_left(left_column);
    let right = Block::empty().add_multiple_texts(content_lines);
    left.pad_right(1).beside_top(&right)
}

fn money(value: f64, width: usize) -> Block {
    Block::of(format!["$ {:.2}", value]).pad_to_width_left(width)
}

/// Create item specification with columns description (36), unit price (12),
/// quantity (10) and ammount (12)   
fn item_line(item: &Item) -> Block {
    let desc = Block::of(&item.description).pad_to_width_right(36);
    let unit = money(item.unit_price, 12);
    let quant = Block::of(item.quantity).pad_to_width_left(10);
    let amnt = money(item.ammount(), 12);
    desc.beside_top(&unit).beside_top(&quant).beside_top(&amnt)
}

fn create_text_invoice(i: &Invoice) -> Block {
    let page_width: usize = 70;
    let left_margin = 2;
    let info_left_margin = 10;
    let right_column = page_width - 32;

    // Invoice top -------------------------------------------------------------
    let company_info = Block::of(&i.company_name)
        .add_text(&i.company_slogan)
        .pad_bottom(1)
        .add_multiple_texts(&i.company_addres);

    let invoice_info = Block::of("INVOICE")
        .pad_to_width_left(10)
        .pad_bottom(1)
        .stack_left(&info_single(info_left_margin, "DATE", &i.date))
        .stack_left(&info_single(info_left_margin, "INVOICE #", &i.invoice_no));

    let top = company_info
        .pad_top(2)
        .in_front_of(&invoice_info.pad_left(right_column));

    // Customer addresses ------------------------------------------------------
    let ship_address = info(info_left_margin, "BILL TO", &i.ship_to);
    let bill_address = info(info_left_margin, "SHIP TO", &i.bill_to);
    let addresses = bill_address.in_front_of(&ship_address.pad_left(right_column));

    // Specification -----------------------------------------------------------
    let hline = Block::of_height(1).fill_right(page_width, '─');

    let item_header = Block::of("DESCRIPTION")
        .pad_to_width_right(36)
        .beside_top(&Block::of("UNIT PRICE").pad_to_width_left(12))
        .beside_top(&Block::of("QUANTITY").pad_to_width_left(10))
        .beside_top(&Block::of("AMMOUNT").pad_to_width_left(12));

    let items = i
        .items
        .iter()
        .fold(Block::empty(), |acc, item| acc.stack_left(&item_line(item)));

    let spec = item_header
        .stack_left(&hline)
        .stack_left(&items)
        .stack_left(&hline);

    // Totals ------------------------------------------------------------------
    let totals_width: usize = 22;
    let totals_hline = Block::of_height(1).fill_right(totals_width, '─');
    let totals_hline_thick = Block::of_height(1).fill_right(totals_width, '═');

    let subtotals = Block::of("SUBTOTAL").beside_top(&money(i.subtotal(), 12));
    let tax_rate = Block::of("TAX RATE")
        .beside_top(&Block::of(format!("{:.00} %", i.tax_rate * 100.0)).pad_to_width_left(12));
    let sales_tax = Block::of("SALES TAX").beside_top(&money(i.sales_tax(), 12));
    let totals = Block::of("TOTAL").beside_top(&money(i.total(), 12));

    let totals = subtotals
        .stack_right(&totals_hline)
        .stack_right(&tax_rate)
        .stack_right(&totals_hline)
        .stack_right(&sales_tax)
        .stack_right(&totals_hline)
        .stack_right(&totals)
        .stack_right(&totals_hline_thick)
        .pad_to_width_left(page_width);

    // Composition -------------------------------------------------------------
    top.pad_bottom(3)
        .stack_left(&addresses)
        .pad_bottom(3)
        .stack_left(&spec)
        .stack_left(&totals)
        .pad_left(left_margin)
}

fn main() {
    let invoice = Invoice {
        date: "2020/01/01".into(),
        invoice_no: "12345678".into(),
        company_name: "Acme".into(),
        company_slogan: "Where customers are billed".into(),
        company_addres: vec!["Address".into(), "City, State ZIP".into()],
        bill_to: vec!["Name".into(), "Address".into(), "City, State ZIP".into()],
        ship_to: vec!["Name".into(), "Address".into(), "City, State ZIP".into()],
        items: vec![
            Item::new("Toilet paper, 13-pack", 3.95, 2_00),
            Item::new("Coffee, medium ground, 3 lbs", 6.95, 4),
        ],
        tax_rate: 0.08,
    };

    println!("{}", create_text_invoice(&invoice).to_string());
}
