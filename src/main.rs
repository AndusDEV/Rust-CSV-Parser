use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("usernames.csv").expect("Failed to open file");
    let reader = BufReader::new(file);
    let mut table: Vec<Vec<String>> = Vec::new();
    let mut table_started = false;

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let cells: Vec<String> = line.split(',').map(|s| s.trim().to_string()).collect();
        if cells.is_empty() {
            if table_started {
                display_table(&table);
                println!();
                table = Vec::new();
                table_started = false;
            }
        } else {
            table.push(cells);
            table_started = true;
        }
    }
    if table_started {
        display_table(&table);
    }
}

fn display_table(table: &[Vec<String>]) {
    let num_cols = table[0].len();
    let mut col_widths = vec![0; num_cols];
    for row in table {
        for (i, cell) in row.iter().enumerate() {
            col_widths[i] = col_widths[i].max(cell.len());
        }
    }
    for row in table {
        for (i, cell) in row.iter().enumerate() {
            let width = col_widths[i];
            print!("{:^width$}", cell, width = width);
            if i < num_cols - 1 {
                print!("  ");
            }
        }
        println!();
    }
}