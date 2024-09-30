use std::io;

struct Coco {
    name: String,
    height: i32,
    width: i32,
    thickness: i32,
}

fn main() {
    let _choco1 = Coco {
        name: "WhiteCoco".to_string(),
        height: 10,
        width: 30,
        thickness: 1,
    };
    println!("The name of the Coco is {} with thickness of {}.",
    _choco1.name,
    _choco1.thickness);

    println!("\nPRESS ENTER TO EXIT");
    let _ = io::stdin().read_line(&mut String::new());
}