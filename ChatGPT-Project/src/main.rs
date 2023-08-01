use std::io::{self, Write};
use rand::Rng;
//enumerator for our math operations
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
    TeachScience,
    None,
}

//structure for our math operations
struct MathProblem {
    num1: f64,
    num2: f64,
    operation: Operation,
}

//method for the structure
impl MathProblem {
    fn new() -> MathProblem {
        MathProblem {
            num1: 0.0,
            num2: 0.0,
            operation: Operation::None,
        }
    }

    fn set_numbers(&mut self, num1: f64, num2: f64) {
        self.num1 = num1;
        self.num2 = num2;
    }

    fn set_operation(&mut self, op: Operation) {
        self.operation = op;
    }

    fn solve(&self) -> Option<f64> {
        match self.operation {
            Operation::Add => Some(self.num1 + self.num2),
            Operation::Subtract => Some(self.num1 - self.num2),
            Operation::Multiply => Some(self.num1 * self.num2),
            Operation::Divide => {
                if self.num2 != 0.0 {
                    Some(self.num1 / self.num2)
                } else {
                    None
                }
            }
            Operation::TeachScience => None,
            Operation::None => None,
        }
    }


    fn explanation(&self) -> String {
        match self.operation {
            Operation::Add => "Addition in math is a process of combining two or more numbers.
        Addends are the numbers being added, and the result or the final answer we get after the process is called the sum.
        It is one of the essential mathematical functions we use in our everyday activities. There are many situations in which we add numbers.
        One of the most common everyday uses for adding numbers is when we work with time or money—for example, adding up bills and receipts.
        The addition has an infinite number of applications in our day-to-day life.
        We use addition while cooking food, while calculating bills at supermarkets, while calculating distances, and much more.".to_string(),

            Operation::Subtract => "Subtraction is simply the process of taking one quantity and removing part of it to work out what is left.
        It is the opposite of addition, because we are reducing the value of the number rather than increasing it by adding more numbers.
        You can subtract more than one number away from each other, but this usually involves the use of parenthesis and an understanding of number facts like number order.
        It’s a great idea to introduce subtraction with some examples.".to_string(),

            Operation::Multiply => "Multiplication is an operation that represents the basic idea of repeated addition of the same number.
        The numbers that are multiplied are called the factors and the result that is obtained after the multiplication of two or more numbers is known as the product of those numbers.
        Multiplication is used to simplify the task of repeated addition of the same number.".to_string(),

            Operation::Divide => "Division is the opposite of multiplication.
        If 3 groups of 4 make 12 in multiplication, 12 divided into 3 equal groups give 4 in each group in division.
        The main goal of dividing is to see how many equal groups are formed or how many are in each group when sharing fairly.".to_string(),

            Operation::TeachScience => "Science is awesome! Let me tell you a science fact:".to_string(),

            Operation::None => "I'm sorry, I don't understand the operation.".to_string(),
        }
    }
}



fn parse_operation(input: &str) -> Option<Operation> {
    match input.trim().to_lowercase().as_str() {
        "add" => Some(Operation::Add),
        "subtract" => Some(Operation::Subtract),
        "multiply" => Some(Operation::Multiply),
        "divide" => Some(Operation::Divide),
        "teach science" => Some(Operation::TeachScience),
        _ => None,
    }
}


fn teach_science() -> String {
    // Simple science facts or explanations
    let science_facts = [
        "The Earth orbits around the Sun.",
        "Water is made up of two hydrogen atoms and one oxygen atom (H2O).",
        "The force that pulls objects towards the center of the Earth is called gravity.",
        "Photosynthesis is the process by which plants convert sunlight into energy.",
        "The atomic number of carbon is 6, and its symbol is C.",
    ];
    let idx = rand::thread_rng().gen_range(0..science_facts.len());
    science_facts[idx].to_string() //convert &str to String
}


fn main() {
    println!("Kintel: Hello! I am a Rust-based chatbot. You can start chatting with me! And please call me Kintel cause that's my name.");
    println!("Kintel: I can also help with basic math operations. Just ask me to add, subtract, multiply, or divide two numbers! By the way, I also have some very interesting Science facts. Just ask me 'teach science' and I will be all over the place.");

    let mut math_problem = MathProblem::new();

    loop {
        let mut user_input = String::new();

        // Read user input from the console
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut user_input).unwrap();

        let response = match parse_operation(&user_input) {
            Some(Operation::Add) | Some(Operation::Subtract) | Some(Operation::Multiply) | Some(Operation::Divide) => {
                math_problem.set_operation(parse_operation(&user_input).unwrap());

                println!("Kintel: Enter the first number:");
                let mut num1_input = String::new();
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut num1_input).unwrap();
                let num1: f64 = num1_input.trim().parse().unwrap();

                println!("Kintel: Enter the second number:");
                let mut num2_input = String::new();
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut num2_input).unwrap();
                let num2: f64 = num2_input.trim().parse().unwrap();

                math_problem.set_numbers(num1, num2);

                let explanation = math_problem.explanation();
                let result = match math_problem.solve() {
                    Some(result) => format!("The result is: {}", result),
                    None => "Cannot divide by zero!".to_string(),
                };
                format!("{} {}", explanation, result)
            }

            Some(Operation::TeachScience) => {
                // Teach a science fact
                println!("Chatbot: Sure! Here's a science fact:");
                teach_science()
            }

            Some(Operation::None) => {
                "Kintel: I'm sorry, I don't understand the operation.".to_string()
            }

            None => "I'm sorry, I don't understand.".to_string(),
        };

        println!("Kintel: {}", response);
    }
}