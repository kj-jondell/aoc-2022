use std::collections::HashSet;

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

fn get_priority(stream: String) -> u32{
    let unique_run: Vec<char> = stream.chars().collect();
    let first_half = &unique_run[0..unique_run.len()/2].iter().collect::<HashSet<_>>();
    let second_half = &unique_run[unique_run.len()/2..].iter().collect::<HashSet<_>>();
    let mut priority = **first_half.intersection(&second_half).collect::<Vec<_>>()[0] as u32;
    if priority <= 90 {
        priority = priority - 38;
    }
    else {
        priority = priority - 96;
    }
    priority
}

fn get_priority_from_hash_set(first_hash: HashSet<char>, second_hash: HashSet<char>, third_hash: HashSet<char>) -> u32{
//fn get_priority_from_hash_set(hashes: Vec<String>) -> u32{
    //let mut intersection = hashes[0].intersection(&hashes[1]).collect::<HashSet<&char>>();
    //let intersection = hashes[0].chars().collect::<Vec<char>>().iter().collect::<HashSet<_>>();
    //for index in 1..hashes.len()-2 {
    //    let comp = hashes[index].chars().collect::<Vec<char>>().iter().collect::<HashSet<_>>();
    //    let intersection_of_hashes = intersection.intersection(&comp);

    //}
    let mut priority = first_hash.intersection(&second_hash).collect::<HashSet<char>>().intersection(&third_hash);
    println!("{:?}", priority);
    //.collect::<Vec<_>>()[0] as u32;
    let mut priority = 100;
    if priority <= 90 {
        priority = priority - 38;
    }
    else {
        priority = priority - 96;
    }
    priority
}

fn main() {
    let mut total_score = 0;
    let mut hashes: Vec<String> = Vec::new();

    loop {
        match get_input() { //get next_line
            Ok(resp) => {
                if hashes.len() == 3 {
                    total_score += get_priority_from_hash_set(hashes[0].chars().collect(), 
                    hashes[1].chars().collect(), hashes[2].chars().collect());
                    hashes = Vec::new();
                } else {
                    hashes.push(resp.trim().to_string());
                }
            },
            Err(_) => { //break on EOF
                println!("{}", total_score);
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
    fn priorities() {
        let eq1 = get_priority("PmmdzqPrVvPwwTWBwg".to_string());
        let eq2 = get_priority("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".to_string());
        let eq3 = get_priority("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn".to_string());
        let eq4 = get_priority("vJrwpWtwJgWrhcsFMMfFFhFp".to_string());
        assert_eq!(eq1, 42);
        assert_eq!(eq2, 38);
        assert_eq!(eq3, 22);
        assert_eq!(eq4, 16);
        //assert_eq!(eq1+eq2+eq3+eq4, 157);
    }

}
