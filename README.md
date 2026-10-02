# Overview

The Student Grade Manager is a command-line program written in Rust that allows a user to enter a student's name and multiple grades. The program calculates the total and average of the grades and then determines whether the student passed or needs improvement.

I created this software to improve my understanding of Rust and to learn how Rust handles variables, functions, loops, conditionals, data structures, references, and borrowing. One of the main things I wanted to understand better was how Rust manages ownership when data is passed between functions.

The program also validates the user's input. Grades must be between 0 and 100, and the user can enter as many grades as needed before typing "done" to finish. The program then displays a complete grade report.

[Software Demo Video](https://www.youtube.com/watch?v=vyKbOcOdPSc)

# Development Environment

I developed this project using Visual Studio Code on Windows. I used the Rust Analyzer extension to provide Rust language support in Visual Studio Code and used Cargo from the terminal to create, build, and run the project.

The programming language used for this project is Rust. The program mainly uses Rust's standard library, including std::io for reading input from the user. No external libraries or crates were required.

The project uses a Vec<f64> to store multiple grades. I also used references and slices such as &[f64] when passing the grades to functions so that the functions could access the data without taking ownership of it.

# Useful Websites

- [Rust Official Website](https://www.rust-lang.org/)
- [The Rust Programming Language Book](https://doc.rust-lang.org/book/)
- [Rust Standard Library](https://doc.rust-lang.org/std/)
- [Rust Documentation](https://doc.rust-lang.org/)


# Future Work

- Add the ability to enter student names and save multiple students in the same program.
- Add different subjects so that each grade can be connected to a specific subject.
- Add the ability to save and load grade records from a file.
- Improve the grade report by including letter grades such as A, B, C, D, and F.
- Add more detailed input validation and error handling.
