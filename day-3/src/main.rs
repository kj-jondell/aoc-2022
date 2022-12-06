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

fn main() {
    loop {
        match get_input() { //get next_line
            Ok(resp) => {
                //total_score += get_score_day_1();
            },
            Err(_) => { //break on EOF
                println!("{}", "hello");
                break
            }
        }
    }
}
