use csv::Reader;
use std::env;
use std::fs::{self, remove_dir_all};
fn main() {
    let args: Vec<String> = env::args().collect();
    let csv_path = &args[2];
    let mut csv = Reader::from_path(csv_path).expect("Failed to open CSV file");
    let records = csv.records();
    //place holder path
    let steam_path = &args[1];
    let mut id_vector = Vec::new();
    let mut dir_vector = Vec::new();
    for record in records {
        let string = record.expect("Something went wrong");
        let url = string.get(1).expect("Unable to extract URL!");
        let url_vec: Vec<&str> = url.split("=").collect();
        let id = url_vec.get(1).expect("Unable to extract ID!").to_string();
        id_vector.push(id);
    }

    let mod_folders = fs::read_dir(steam_path).expect("Couldn't read mod folders!");
    for dir in mod_folders {
        let directory = dir.expect("Unable to read dir");

        dir_vector.push(directory);
    }
    let mut fs_ids: Vec<String> = Vec::new();
    for dir in dir_vector.into_iter() {
        let id = dir.file_name().into_string().expect("Error occured");
        fs_ids.push(id);
    }
    let mut extra_ids: Vec<String> = Vec::new();
    for id in fs_ids {
        //  println!("{:?}", id);
        if !id_vector.contains(&id) {
            println!("Not subscribed but present on disk - {}", id);
            extra_ids.push(id);
        }
    }
    for id in extra_ids {
        let path = format!("{}/{}", steam_path, id);
        println!("Removing - {}", id);
        remove_dir_all(path).expect("Unable to delete directory!")
    }
}
