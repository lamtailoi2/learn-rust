use std::io::{self, Read, Write};
#[derive(Debug)]
struct Student {
    id: String,
    name: String,
    yob: i16,
}

trait StudentTrait {
    fn getInfo(&self) -> String;
    fn getName(&self) -> String;
}

impl StudentTrait for Student {
    fn getInfo(&self) -> String {
        format!("ID: {} |Name: {} |Yob {}", self.id, self.name, self.yob)
    }
    fn getName(&self) -> String {
        format!("{}", self.name)
    }
}

fn main() {
    let mut student_list: Vec<Student> = Vec::new();
    loop {
        println!("\n=== CRUD Console App ===");
        println!("1. Create Student");
        println!("2. Delete Student");
        println!("3. Update Student");
        println!("4. Display Student List");
        println!("0. Quit");
        io::stdout().flush().unwrap();
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Invalid");
        match choice.trim() {
            "1" => {
                print!("Enter Student ID: ");
                io::stdout().flush().unwrap();
                let mut id = String::new();
                io::stdin().read_line(&mut id).expect("Invalid");

                print!("Enter Student Name: ");
                io::stdout().flush().unwrap();
                let mut name = String::new();
                io::stdin().read_line(&mut name).expect("Invalid");

                print!("Enter Student Yob: ");
                io::stdout().flush().unwrap();
                let mut yob = String::new();
                io::stdin().read_line(&mut yob).expect("Invalid");

                let new_student = Student {
                    id: id.trim().to_string(),
                    name: name.trim().to_string(),
                    yob: yob.trim().parse().expect("Invalid"),
                };
                student_list.push(new_student);
            }

            "2" => {
                if student_list.is_empty() {
                    print!("List is empty");
                    break;
                }
                print!("Enter Student ID: ");
                io::stdout().flush().unwrap();
                let mut delete_ID = String::new();
                io::stdin().read_line(&mut delete_ID).expect("Error");
                let delete_id_normalized = delete_ID.trim().to_lowercase();
                if let Some(index) = student_list
                    .iter()
                    .position(|student| student.id.to_lowercase() == delete_id_normalized)
                {
                    println!("Deleted Student: {}", student_list[index].getInfo());
                    student_list.remove(index);
                    print!("Deleted successful!");
                } else {
                    print!("Student not found!")
                }
            }
            "3" => {
                if student_list.is_empty() {
                    print!("List is empty");
                    break;
                }
                print!("Enter Student ID: ");
                io::stdout().flush().unwrap();
                let mut update_ID = String::new();
                io::stdin().read_line(&mut update_ID).expect("Error");
                let mut update_ID_normalized = update_ID.trim().to_lowercase();
                if let Some(index) = student_list
                    .iter()
                    .position(|student| student.id.to_lowercase() == update_ID_normalized)
                {
                    print!("Enter Student Name: ");
                    io::stdout().flush().unwrap();
                    let mut name = String::new();
                    io::stdin().read_line(&mut name).expect("Invalid");
                    student_list[index].name = name.trim().to_string();

                    print!("Enter Student Yob: ");
                    io::stdout().flush().unwrap();
                    let mut yob = String::new();
                    io::stdin().read_line(&mut yob).expect("Invalid");
                    student_list[index].yob = yob.trim().parse().expect("Invalid");

                    print!("{:?}", student_list[index].getInfo());
                }
            }
            "4" => {
                if student_list.is_empty() {
                    print!("List is empty");
                } else {
                    for student in &student_list {
                        print!(
                            "ID: {}|Name: {}|Yob: {}",
                            student.id, student.name, student.yob
                        );
                    }
                }
            }
            "0" => break,
            _ => println!("Invalid choice. Please try again."),
        }
    }
}
