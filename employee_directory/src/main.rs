use std::io;
use std::collections::HashMap;

fn get_input(output: &str) -> String {
    println!("{}", output);
    let mut ret: String = String::new();
    io::stdin()
        .read_line(&mut ret)
        .expect("Failed to read line!");

//    let ret: f64 = ret.trim().parse().expect("Couldn't convert input to float!");
    ret
}

fn add_to_directory(directory: &mut HashMap<String, Vec<String>>) {
    println!("Time to add to the employee directory!");

    loop {
        println!("Please input the employees department and name in the format (department: name)");
        let input = get_input("or you can input q to exit");
        match input.trim() {
            "q" => {
                println!("Finished inputting employees!");
                println!("Returning to main menu");
                return;
            },
            other => {
                let parts = other.split(": ").collect::<Vec<&str>>();
                if parts.len() != 2 {
                    println!("Unable to parse your answer: {}", other);
                    continue;
                }
                let department = parts[0].to_owned();
                let name = parts[1].to_owned();
                let employee_list = directory.entry(department).or_insert(Vec::new());
                employee_list.push(name);
            }
        }
    }
}

fn print_employees(directory: &mut HashMap<String, Vec<String>>) {
    println!("Now we're going to print the employees. Please enter the \
              department name you'd like to print or ALL for all of \
              them. If you want to quit, please enter q\
              ");
    
    let answer = get_input("Which Department?");
    match answer.trim() {
        "q" => return,
        "dump" => {
            println!("Dumping hash: ");
            println!("{:?}", directory);
        },
        "All" | "all" | "ALL" => {
            for (department, employees) in directory {
                print_department(department, employees);
            }
        },
        other => {
            if let Some(employees) = directory.get_mut(other) {
                print_department(other, employees);
            } else {
                println!("Department not filled out yet. Please input employees first!");
            }
        },
    }
}

fn print_department(name: &str, employees: &mut Vec<String>) {
    println!("Printing all employees for department: {}", name);
    employees.sort_unstable();
    for employee in employees {
        println!("{}",employee);
    }
}

fn main() {
    println!("Welcome to Thomerica Industries bookkeeping service!");

    let mut directory: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        println!("Main Menu: ");
        println!("  1. Add employees to directory");
        println!("  2. Print employees in a department");
        println!("q will exit");
        println!("");
        let answer = get_input("Please enter your selection");

        match answer.trim() {
            "1" => add_to_directory(&mut directory),
            "2" => print_employees(&mut directory),
            "q" => {
                println!("Thank you for using the Thomerica Industries management system!");
                break;
            },
            other => println!("{} is not a valid option. Please input again.", other),
        };

    }
}
