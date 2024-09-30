use std::io;

struct Coco {
    name: String, 
    height: i32, // field
    width: i32,
    thickness: i32,
}

fn main() {
    let mut coco1 = Coco { // instance
        name: "WhiteCoco".to_string(),
        height: 10,
        width: 25,
        thickness: 1,
    };
    println!("The name of the Coco is {} with a thickness of {}.",
    coco1.name,
    coco1.thickness);

    let mut area = coco1.height * coco1.width;
    println!("Area of {} = {}.", coco1.name, area);

    coco1.height = 15;
    area = coco1.height * coco1.width;
    println!("New area of {} = {}.", coco1.name, area);


    // exit
    println!("\nPRESS ENTER TO EXIT");
    let _ = io::stdin().read_line(&mut String::new());
}