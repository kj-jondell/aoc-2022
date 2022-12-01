use std::io;

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
    let amt_to_sum: u8 = 3; //amt of top values
    let mut sum: u32 = 0; // sum of calories
    let mut top = vec![0; amt_to_sum]; 

    loop {
        match get_input() { //get next_line
            Ok(resp) => {
                if resp == "\n" {
                    if sum>top[0] {
                        top.push(sum);
                        top.sort();
                        top.remove(0);
                    }
                    sum = 0;
                }
                else {
                    sum += resp.trim().parse::<u32>().unwrap();
                }
            },
            Err(resp) => { //break on EOF
                let max_value: u32 = top.iter().sum();
                println!("{:?}", max_value); //print sum of top n values
                break
            }
        }
    }
}
