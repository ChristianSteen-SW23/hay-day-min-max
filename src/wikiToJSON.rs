struct Item {
    name: String,
    level: u32,
    price: u32,
    time: u32,
    needs: String,
    source: String,
}

fn main() {
    println!("Hello, world!");
}



fn readFileData(fileName: String) -> Result<BothList, io::Error> {
    // Read a file line for line
    let file = File::open(&fileName)?; // Open the file
    let reader = io::BufReader::new(file); // Create a buffered reader

    

    let mut arr1 = Vec::new();
    let mut arr2 = Vec::new();

    for line in reader.lines() {
        let line = line?; // Unwrap the Result to get the line as a string
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 2 {
            arr1.push(parts[0].parse::<i32>().unwrap());
            arr2.push(parts[1].parse::<i32>().unwrap());
        }
    }

    Ok(BothList { arr1, arr2 })
}