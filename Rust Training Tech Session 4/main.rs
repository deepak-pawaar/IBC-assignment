use std::io;

#[derive(Debug)]
struct Student {
    name: String,
    email: String,
    phno: String,
    id: u32,
}

fn main() {
    let mut students = Vec::new();

    students.push(Student {
        name: "Alice".to_string(),
        email: "alice@example.com".to_string(),
        phno: "1234567890".to_string(),
        id: 1,
    });
    students.push(Student {
        name: "Bob".to_string(),
        email: "bob@example.com".to_string(),
        phno: "0987654321".to_string(),
        id: 2,
    });
    students.push(Student {
        name: "Charlie".to_string(),
        email: "charlie@example.com".to_string(),
        phno: "1112223333".to_string(),
        id: 3,
    });
    students.push(Student {
        name: "Dave".to_string(),
        email: "dave@example.com".to_string(),
        phno: "4445556666".to_string(),
        id: 4,
    });
    students.push(Student {
        name: "Eve".to_string(),
        email: "eve@example.com".to_string(),
        phno: "7778889999".to_string(),
        id: 5,
    });

    loop {
        println!("Enter the index of the student you want to view (or 'q' to quit):");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        if input.trim() == "q" {
            break;
        }

        let index = match input.trim().parse::<usize>() {
            Ok(i) => i,
            Err(_) => {
                println!("Invalid input, please enter a valid index or 'q' to quit.");
                continue;
            }
        };

        match students.get(index) {
            Some(student) => println!("{:#?}", student),
            None => println!("No student found at index {}", index),
        }
    }
}
