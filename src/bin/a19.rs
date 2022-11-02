// HashMap
use std::collections::HashMap;

fn main() {
    let mut hashmap = HashMap::new();

    hashmap.insert("Couche", 0);
    hashmap.insert("Table", 2);
    hashmap.insert("Bed", 3);
    hashmap.insert("Chair", 5);

    // for (k ,v) in &hashmap { 
    //     let value = v.to_owned();
    //     if value == 0 {
    //         println!("Out of stock!");
    //     } else if value == 1 {
    //         println!("A {:?} is left", k.to_lowercase());
    //     } else {
    //         println!("{:?} {:?} is left.", k, v);
    //     }
    // }

    // println!("-------------------------");

    // for hash in &hashmap {
    //     println!("{}", hash.0)
    // }

    let mut total_qty = 0;

    // .iter()을 호출했을때 반환되는 형태는 기본적으로 참조형
    // 따라서 참조('&')를 해줄 필요가 없음
    for (item, qty) in hashmap.iter() {
        total_qty = total_qty + qty;
        let qty_count = if qty == &0 {
            "Out of stock.".to_owned() // 반환값으로 쓰려면 ;가 없어야함
        } else {
            format!("{:?}", qty)
            // println! 과 비슷하지만 메시지를 콘솔에 출력하지 않고 문자열 형태로 반환
            // 재고 수량을 문자로 반환
        };
        println!("item = {:?}, qty = {:?}", item, qty_count);
    }
    println!("-------------------------------");
    println!("total qty = {:?}", total_qty);
    println!("-------------------------------");

    for hash in hashmap.iter() {
        println!("{} {}s is left", hash.1, hash.0)
    }
}