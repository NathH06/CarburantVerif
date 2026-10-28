use carburant_lib::{self, CarburantStruct, Value};
use std::env;

fn update(long: usize) {
    if long == 1 {
        println!("Mise à jour des données en cours...");
        carburant_lib::update_data();
    }
    else{
        println!("Nombre d'arguments trop élevés !");
    }
}

fn search(long: usize, args: Vec<String>) {
    if long == 3 {
        let stations: Vec<Value> = match args[1].as_str() {
            "--city" => carburant_lib::search_by_city(args[2].as_str()),
            "--postal-code" => carburant_lib::search_by_postal_code(args[2].as_str()),
            "--address" => carburant_lib::search_by_adresse(args[2].as_str()),
            _ => vec![Value::Null]
        };

        for (i, station) in stations.iter().enumerate() {
            println!("Station {}: {} \n", i, station);
        }
    }
}

fn version (long: usize) {
    if long == 1 {
        println!("La version actuelle de ce programme est : {}", env!("CARGO_PKG_VERSION"));
    }
    else{
        println!("Nombre d'arguments trop élevés !");
    }
}

fn author(long: usize, args: Vec<String>) {
    if long == 2 {
        match args[1].as_str() {
            "--data" => println!("L'auteur des données utilisés est : {}", carburant_lib::get_data_author()),
            "--lib" => println!("L'auteur de la librairie des carburants est : {}", carburant_lib::get_lib_author()),
            _ => println!("L'auteur de ce programme est : {}", env!("CARGO_PKG_AUTHORS"))
        }
    }
    else{
        println!("L'auteur de ce programme est : {}", env!("CARGO_PKG_AUTHORS"));
    }
}

fn get(long: usize, args: Vec<String>) {
    let resp: CarburantStruct;

    if long == 5 {
        let mode: i32 = match args[1].as_str() {
            "--lower-price" => 1,
            _ => 0
        };

        let data: Vec<Value> = match args[3].as_str() {
            "--city" => carburant_lib::search_by_city(args[4].as_str()),
            "--postal-code" => carburant_lib::search_by_postal_code(args[4].as_str()),
            "--address" => carburant_lib::search_by_adresse(args[4].as_str()),
            _ => vec![Value::Null]
        };

        if mode == 1 {
            resp = carburant_lib::get_lower_price(&data, args[2].as_str());
        }
        else {
            resp = CarburantStruct::default();
        }

        println!("Informations pour la station la moins chère selon le paramètre {} {} :", args[3].as_str(), args[4].as_str());
        println!("Ville: {}  |  Adresse: {}  |  Prix {}: {}€", resp.ville, resp.adresse, resp.type_name, resp.prix);
    }
}

fn infos(long: usize) {
    if long == 1 {
        println!("Commande bientôt disponible.");
    }
    else{
        println!("Nombre d'arguments trop élevés !");
    }
}

fn list (long: usize) {
    if long == 1 {
        for (i, city) in carburant_lib::get_city_list().iter().enumerate() {
            println!("Ville {} : {}", i, city);
        }
    }
    else{
        println!("Nombre d'arguments trop élevés !");
    }
}

fn help (long: usize) {
    if long == 1 {
        println!("commandes: [ update | search *2 'data' | version | author *3 | get *1 'carburant_type' *2 'data' | infos | list | help ]");
        println!("*1 [ --lower-price, --upper-price ]");
        println!("*2 [ --city, --postal-code, --address ]");
        println!("*3 [ --data, --lib ]");
        print!("Liste des carburants : [");

        let carb_list: Vec<&str> = carburant_lib::get_carburant_list();
        for (i, carb) in carb_list.iter().enumerate(){
            if i != carb_list.len()-1 {
                print!(" {},", carb);
            }
            else{
                print!(" {} ]", carb);
            }
        }

        println!("Pour lister les villes ==> commande list");

        println!("Les paramètre et les commandes non disponible pour l'instant sont: [ infos ]");
        println!("Les arguments non disponibles pour l'instant sont: [ --upper-price ]");
    }
    else{
        println!("Nombre d'arguments trop élevés !");
    }
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    
    if !args.is_empty() {
        let long: usize = args.len();

        match args[0].as_str() {
            "update" => update(long),
            "search" => search(long, args),
            "version" => version(long),
            "author" => author(long, args),
            "get" => get(long, args),
            "infos" => infos(long),
            "list" => list(long),
            "help" => help(long),
            _ => println!("Commande invalide !")
        };
    }
    else{
        println!("Utilisation de la commande par défaut ==> help:\n");
        help(1);
    }
}