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

fn get_score_day_1(resp: String) -> usize {
    let input_array = resp.split_whitespace().collect::<Vec<&str>>();
    let mut score: usize = 0;

    // let vec = ["Rock", "Paper", "Scissor"];
    let index_a = ["A", "B", "C"].iter().position(|&r| r == input_array[0]).unwrap();
    let index_b = ["X", "Y", "Z"].iter().position(|&r| r == input_array[1]).unwrap();

    if index_a == index_b {
        score += 3;
    }
    else if (index_a+1)%3 == index_b {
        score += 6;
    }
    else {
        score += 0;
    }

    score += index_b + 1;

    score
}

fn get_score_day_2 (resp: String) -> usize {
    let input_array = resp.split_whitespace().collect::<Vec<&str>>();
    let mut score: usize = 0;

    // let vec = ["Rock", "Paper", "Scissor"];
    let index_a = ["A", "B", "C"].iter().position(|&r| r == input_array[0]).unwrap();
    let index_b = ["X", "Y", "Z"].iter().position(|&r| r == input_array[1]).unwrap();

    score += match input_array[1] {
        "X" => (index_a + 2) % 3,
        "Y" => index_a,
        "Z" => (index_a + 1) % 3,
        &_ => 0,
    } + 1;

    score += index_b * 3;

    //println!("{}", score);

    score
}

fn main() {
    let mut total_score: usize = 0;
    loop {
        match get_input() { //get next_line
            Ok(resp) => {
                //total_score += get_score_day_1();
                total_score += get_score_day_2(resp);
            },
            Err(_) => { //break on EOF
                println!("{}", total_score);
                break
            }
        }
    }
}