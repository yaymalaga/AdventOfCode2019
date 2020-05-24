use day9::computer::Computer;


fn run_program(program: &str, inputs: Vec<i64>) -> Vec<i64>{
    let mut computer = Computer::new();
    let program = program.split(',').map(|x| x.parse().expect("Invalid number")).collect();
    computer.load_program(program, inputs);
    computer.run();
    computer.get_outputs().expect("No outputs available").clone()
}

#[test]
fn test1() {
    let program = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99";
    let inputs: Vec<i64> = vec![];

    println!("{:?}", run_program(program, inputs));
}

#[test]
fn test2() {
    let program = "1102,34915192,34915192,7,4,7,99,0";
    let inputs: Vec<i64> = vec![];

    println!("{:?}", run_program(program, inputs));
}

#[test]
fn test3() {
    let program = "104,1125899906842624,99";
    let inputs: Vec<i64> = vec![];

    println!("{:?}", run_program(program, inputs));
}
