use std::io::{self, Write};

const PASSING_GRADE: f64 = 50.0;

fn main() {
    println!("=================================");
    println!("      STUDENT GRADE MANAGER");
    println!("=================================");
    println!("This program calculates a student's");
    println!("total, average, and final result.\n");

    // Get the student's name from the user.
    let student_name = get_student_name();

    // Get the grades and store them in a vector.
    let grades = get_grades();

    // Calculate the student's total and average.
    let total = calculate_total(&grades);
    let average = calculate_average(&grades);

    // Display the complete grade report.
    display_results(&student_name, &grades, total, average);
}

// Gets the student's name from the user.
// A loop is used so that the program can ask again if the input is empty.
fn get_student_name() -> String {
    loop {
        print!("Enter the student's name: ");
        io::stdout().flush().unwrap();

        let mut name = String::new();

        io::stdin()
            .read_line(&mut name)
            .expect("Failed to read input.");

        let name = name.trim();

        if !name.is_empty() {
            return name.to_string();
        }

        println!("Please enter a valid name.");
    }
}

// Gets multiple grades from the user and stores them in a Vec.
// The loop continues until the user types "done".
fn get_grades() -> Vec<f64> {
    let mut grades: Vec<f64> = Vec::new();

    println!("\nEnter the student's grades.");
    println!("Grades must be between 0 and 100.");
    println!("Type 'done' when you have finished.\n");

    loop {
        print!("Enter grade: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input.");

        let input = input.trim();

        // Allow the user to finish entering grades.
        if input.eq_ignore_ascii_case("done") {
            if grades.is_empty() {
                println!("Please enter at least one grade.");
                continue;
            }

            break;
        }

        // Try to convert the user's input into a decimal number.
        match input.parse::<f64>() {
            Ok(grade) if (0.0..=100.0).contains(&grade) => {
                grades.push(grade);
                println!("Grade added successfully: {:.2}", grade);
            }

            Ok(_) => {
                println!("Grade must be between 0 and 100.");
            }

            Err(_) => {
                println!("Invalid input. Enter a number or type 'done'.");
            }
        }
    }

    grades
}

// Calculates the total of all grades.
// A reference is used so that ownership of the grades remains with the caller.
fn calculate_total(grades: &[f64]) -> f64 {
    let mut total = 0.0;

    for grade in grades {
        total += grade;
    }

    total
}

// Calculates the average of all grades.
// The grades are borrowed through a slice instead of being moved into the function.
fn calculate_average(grades: &[f64]) -> f64 {
    let total = calculate_total(grades);
    let number_of_grades = grades.len() as f64;

    total / number_of_grades
}

// Determines whether the student passed or needs improvement.
fn determine_result(average: f64) -> &'static str {
    if average >= PASSING_GRADE {
        "PASS"
    } else {
        "NEEDS IMPROVEMENT"
    }
}

// Displays all the student's grades and the calculated results.
// References are used for the student name and grades.
fn display_results(
    student_name: &str,
    grades: &[f64],
    total: f64,
    average: f64,
) {
    let result = determine_result(average);

    println!("\n=================================");
    println!("          GRADE REPORT");
    println!("=================================");

    println!("Student: {}", student_name);

    println!("\nGrades:");

    for (index, grade) in grades.iter().enumerate() {
        println!("Subject {}: {:.2}", index + 1, grade);
    }

    println!("\nNumber of grades: {}", grades.len());
    println!("Total: {:.2}", total);
    println!("Average: {:.2}", average);
    println!("Passing grade: {:.2}", PASSING_GRADE);
    println!("Result: {}", result);

    if result == "PASS" {
        println!("The student has passed.");
    } else {
        println!("The student needs to improve.");
    }

    println!("=================================");
}
