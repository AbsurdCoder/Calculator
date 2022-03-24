use std::io::{stdin, stdout, Write};

fn read(input: &mut String) {
    stdout().flush()
        .expect("failed to flush");
    stdin().read_line(input)
        .expect("failed to read");
}

struct BinaryCalc {
    var1: f32,
    var2: f32,
    operator: char
}

fn main() {
    println!("----Hello Calculator----");

    loop {

        let mut var1 = String::new();
        let mut var2 = String::new();
        let mut operator = String::new();

        print!("First Value: ");
        read(&mut var1);

        print!("Second Value: ");
        read(&mut var2);

        print!("Operator  [+/-*] ");
        read(&mut operator);

        let calc = BinaryCalc {
            var1: var1.trim().parse().unwrap(),
            var2: var2.trim().parse().unwrap(),
            operator: operator.trim().chars().next().unwrap()
        };
        

        let operators = String::from("+-/*"); // Defining default characters

        if !operators.contains(calc.operator) {
            println!("unknown operator");
            continue;
        }

        let calc_result = match calc.operator {
            '+' => calc.sum(),
            '-' => calc.diff(),
            '/' => calc.div(),
            '*' => calc.multi(),
            _ => panic!("Error while calculating")
        };

        println!("Result ==> {}", calc_result);
         
    }
}

impl BinaryCalc {

    fn sum(&self) -> f32 {
        self.var1 + self.var2
    }
    fn diff(&self) -> f32 {
        self.var1 - self.var2
    }
    fn div(&self) -> f32 {
        self.var1 / self.var2
    }
    fn multi(&self) -> f32 {
        self.var1 * self.var2
    }

}