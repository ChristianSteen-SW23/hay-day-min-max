use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufRead};

#[derive(Debug, serde::Serialize)]
struct Item {
    name: String,
    level: u32,
    price: u32,
    time: u32,
    needs: Vec<String>,
    source: String,
}

fn main() {
    match read_file_data("./dataFiles/ItemDataTest.txt".to_owned()) {
        Ok(string_arr) => match convert_to_struct(string_arr) {
            Some(item_arr) => write_json_data("JSONTEST".to_string(), &item_arr),
            None => println!("Error in convert to struct"),
        },
        Err(e) => eprintln!("Error: {}", e),
    }
    println!("Done");
}

fn write_json_data(file_name: String, item_arr: &Vec<Item>) {
    match serde_json::to_string_pretty(item_arr) {
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

fn read_file_data(file_name: String) -> Result<Vec<String>, io::Error> {
    // Read a file line for line
    let file = File::open(&file_name)?; // Open the file
    let reader = io::BufReader::new(file); // Create a buffered reader

    let mut arr = Vec::new();
    for line in reader.lines() {
        let line = line?; // Unwrap the Result to get the line as a string
        arr.push(line);
    }

    Ok(arr)
}

fn convert_to_struct(lines: Vec<String>) -> Option<Vec<Item>> {
    let mut item_arr: Vec<Item> = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        if line == "</th></tr>" || line == "</td></tr>" {
            let name = extract_title(lines.get(i + 2)?.as_str())?;
            let level = extract_level(lines.get(i + 4)?.as_str())?;
            let price = extract_price(lines.get(i + 6)?.as_str())?;
            let time = extract_time(lines.get(i + 8)?.as_str())?;

            let mut j = i + 12;
            let mut needs: Vec<String> = Vec::new();
            loop {
                if let Some(need) = extract_title(lines.get(j)?.as_str()) {
                    needs.push(need);
                    j = j + 1;
                } else {
                    break;
                }
            }

            let source_add_index = if needs.len() == 0 || needs.len() == 1 {
                0
            } else {
                needs.len()
            };
            let source = extract_source(lines.get(i + 14 + source_add_index)?.as_str())?;

            let cur_item: Item = Item {
                name: name,
                level: level,
                price: price,
                time: time,
                needs: needs,
                source: source,
            };
            //println!("{:?}", cur_item);
            item_arr.push(cur_item);
        }
    }
    Some(item_arr)
}

fn extract_title(input: &str) -> Option<String> {
    let re = Regex::new(r#"title="([^"]+)""#).unwrap();

    if let Some(captures) = re.captures(input) {
        return Some(captures[1].to_string());
    }
    None
}

fn extract_level(input: &str) -> Option<u32> {
    let re = Regex::new(r#"<td>(\d+)"#).unwrap();

    if let Some(captures) = re.captures(input) {
        return captures[1].parse().ok();
    }
    None
}

fn extract_price(input: &str) -> Option<u32> {
    extract_level(input)
}

fn extract_source(input: &str) -> Option<String> {
    extract_title(input)
}

fn extract_time(input: &str) -> Option<u32> {
    let re = Regex::new(r#"<td>(?:(\d+) d)?(?:(\d+) h)?(?:(\d+) min)?"#).unwrap();

    if let Some(captures) = re.captures(input) {
        // Extract days, hours, and minutes, defaulting to 0 if not present
        let days = captures
            .get(1)
            .map_or(0, |m| m.as_str().parse::<u32>().unwrap_or(0));
        let hours = captures
            .get(2)
            .map_or(0, |m| m.as_str().parse::<u32>().unwrap_or(0));
        let minutes = captures
            .get(3)
            .map_or(0, |m| m.as_str().parse::<u32>().unwrap_or(0));
        //println!("{} {} {}",days,hours,minutes);
        // Convert to total minutes
        return Some(days * 24 * 60 + hours * 60 + minutes);
    }

    None
}

#[cfg(test)]
mod test_cases {
    use super::*;
    #[test]
    fn test_title() {
        assert_eq!(
            extract_title(r#"<a href="/wiki/Wheat" title="Wheat">Wheat</a>"#).unwrap(),
            "Wheat"
        );
    }

    #[test]
    fn test_title_error() {
        assert!(extract_title(r#"<a href="/wiki/Wheat" title>Wheat</a>"#).is_none());
    }

    #[test]
    fn test_level() {
        assert_eq!(extract_level(r#"<td>3"#).unwrap_or(1), 3);
    }

    #[test]
    fn test_price() {
        assert_eq!(extract_price(r#"<td>3"#).unwrap_or(1), 3);
    }

    #[test]
    fn test_time() {
        assert_eq!(
            extract_time(r#"<td>1 d 20 h 24 min<br />3"#).unwrap_or(1),
            2664
        );
    }

    #[test]
    fn test_time2() {
        assert_eq!(
            extract_time(r#"<td>3 h<br />★★★ 2 h 33 min"#).unwrap_or(1),
            180
        );
    }
    #[test]
    fn test_time3() {
        assert_eq!(extract_time(r#"<td>3 h"#).unwrap_or(1), 180);
    }
}
