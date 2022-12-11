/*
 Generic function to read values from stdin. Pipe file with e.g. 'cat file|cargo run'
 TODO move to other module
 */
fn get_input() -> Result<String, String> { 
    let mut buffer = String::new();
    let stdin = std::io::stdin();
    let bytes = stdin.read_line(&mut buffer).unwrap();
    if bytes == 0 {
        return Err("EOF".to_string())
    }
    Ok(buffer)
}

fn contains_all(string: String) -> bool {
    let out = string.trim().split(",").collect::<Vec<&str>>();
    let first_pair = out[0].split("-").collect::<HashSet<_>>();
    all_pairs.append(&mut out[1].split("-").collect::<Vec<_>>());
    let mut 
    all_pairs[0]<=all_pairs[2] && all_pairs[1]>=all_pairs[3]
}

fn main() {
    let mut tot_count = 0;
    loop {
        match get_input() { //get next_line
            Ok(resp) => {
                if contains_all(resp) {
                    tot_count += 1;
                }
            },
            Err(_) => { //break on EOF
                println!("{}", tot_count);
                break
            }
        }
    }
}