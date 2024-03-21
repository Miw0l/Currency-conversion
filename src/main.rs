use clap::Parser;
use reqwest::Error;
use serde_json::Value;

#[derive(Parser)]
#[derive(Clone)]
struct Cli {
    source_currency: String,
    target_currency: String,
    amount: i32,
}


async fn get_currencies_request(arg : Cli) -> Result<(), Error> {
    // getting all the possible currencies
    let response = reqwest::get("https://api.frankfurter.app/currencies").await?;

    let body = response.text().await?;
    // checking if the first currency provided by user is in the json file
    let arg_one_check = is_key_in_json(&body, &arg.source_currency);
    if !arg_one_check {
        panic!("Podano niewlasciwa walute w pierwszym argumencie");
    }
    // checking if the second currency provided by user is in the json file
    let arg_two_check = is_key_in_json(&body, &arg.target_currency);
    if !arg_two_check {
        panic!("Podano niewlasciwa walute w drugim argumencie");
    }
    
    Ok(())
}

fn is_key_in_json(json: &str, key: &str) -> bool {
    // looking for correct currency in the json file
    if let Ok(parsed_json) = serde_json::from_str::<Value>(json) {
        if let Some(obj) = parsed_json.as_object() {
            return obj.contains_key(key);
        }
    }
    false
}

fn parsin_json(body : &str, currency : String) -> Result<String, Error> {
    let v : Value = serde_json::from_str(&body).unwrap();
    let z : Value = serde_json::from_str(&v["rates"].to_string()).unwrap();

    let x = z[currency].to_string();
    Ok(x)
}

async fn get_convert_request(arg : Cli) -> Result<String, Error> {
    get_currencies_request(arg.clone()).await?;
    // sending request to the API with arguments provided by the user
    let url = format!("https://api.frankfurter.app/latest?amount={}&from={}&to={}", arg.amount, arg.source_currency, arg.target_currency);
    let response = reqwest::get(url).await?;
    let body = response.text().await?;

    if let Ok(converted_amount) = parsin_json(&body, arg.target_currency) {
        return Ok(converted_amount.to_string())
    } else {
        panic!("test")
    }
}

#[tokio::main]
async fn main(){
let args = Cli::parse();
    
let value = get_convert_request(args.clone()).await.unwrap();
print!("currency after conversion {} ", value);
}