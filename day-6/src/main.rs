use std::ops::Index;

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

fn get_unique_run(stream: String, nth: usize) -> usize{
    let unique_run: Vec<char> = stream.chars().collect();
    for index in 0..unique_run.len()-nth {
        let part = &mut unique_run[index..index+nth].to_vec();
        part.sort_unstable();
        part.dedup();
        if part.len() == nth {
            return index+nth;
        }
    }
    0
}

fn main() {
    loop {
        match get_input() { //get next_line
            Ok(resp) => {
                println!("{}", get_unique_run(resp, 14));
            },
            Err(_) => { //break on EOF
                break
            }
        }
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn start_of_packet () {
        assert_eq!(get_unique_run("mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string(), 4), 7);
        assert_eq!(get_unique_run("bvwbjplbgvbhsrlpgdmjqwftvncz".to_string(), 4), 5);
        assert_eq!(get_unique_run("nppdvjthqldpwncqszvftbrmjlhg".to_string(), 4), 6);
        assert_eq!(get_unique_run("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string(), 4), 10);
        assert_eq!(get_unique_run("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string(), 4), 11);
    }

    /* mjqjpqmgbljsphdztnvjfqwrcgsmlb: first marker after character 19
bvwbjplbgvbhsrlpgdmjqwftvncz: first marker after character 23
nppdvjthqldpwncqszvftbrmjlhg: first marker after character 23
nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg: first marker after character 29
zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw: first marker after character 26 */
    #[test]
    fn start_of_message () {
        assert_eq!(get_unique_run("bvwbjplbgvbhsrlpgdmjqwftvncz".to_string(), 14), 23);
        assert_eq!(get_unique_run("nppdvjthqldpwncqszvftbrmjlhg".to_string(), 14), 23);
        assert_eq!(get_unique_run("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string(), 14), 29);
        assert_eq!(get_unique_run("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string(), 14), 26);
        assert_eq!(get_unique_run("mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string(), 14), 19);
    }
}