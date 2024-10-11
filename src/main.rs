
fn find_first_index(s: String) -> Option<i32>{
    for(index,character) in s.chars().enumerate(){
        if character == 'a'{
            return Some(index as i32);
        }
    }
    return None;
}

fn main() {
    let my_string=String::from("anurag,World!");
    match find_first_index(my_string){
        Some(index) => println!("Found at index: {}",index),
        None => println!("The letter 'a' is not found in the string")
    }
}
