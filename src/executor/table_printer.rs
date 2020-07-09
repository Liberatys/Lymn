use prettytable::{color, Attr};
use prettytable::{Cell, Row, Table};

pub struct TablePrinter {}

impl TablePrinter {
    pub fn print(columns: Vec<String>, values: Vec<Vec<String>>) -> String {
        let mut table = Table::new();
        let mut columns_cells = Vec::new();
        for t in columns {
            columns_cells.push(Cell::new(t.as_ref()).with_style(Attr::Bold));
        }
        table.add_row(Row::new(columns_cells));
        for x in 0..values[0].len() {
            let mut row = Vec::new();
            for t in 0..values.len() {
                row.push(Cell::new(&values[t][x]));
            }
            table.add_row(Row::new(row));
        }
        table.to_string()
    }
}
