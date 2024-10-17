use csv::Reader;
fn main() {
    let mut csv = Reader::from_path("./files.csv").expect("Failed to open CSV file");
    let records = csv.records();
    //place holder path
    let _steam_path = "C:\\Program Files x86\\Steam\\steamapps\\common\\workshop\\content\\107410";
    for record in records {
        let string = record.expect("Something went wrong");
        let url = string.get(1).expect("Unable to extract URL!");
        let url_vec: Vec<&str> = url.split("=").collect();
        let id = url_vec.get(1).expect("UNable to extract ID!");
        println!("{:?}", id)
    }
}
