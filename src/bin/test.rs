extern crate enum_map;
#[macro_use] extern crate enum_map_derive;

#[derive(Debug, EnumMap)]
enum Person { John, Tom, Nick }

#[derive(Debug, EnumMap)]
enum Currency { USD, EUR }

use enum_map::EnumMap;
use Person::*;
use Currency::*;

fn main() {
    // Create 2D EnumMap populated with f64::default(), which is 0.0
    let mut table : EnumMap<Person, EnumMap<Currency, f64>> = EnumMap::default();

    table[John][EUR] = 15.25;

    println!("table = {:?}", table);
    println!("table[John][EUR] = {:?}", table[John][EUR]);
}