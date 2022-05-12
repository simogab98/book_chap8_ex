use rand::Rng;
use std::collections::HashSet;

const DIM: usize = 100;
const RANGE_START: u8 = 0;
const RANGE_END: u8 = 255;

fn main() {
    println!("The Rust Book - Chapter 8 exercises!");

    let vect = rand_vec(DIM);
    let (median, mode) = median_mode::median_and_mode(&vect);
    println!("Median = {}\nMode = {}", median, mode);

    let arr = [1,1,3,4,5,6,7,7,8,8,8,99,100,100,100,100,100,7,7,7];
    println!("Another Mode: {}", median_mode::mode_calc(&arr));

    println!("'First' converted in Pig Latin -> {}", pig_latin::convert("First"));
    println!("'Apple' converted in Pig Latin -> {}", pig_latin::convert("Apple"));

    println!("");
    employees::interface();
    println!("");
}

fn rand_vec(dim: usize) -> Vec<u8> {
    let mut nums_vect = Vec::new();
    for _ in 0..dim {
        let random = rand::thread_rng().gen_range(RANGE_START..=RANGE_END);
        nums_vect.push(random);
    }
    nums_vect
}

mod employees {

    use std::io;
    use std::collections::HashMap;

    pub fn interface() {
        println!("Interface opened");

        let mut dep_emp: HashMap<String, Vec<String>> = HashMap::new();

        loop {

            menu();

            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read input!");

            if input.is_empty() { break; }

            let input: u8 = input.trim().parse().expect("Error parsing input option!");

            match input {
                1 => record_employee(&mut dep_emp),
                2 => show_deparment_employees(&dep_emp),
                3 => show_all_employees(&dep_emp),
                _ => {
                    println!("Quitting...");
                    break;
                }
            }

        }

        println!("Interface closed.")
    }

    fn menu() {
        println!("");
        println!("--- OPTIONS MENU ---");
        println!("1) Record an employee");
        println!("2) Show employees in a department");
        println!("3) Show all employees by department, sorted alphabetically");
        println!("Insert something else to quit.\n");
        println!("Insert an option: ");

        // io::stdout().flush().unwrap();
        // print!("Insert an option: ");
    }

    fn record_employee(dep_emp: &mut HashMap<String, Vec<String>>) {
        println!("Insert employee info to record in the following format: “Add 'Employee' to 'Department'”");
        
        // print!("Employee info: ");
        println!("Employee info: ");
        let mut employee_new = String::new();
        io::stdin().read_line(&mut employee_new).expect("Error during employee info!");

        let splitted_info: Vec<String> = employee_new.split_whitespace().map(String::from).collect();

        if splitted_info.len() != 4 || splitted_info[0].ne("Add") || splitted_info[2].ne("to") {
            println!("Wrong input format :(");
            return;
        }

        let employee = &splitted_info[1];
        let department = &splitted_info[3];

        let department_employees = dep_emp.entry(department.to_string()).or_insert_with(Vec::new);
        (*department_employees).push(employee.to_string());

        println!("Employee '{}' added to '{}' department", employee, department);
    }

    fn show_deparment_employees(dep_emp: &HashMap<String, Vec<String>>) {
        // print!("Insert a deparment name to show employees of it: ");
        println!("Insert a deparment name to show employees of it: ");
        let mut department = String::new();
        io::stdin().read_line(&mut department).expect("Error reading deparment!");
        department.pop(); // Remove newline char

        match dep_emp.get(&department as &str) {
            Some(employees) => println!("Employees in '{}' department: {:?}", department, employees),
            None => println!("Department '{}' does not exist", department)
        }
    }

    fn show_all_employees(dep_emp: &HashMap<String, Vec<String>>) {
        println!("Show all employees, by deparment, alphabetically sorted: \n");
        for (department, employees) in dep_emp {
            println!("Department: {}", department);

            let mut emp_sorted = employees.clone();
            emp_sorted.sort();
            println!("Employees (alphabetically sorted): {:?}", emp_sorted);
            println!("");
        }
        println!("End of epmloyees.");
    }

}

mod pig_latin {

    pub fn convert(string: &str) -> String {
        let mut chars_stream = string.chars();
        let first_char = chars_stream.next().unwrap();

        if is_vowel(first_char) {
            return string.to_string() + "-hay";
        }

        let mut result = String::new();
        for c in chars_stream {
            result.push(c);
        }

        result.push_str(&format!("-{}ay", first_char));

        result
    }

    fn is_vowel(letter: char) -> bool {
        let vowels = super::HashSet::from(["A", "E", "I", "O", "U"]);
        let upp_case = letter.to_uppercase().to_string();
        vowels.contains(upp_case.as_str())
    }

}

mod median_mode {

    use std::collections::HashMap;
    use num_integer;

    pub fn median_and_mode(sorted_arr: &[u8]) -> (f32, u8) {
        (median(sorted_arr), mode_calc(sorted_arr))
    }
    
    fn median(sorted_arr: &[u8]) -> f32 {
        let (middle, remainder) = num_integer::div_rem(sorted_arr.len(), 2);
        if remainder == 0 {
            let sum: u16 = sorted_arr[middle] as u16 + sorted_arr[middle - 1] as u16;
            return sum as f32 / 2 as f32;
        }
            
        sorted_arr[middle] as f32
    }
    
    pub fn mode_calc(arr: &[u8]) -> u8 {
        let mut mode = arr[0];
        let mut mod_occ = 1;
    
        let mut freq = HashMap::new();
        for &el in arr {
            let count = freq.entry(el).or_insert(0);
            *count += 1;
    
            if *count > mod_occ {
                mod_occ = *count;
                mode = el;
            }
        }
    
        return mode;
    }

}



