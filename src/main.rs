// use std::fs::File;
// use std::io::{self, BufRead, BufReader};
// mod location;
// mod transaction;
// use transaction::Transaction;
// fn main() {
//     let file = File::open("transactions.csv").unwrap();
//     let reader = BufReader::new(file);
//     let mut transactions: Vec<Transaction> = Vec::new();
//     let mut skipped_lines = Vec::new();
//     for (idx, line) in reader.lines().enumerate() {
//         if idx == 0 {
//              continue;
//          }
//         let line_str = line.unwrap();
//         let parsed_transaction = Transaction::from_csv_line(&line_str);
//         match parsed_transaction {
//             Ok(tx) => transactions.push(tx),
//             Err(err) => skipped_lines.push((idx, err, line_str)),
//         }
//     }
//     for tx in transactions {
//         println!("{:?}", tx);
    
//     }
// }

use std::fs::File;
use std::io::{BufRead, BufReader};

mod location;
mod transaction;
use transaction::Transaction;
use location::Continent;
use std::collections::HashMap;


fn print_transactions_by_continent(transactions: &[Transaction], continent: &Continent) {
    transactions.iter()
        .filter(|tx| &tx.continent == continent)
        .for_each(|tx| println!("{:?}", tx));
}

fn main() {
    let file = File::open("transactions.csv").unwrap();
    let reader = BufReader::new(file);
    let mut transactions: Vec<Transaction> = Vec::new();
    let mut skipped_lines = Vec::new();
    for (idx, line) in reader.lines().enumerate() {
        if idx == 0 {
            continue;
        }
        let line_str = line.unwrap();
        let parsed_transaction = Transaction::from_csv_line(&line_str);
        match parsed_transaction {
            Ok(tx) => transactions.push(tx),
            Err(err) => skipped_lines.push((idx, err, line_str)),
        }
    }

    // Utilize HashMap to keep track of the total invested amount per continent and print the result out for each continent
    let mut totals: HashMap<String, f64> = HashMap::new();
    for tx in &transactions {
    // You would need to convert the continent to String to store as keys
    let key = format!("{:?}", tx.continent); 
    *totals.entry(key).or_insert(0.0) += tx.amount;
    }
    for (continent, total) in &totals {
    println!("Total invested in {}: {}", continent, total);
    }

    //Print only European transactions
    print_transactions_by_continent(&transactions, &Continent::Europe);

    // Print all transactions
    // for tx in &transactions {
    //     println!("{:?}", tx);
    // }


}
