use serde::Deserialize;
use std::{collections::HashMap, fs, process};
use std::fs::File;
use std::io::Write;


#[derive(Debug, Deserialize, Clone,serde::Serialize)]
struct Item {
    name: String,
    level: u32,
    price: u32,
    time: u32,
    needs: Vec<String>,
    source: String,
    need_value: Option<u32>,
    time_per_value: Option<f32>,
}

impl Item {
    fn new(name: String, price: u32, needs: Vec<String>, time: u32) -> Self {
        Item {
            name, price, needs, time,
            source: "IDK".to_string(),
            need_value: None,
            level: 0,
            time_per_value: None,
        }   
    }
}

fn main() {
    println!("Hello, world!");
    let content = file_to_string("Hay day items JSON".to_string());
    let items: Vec<Item> = serde_json::from_str(&content).expect("JSON was not well-formatted");

    let mut item_map = cal_added_value(items);
    item_map = cal_time_per_value(item_map);
    
    println!("{:#?}", item_map);
    write_json_data("JSON with data analyses".to_string(), item_map);
    println!("Done");
}

fn write_json_data(file_name: String, item_map: HashMap<String,Item>) {
    let item_arr: Vec<Item>  = item_map.into_values().collect();
    match serde_json::to_string_pretty(&item_arr) {
        Ok(item_json) => {
            match File::create(format!("./dataFiles/{}{}", file_name, ".txt")) {
                Ok(mut file) => {
                    match file.write_all(item_json.as_bytes()) {
                        Ok(_) => {},
                        Err(e) => eprint!("{}", e),
                    }
                }
                Err(e) => eprint!("{}", e),
            }
        }
        Err(e) => eprint!("{}", e),
    }
}

fn cal_time_per_value (mut item_map: HashMap<String,Item>) -> HashMap<String,Item> {
    item_map.iter_mut().filter(|(_,value)| value.need_value.is_some() && value.time != 0).for_each(|(_,value)| value.time_per_value = Some(((value.price as f32) - (value.need_value.unwrap() as f32))/( (value.time as f32) / 60.0 )));
    item_map
}

fn cal_added_value(items: Vec<Item>) -> HashMap<String,Item> {
    let mut item_map: HashMap<String,Item> = HashMap::new();
    item_map = items.into_iter().map(|item| (item.name.clone(), item)).collect();
    let item_map_clone = item_map.clone();
    
    item_map.iter_mut().for_each(|(_, value)| value.need_value = Some(value.needs.iter().filter_map(|need| item_map_clone.get(need).map(|item| item.price)).sum::<u32>()));

    println!("{:#?}",item_map);
    
    item_map
}

fn cal_added_value_2(items: Vec<Item>) -> HashMap<String,Item> {
    let item_map: HashMap<String, Item> = items.into_iter().map(|item| (item.name.clone(), item)).collect();
    item_map.iter().fold(HashMap::new(), |mut acc, (key, value)| {
        let need_value = value.needs.iter().filter_map(|need| item_map.get(need).map(|item| item.price)).sum::<u32>();
        let mut new_item: Item = value.clone();
        new_item.need_value = Some(need_value); 
        acc.insert(key.clone(), new_item);
        acc
    })
}

// fn file_to_string_2(file_name: String) ->Result<String, io::Error> {
//     Ok(fs::read_to_string(&file_name)?)
// }

fn file_to_string(file_name: String) -> String {
    match fs::read_to_string(format!("./dataFiles/{}.txt", &file_name)) {
        Ok(content) => content,
        Err(e) => {
            println!("Error reading file: {}", e);
            process::exit(1)
        }
    }
}

#[cfg(test)]
mod test_cases {
    use super::*;
    #[test]
    fn test_cal_added_value() {
        // Arrange
        let s1 = Item::new("1".to_string(), 1, vec![],0);
        let s2 = Item::new("2".to_string(), 9, vec![],0); 
        let s3 = Item::new("3".to_string(), 10, vec!["1".to_string(),"2".to_string()],0);
        let test_vec = vec![s1,s2,s3];
        
        // Act
        let test_map = cal_added_value(test_vec);

        // Assert
        assert_eq!(test_map.get("1").unwrap().need_value.unwrap_or_default(), 0);
        assert_eq!(test_map.get("2").unwrap().need_value.unwrap_or_default(), 0);
        assert_eq!(test_map.get("3").unwrap().need_value.unwrap(), 10);
    }
    #[test]
    fn test_cal_added_value2() {
        // Arrange
        let s1 = Item::new("1".to_string(), 1, vec![],0);
        let s2 = Item::new("2".to_string(), 9, vec![],0); 
        let s3 = Item::new("3".to_string(), 10, vec!["1".to_string(),"2".to_string()],0);
        let test_vec = vec![s1,s2,s3];
        
        // Act
        let test_map = cal_added_value_2(test_vec);

        // Assert
        assert_eq!(test_map.get("1").unwrap().need_value.unwrap_or_default(), 0);
        assert_eq!(test_map.get("2").unwrap().need_value.unwrap_or_default(), 0);
        assert_eq!(test_map.get("3").unwrap().need_value.unwrap(), 10);
    }

    #[test]
    fn test_cal_time_per_value_withtime() {
        // Arrange
        let mut s1 = Item::new("1".to_string(), 120, vec![],60);
        s1.need_value = Some(60);
        let mut test_map: HashMap<String,Item> = HashMap::new();
        test_map.insert(s1.name.clone(), s1.clone());

        // Act
        test_map = cal_time_per_value(test_map);

        // Assert
        assert_eq!(test_map.get("1").unwrap().time_per_value.unwrap_or_default(), 60.0);
    }
    #[test]
    fn test_cal_time_per_value_withouttime() {
        // Arrange
        let mut s1 = Item::new("1".to_string(), 120, vec![], 0);
        s1.need_value = Some(60);
        let mut test_map: HashMap<String,Item> = HashMap::new();
        test_map.insert(s1.name.clone(), s1.clone());

        // Act
        test_map = cal_time_per_value(test_map);

        // Assert
        assert_eq!(test_map.get("1").unwrap().time_per_value.unwrap_or_default(), 0.0);
    }
    #[test]
    fn test_cal_time_per_value_withoutneeded() {
        // Arrange
        let s1 = Item::new("1".to_string(), 120, vec![], 0);
        let mut test_map: HashMap<String,Item> = HashMap::new();
        test_map.insert(s1.name.clone(), s1.clone());

        // Act
        test_map = cal_time_per_value(test_map);

        // Assert
        assert_eq!(test_map.get("1").unwrap().time_per_value.unwrap_or_default(), 0.0);
    }
}
