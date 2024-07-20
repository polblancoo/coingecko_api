use colored::Colorize;
mod api_call;
mod config;
mod coin_stdin;
mod vista;
use vista::print_coins_result01;

pub fn main(){

  //tomar datos del user  ---- vec!["polkadot", "bitcoin"];
 let vec_string =   coin_stdin::coin_stdin();
  // Convertir Vec<String> a Vec<&str>
  let coin: Vec<&str> = vec_string.iter().map(|s| s.as_str()).collect();

// let consulta = "all"; 
//levantar api_key de un archivo de configuracion 
 let (api_key01,  consulta) = config::leer_config();


 //process::exit(0);
//
if consulta == "all"{ 
  let result = api_call::get_coins_list_full(coin.clone(), &api_key01);
      if result.is_ok(){
        //let dato = format!("{:#?}", result);
        //println!("{:#?}",result); 
        print_coins_result01(result.unwrap());
      }else{
        println!("{}" , "Error al obtener los datos. ".bright_red() );
      }
};



//println!("{:#?}", result);


}
