pub fn greet_world_enumerate() {
    let japan = "日本";
    let german = "Deutschland";
    let regions = [japan, german];
    for (i, region) in regions.iter().enumerate() {
        println!("{}: {}", i, region);
    }
}

pub fn greet_world() {
    let japan = "日本";
    let german = "Deutschland";
    let regions = [japan, german];
    for region in regions.iter() {
        println!("{}", &region)
    }
}

pub fn make_csv_great() {
    let penguin_data = "
    common_name, length (cm)
    Little penguin, 33
    Yellow-eyed penguin, 65
    Fiordland penguin, 60
    Invalid,data
    ";

    let records = penguin_data.lines();

    for (i, record) in records.enumerate() {
        if i == 0 || record.trim().len() == 0 {
            continue;
        }
        let fields: Vec<_> = record.split(',').map(|field| field.trim()).collect();
        if cfg!(debug_assertions) {
            eprintln!("debug: {:?} -> {:?}", record, fields);
        }
        let name = fields[0];
        if let Ok(length) = fields[1].parse::<f32>() {
            println!("{}, {}cm", name, length);
        }
    }
}
