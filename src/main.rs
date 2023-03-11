
use std::{error::Error, io, process, collections::HashMap};
use serde::{Deserialize, Serialize};
use std::fs::File;
use reqwest;

#[derive(Debug, Deserialize)]
struct Transaction {
    timestamp: u64,
    transaction_type: String,
    token: String,
    amount: f64,
}

//Read CSV from file
async fn read_csv(transactions: &mut Vec<Transaction>) -> Result<(), Box<dyn Error>> {
    // let t_file = File::open("data/transactions.csv");

    // Build the CSV reader and iterate over each record.
    let mut rdr = csv::Reader::from_path("data/transactions.csv")?;
    // let mut transactions: Vec<Transaction> = Vec::new();
    for result in rdr.deserialize::<Transaction>() {
        let transaction: Transaction = result?;
        transactions.push(transaction);
    }

    // println!("{:?}", transactions);
    Ok(())
}
// Calculate token balances
fn calculate_balance(transactions: &Vec<Transaction>, balances: &mut std::collections::HashMap<String, f64>) {
    // let mut balances: std::collections::HashMap<String, f64> = std::collections::HashMap::new();
    for transaction in transactions {
        let balance = balances.entry(transaction.token.clone()).or_insert(0.0);
        if transaction.transaction_type == "DEPOSIT" {
            *balance += transaction.amount;
        } else {
            *balance -= transaction.amount;
        }
    }
    println!("{:?}", balances);
}

// Fetch exchange rates from Cryptocompare
async fn fetch_exchange_rates_from_cryptocompare(balances: &std::collections::HashMap<String, f64>, usd_rates: &mut std::collections::HashMap<String, f64>) -> Result<(), Box<dyn std::error::Error>> {
    let tokens: Vec<String> = balances.keys().cloned().collect();
    println!("{:?}", tokens);
    for token in tokens {
        let api_url = format!("https://min-api.cryptocompare.com/data/price?fsym={}&tsyms=USD",token);
    
        // let api_url = format!("https://min-api.cryptocompare.com/data/price?fsym={}&tsyms=USD", tokens.join(","));
        let response = reqwest::get(&api_url)
            .await?
            .json::<HashMap<String, f64>>()
            .await?;
        if let Some(val) = response.get("USD") { 
            let rate = val.clone();
            usd_rates.insert(token, rate);
         }
        
    // let rates: std::collections::HashMap<String, f64> = response.json().await?;
    println!("{:?}", response);
    }
    println!("{:?}", usd_rates);

    // Calculate portfolio value of each token
    let mut portfolio_values: Vec<(String, f64)> = Vec::new();
    for (token, balance) in balances {
        let rate = usd_rates.get(token).unwrap_or(&0.0);
        let value = balance * rate;
        portfolio_values.push((token.clone(), value));
    }
    println!("{:?}", portfolio_values);


    Ok(())
}

//     // Calculate portfolio value of each token
//     let mut portfolio_values: Vec<(String, f64)> = Vec::new();
//     for (token, balance) in &balances {
//         let rate = rates.get(token).unwrap_or(&0.0);
//         let value = balance * rate;
//         portfolio_values.push((token.clone(), value));
//     }

//     // Sort portfolio values by token name
//     portfolio_values.sort_by_key(|(token, _)| token.to_lowercase());

//     // Print portfolio values
//     for (token, value) in portfolio_values {
//         println!("{}: ${:.2}", token, value);
//     }

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut transactions: Vec<Transaction> = Vec::new();
    let mut balances: std::collections::HashMap<String, f64> = std::collections::HashMap::new();
    let mut usd_rates: std::collections::HashMap<String, f64> = std::collections::HashMap::new();


    read_csv(&mut transactions).await?;
    calculate_balance(&transactions, &mut balances);
    fetch_exchange_rates_from_cryptocompare(&balances, &mut usd_rates).await?;

    Ok(())
}