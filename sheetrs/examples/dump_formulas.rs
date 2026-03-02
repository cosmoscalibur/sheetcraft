use sheetrs::reader::CellValue;
use sheetrs::reader::read_workbook;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: dump_formulas <file>");
        return;
    }
    let path = &args[1];
    let workbook = read_workbook(path).unwrap();

    for sheet in workbook.sheets {
        println!("Sheet: {}", sheet.name);
        if let Some(err) = &sheet.formula_parsing_error {
            println!("  [FORMULA PARSING ERROR]: {}", err);
        }
        for ((row, col), cell) in sheet.cells {
            if let Some(formula) = &cell.formula {
                if cell.value.is_error() {
                    println!(
                        "  ({}, {}) [ERROR]: {} (Formula: {})",
                        row,
                        col,
                        cell.value.as_error().unwrap_or("Unknown"),
                        formula
                    );
                } else {
                    println!("  ({}, {}) [FORMULA]: {}", row, col, formula);
                }
            } else {
                match &cell.value {
                    CellValue::Text(t) => println!("  ({}, {}) [TEXT]: {}", row, col, t),
                    CellValue::Number(n) => println!("  ({}, {}) [NUMBER]: {}", row, col, n),
                    CellValue::Boolean(b) => println!("  ({}, {}) [BOOL]: {}", row, col, b),
                    CellValue::Error(e) => println!("  ({}, {}) [ERROR]: {}", row, col, e),
                    CellValue::Empty => println!("  ({}, {}) [EMPTY]", row, col),
                }
            }
        }
    }
}
