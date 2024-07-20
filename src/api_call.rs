use std::io; 
use reqwest::Request;
use serde::{Deserialize, Serialize};
use colored::*; // Importing colored crate for text coloring

#[derive(Deserialize,Serialize, Debug)]
#[derive(Clone)]
pub struct Coin_price{
    usd: f64,
    usd_market_cap: f64,
    usd_24h_vol: f64,
    usd_24h_change: f64,
    last_updated_at: String
}

 
#[derive(Deserialize,Serialize, Debug)]
#[derive(Clone)]
pub struct Coin{
  pub id: String,
  pub symbol: String,
  pub name: String,
  pub image: String,
  pub current_price: f64,
  pub market_cap: u64,
  pub market_cap_rank: u32,
  pub fully_diluted_valuation: Option<u64>,
  pub total_volume: u64,
  pub high_24h: f64,
  pub low_24h: f64,
  pub price_change_24h: f64,
  pub price_change_percentage_24h: f64,
  pub market_cap_change_24h: f64,
  pub market_cap_change_percentage_24h: f64,
  pub circulating_supply: f64,
  pub total_supply: f64,
  pub max_supply: Option<f64>,
  pub ath: f64,
  pub ath_change_percentage: f64,
  pub ath_date: String,
  pub atl: f64,
  pub atl_change_percentage: f64,
  pub atl_date: String,
  pub roi: Option<serde_json::Value>,
  pub last_updated: String,  
}
fn concat_url(consulta :&str ,coin: Vec<&str>, api_key01 : &str)->String{
      //Constructing de list of coin
          
      let inicio = "vs_currency=usd&ids=";
      let coin_str = concat_vector_to_string(coin , inicio);
      //println!("{}", coin_str);
      let url = match consulta {
        "all" => format!("https://api.coingecko.com/api/v3/coins/markets?{coin_str}&x_cg_demo_api_key={api_key01}"),
        "price" =>format!("https://api.coingecko.com/api/v3/simple/price?{coin_str}&x_cg_demo_api_key={api_key01}"),
        _ => format!("Caso no reconocido: {}", consulta.bright_red())
      };
      url
    }
fn concat_vector_to_string(vec: Vec<&str>, inicio : &str)-> String{
    let mut resultado = inicio.to_string();
   // let inicio = "=vs_currency=usd&ids=";
    let separador ="%2C";

    resultado.push_str(&vec[0]);
    
          for elemento in &vec[1..] {
              resultado.push_str(separador);
              resultado.push_str(elemento);
          }
         resultado 
}

  pub fn get_coins_list_full( coin: Vec<&str>, api_key01 : &str)->Result< Vec<Coin> , reqwest::Error>{

    
    //println!("{:#?}",url.bright_blue());

    let url = concat_url("all",coin,api_key01);
    
    // Sending a blocking GET request to the API endpoint
    let response = reqwest::blocking::get(&url)?;
      
    // Parsing the JSON response into WeatherResponse struct
    let response_json : Vec<Coin>= response.json()?;
    
   //println!("{:?}", &response_json);

    Ok(response_json) // Returning the deserialized response
  }
  pub fn get_coins_price( coin: Vec<&str>, api_key01 : &str)->Result< Vec<Coin_price> , reqwest::Error>{
    //Constructing url , to api
    let url = concat_url("price",coin,api_key01);
    
    // Sending a blocking GET request to the API endpoint
    let response = reqwest::blocking::get(&url)?;
     println!("{}", url.bright_blue()); 
    // Parsing the JSON response into WeatherResponse struct
    let response_json : Vec<Coin_price>= response.json()?;
    
   //println!("{:?}", &response_json);

    Ok(response_json) // Returning the deserialized response
  }


