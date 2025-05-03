use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use std::{collections::HashSet, fs, path::Path};

use coffeez::CoffeeBean;
use coffeez::Origin;

const BEANS_JSON_DEFAULT_PATH: &str = "./resources/beans.json";

fn main() {
    let beans_json_path = match env::var("COFFEEZ_BEANS_PATH") {
        Ok(val) => val,
        Err(_) => BEANS_JSON_DEFAULT_PATH.to_owned(),
    };
    
    let mut beans: HashSet<CoffeeBean> = HashSet::new();
    if Path::new(&beans_json_path).exists() {
        let bean_data = fs::read_to_string(&beans_json_path).expect("Unable to read beans.json!");
        beans = serde_json::from_str(&bean_data).expect("Unable to parse beans.json!");
    }

    let example_bean = CoffeeBean::new(
        "Alonso Bustos".to_owned(),
        Origin::Colombia,
        "Muttley & Jack's Coffee Roasters".to_owned(),
    );
    beans.insert(example_bean);

    let mut bean_file = OpenOptions::new()
        .read(false)
        .write(true)
        .create(true)
        .open(beans_json_path)
        .expect("Unable to open beans.json!");
    let bean_data = serde_json::to_string_pretty(&beans).expect("Unable to serialize beans!");
    bean_file
        .write_all(bean_data.as_bytes())
        .expect("Unable to write beans.json!");

    println!("Beans are: {:?}", beans);
}
