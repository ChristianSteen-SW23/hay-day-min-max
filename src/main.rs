use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;

struct Item {
    name: String,
    level: u32,
    price: u32,
    time: u32,
    needs: Vec<String>,
    source: String,
}

fn main() {
    println!("Hello, world!");
    match read_file_data("./dataFiles/ItemData.txt".to_owned()) {
        Ok(string_arr) => {
            cunvert_to_struct(string_arr);

        }
        Err(e) => eprintln!("Error: {}", e),
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

fn cunvert_to_struct(lines: Vec<String>) -> Option<Vec<Item>> {
    let mut item_arr = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        if line == "</th></tr>" || line == "</td></tr>" {
            println!("-----------------------------------");
            //println!("Line {}: {}", i, line);
            //println!("{}",lines.get(i + 2)?.as_str());

            let name = extract_title(lines.get(i + 2)?.as_str())?;
            println!("Name: {}", name);

            let level = extract_level(lines.get(i + 4)?.as_str())?;
            println!("Level: {}", level);

            let price = extract_price(lines.get(i + 6)?.as_str())?;
            println!("Price: {}", price);

            // TODO: Time

            let mut j = i + 12;
            let mut needs: Vec<String> = Vec::new();
            loop{
                if let Some(need) = extract_title(lines.get(j)?.as_str()){
                    needs.push(need);
                    j = j + 1;
                }else{
                    break
                }
            }
            println!("Needs: {:?}",needs);

            let source_add_index = if needs.len() == 0 || needs.len() == 1 { 0 } else { needs.len() };
            let source = extract_source(lines.get(i + 14 + source_add_index)?.as_str())?;
            println!("Source: {}", source);
            // TODO: Make struct and add it

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


#[cfg(test)]
mod test_cases {
    use super::*;
    #[test]
    fn test_title() {
        assert_eq!(extract_title(r#"<a href="/wiki/Wheat" title="Wheat">Wheat</a>"#).unwrap(), "Wheat");
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
}