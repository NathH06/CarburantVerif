use std::io::{self, BufReader};
use std::fs::File;
use reqwest::blocking;
pub use serde_json::Value;

struct PriceListStruct {
    index: usize,
    price: f64
}

struct ConfigStruct {
    url: &'static str,
    data_author: &'static str,
    lib_author: &'static str
}

fn make_config() -> ConfigStruct {
    ConfigStruct {
        url: "https://www.data.gouv.fr/api/1/datasets/r/b0561905-7b5e-4f38-be50-df05708acb80",
        data_author: "Ministères économiques et financiers.",
        lib_author: "Nathan H"
    }
}

fn search_by(search_type: &str, search_data: &str, subdata: bool) -> Vec<Value> {
    let datas: Value = get_json_data();
    let mut res:Vec<Value> = vec![];

    if let Some(data_array) = datas.as_array() {
        for data in data_array.iter() {
            let val_str = data.get(search_type).and_then(|v| v.as_str()).unwrap_or("");

            if search_data.is_empty() || val_str == search_data {
                if subdata {
                    res.push(data.clone());
                } 
                else {
                    if let Some(field_val) = data.get(search_type) {
                        res.push(field_val.clone());
                    }
                }
            }
        }
    }

    res
}


#[derive(Default)]
pub struct CarburantStruct {
    pub ville: String,
    pub lat: String,
    pub long: String,
    pub adresse: String,
    pub type_name: String,
    pub disponibility: bool,
    pub prix: f64
}

pub fn get_carburant_list () -> Vec<&'static str> {
    vec![
        "Gazole",
        "SP95",
        "SP98",
        "E10",
        "E85",
        "GPLc"
    ]
}

pub fn get_data_author() -> &'static str {
    make_config().data_author
}

pub fn get_lib_author() -> &'static str {
    make_config().lib_author
}

pub fn update_data() {
    let url:&str = make_config().url;
    let client: blocking::Client = blocking::Client::new();

    let mut resp:blocking::Response = client.get(url).send().expect("Request is invalid !");
    let mut out: File = File::create("data_carburant.json").expect("File is not created !");
    io::copy(&mut resp, &mut out).expect("failed to copy !");
}

pub fn get_json_data() -> Value {
    let file: File = File::open("data_carburant.json").expect("failed to open file !");
    let reader: BufReader<File> = BufReader::new(file);
    serde_json::from_reader(reader).expect("failed to convert data !")
}

pub fn get_city_list() -> Vec<Value> {
    let mut res: Vec<Value> = search_by("ville", "", false);

    res.sort_by_key(|v| v.to_string());
    res.dedup();
    res
}

pub fn search_by_city(city: &str) -> Vec<Value> {
    search_by("ville", city, true)
}

pub fn search_by_adresse(adresse: &str) -> Vec<Value> {
    search_by("adresse", adresse, true)
}

pub fn search_by_postal_code(cp:&str ) -> Vec<Value> {
    search_by("cp", cp, true)
}

pub fn get_lower_price(stations: &[Value], carburant_type: &str) -> CarburantStruct {
    let mut carb_price_list: Vec<PriceListStruct> = vec![];
    let mut result_station: CarburantStruct = CarburantStruct::default();
    let carb_option: &str = match carburant_type {
        "Gazole" => "gazole_prix",
        "SP95" => "sp95_prix",
        "SP98" => "sp98_prix",
        "E10" => "e10_prix",
        "E85" => "e85_prix",
        "GPLc" => "gplc_prix",
        _ => return CarburantStruct::default()
    };

    for (i, station) in stations.iter().enumerate() {
        carb_price_list.push(PriceListStruct {index: i, price: station.get(carb_option).and_then(|v| v.as_f64()).unwrap_or(f64::MAX)});
    }

    carb_price_list.sort_by(|a, b| a.price.total_cmp(&b.price));
    
    for price in carb_price_list.iter() {
        if price.price == f64::MAX {
            break;
        }
        
        let is_dispo: bool = stations[price.index].get("carburants_disponibles").and_then(|v| v.as_array()).is_some_and(|list| list.iter().any(|it| it.as_str() == Some(carburant_type)));
        if is_dispo {
            result_station.adresse = stations[price.index].get("adresse").and_then(|v| v.as_str()).unwrap_or("").to_string();
            result_station.disponibility = true;
            result_station.type_name = carburant_type.to_string();
            result_station.prix = price.price;
            result_station.ville = stations[price.index].get("ville").and_then(|v| v.as_str()).unwrap_or("").to_string();
            result_station.lat = stations[price.index].get("latitude").and_then(|v| v.as_str()).unwrap_or("").to_string();
            result_station.long = stations[price.index].get("longitude").and_then(|v| v.as_str()).unwrap_or("").to_string();

            break;
        }
    }

    result_station
}