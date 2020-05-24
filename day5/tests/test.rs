use day5::Computer;

fn run_program(program: &str, inputs: Vec<i32>) -> Vec<i32>{
    let mut computer = Computer::new();
    let program = program.split(',').map(|x| x.parse().expect("Invalid number")).collect();
    computer.load_program(program, inputs);
    computer.run();
    computer.get_outputs().expect("No outputs available").clone()
}

#[test]
fn test1() {
    let program = "3,9,8,9,10,9,4,9,99,-1,8";
    let inputs: Vec<i32> = vec![8];

    assert_eq!(run_program(program, inputs.clone()).len(), 1);
    assert_eq!(run_program(program, inputs)[0], 1);
}

#[test]
fn test2() {
    let program = "3,9,8,9,10,9,4,9,99,-1,8";
    let inputs: Vec<i32> = vec![5];

    assert_eq!(run_program(program, inputs.clone()).len(), 1);
    assert_eq!(run_program(program, inputs)[0], 0);
}

#[test]
fn test3() {
    let program = "3,9,7,9,10,9,4,9,99,-1,8";
    let inputs: Vec<i32> = vec![5];

    assert_eq!(run_program(program, inputs.clone()).len(), 1);
    assert_eq!(run_program(program, inputs)[0], 1);
}

#[test]
fn test4() {
    let program = "3,9,7,9,10,9,4,9,99,-1,8";
    let inputs: Vec<i32> = vec![8];

    assert_eq!(run_program(program, inputs.clone()).len(), 1);
    assert_eq!(run_program(program, inputs)[0], 0);
}

#[test]
fn test5() {
    let program = "3,3,1108,-1,8,3,4,3,99";
    let inputs: Vec<i32> = vec![8];

    assert_eq!(run_program(program, inputs.clone()).len(), 1);
    assert_eq!(run_program(program, inputs)[0], 1);
}

#[test]
fn test6() {
    let program = "3,3,1108,-1,8,3,4,3,99";
    let inputs: Vec<i32> = vec![5];

    assert_eq!(run_program(program, inputs.clone()).len(), 1);
    assert_eq!(run_program(program, inputs)[0], 0);
}

#[test]
fn test7() {
    let program = "3,3,1107,-1,8,3,4,3,99";
    let inputs: Vec<i32> = vec![5];

    assert_eq!(run_program(program, inputs.clone()).len(), 1);
    assert_eq!(run_program(program, inputs)[0], 1);
}

#[test]
fn test8() {
    let program = "3,3,1107,-1,8,3,4,3,99";
    let inputs: Vec<i32> = vec![8];

    assert_eq!(run_program(program, inputs.clone()).len(), 1);
    assert_eq!(run_program(program, inputs)[0], 0);
}

#[test]
fn test9() {
    let program = "3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9";
    let inputs: Vec<i32> = vec![0];

    assert_eq!(run_program(program, inputs.clone()).len(), 1);
    assert_eq!(run_program(program, inputs)[0], 0);
}

#[test]
fn test10() {
    let program = "3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9";
    let inputs: Vec<i32> = vec![6];

    assert_eq!(run_program(program, inputs.clone()).len(), 1);
    assert_eq!(run_program(program, inputs)[0], 1);
}

#[test]
fn test11() {
    let program = "3,3,1105,-1,9,1101,0,0,12,4,12,99,1";
    let inputs: Vec<i32> = vec![0];

    assert_eq!(run_program(program, inputs.clone()).len(), 1);
    assert_eq!(run_program(program, inputs)[0], 0);
}

#[test]
fn test12() {
    let program = "3,3,1105,-1,9,1101,0,0,12,4,12,99,1";
    let inputs: Vec<i32> = vec![6];

    assert_eq!(run_program(program, inputs.clone()).len(), 1);
    assert_eq!(run_program(program, inputs)[0], 1);
}

#[test]
fn test13() {
    let program = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
    let inputs: Vec<i32> = vec![7];

    assert_eq!(run_program(program, inputs.clone()).len(), 1);
    assert_eq!(run_program(program, inputs)[0], 999);
}

#[test]
fn test14() {
    let program = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
    let inputs: Vec<i32> = vec![8];

    assert_eq!(run_program(program, inputs.clone()).len(), 1);
    assert_eq!(run_program(program, inputs)[0], 1000);
}

#[test]
fn test15() {
    let program = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
    let inputs: Vec<i32> = vec![9];

    assert_eq!(run_program(program, inputs.clone()).len(), 1);
    assert_eq!(run_program(program, inputs)[0], 1001);
}