use array2d::Array2D;
use std::collections::HashMap;
use std::fmt::{self, write, Write};

// TODO:
// get the numbers in there
// figure out how to get strings working well, gonna need to make em into static strings
// implement all of the bf_ functions

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::North => "N",
                Self::South => "S",
                Self::East => "E",
                Self::West => "W",
            }
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Position(usize, usize);

impl Position {
    fn step(&self, dir: Direction) -> Self {
        match dir {
            Direction::North => Position(self.0, self.1 - 1),
            Direction::South => Position(self.0, self.1 + 1),
            Direction::East => Position(self.0 + 1, self.1),
            Direction::West => Position(self.0 - 1, self.1),
        }
    }
}

#[derive(Debug)]
enum OperatorSymbol {
    Blank,

    // data
    Number(usize),  // 0-9
    String(String), // stuff between speech marks

    // stack
    PushZero,         // (
    PopZero,          // )
    PopMainToControl, //
    PopControlToMain, //
    SwapStacks,       //

    // i/o
    Write, // w
    Read,  // r
    // number
    //
    Increment, // '
    Decrement, // `
    Add,
    Subtract,
    Divide,
    Multiply,

    // bitwise
    Not,
    And,
    Or,
    Xor,
    RotateLeft,
    RotateRight,

    // comparisons
    ToggleControl,
    EqualityCheck,
    LessThanCheck,
    GreaterThanCheck,

    // stack movement
    SwapTop,
    Dig,
    Bury,
    Flip,
    SwapLower,
    Over,
    Under,

    // misc
    Duplicate,
    Unduplicate,
    InverseMode,
    Halt,

    // direction changing
    Mirror1,     // \
    Mirror2,     // /
    EastBranch,  // >
    WestBranch,  // <
    SouthBranch, // v
    NorthBranch, // ^
}

#[derive(Debug)]
struct Operator {
    operator: OperatorSymbol,
    in_direction: Direction,
    inverse: bool,
}

#[derive(Debug)]
struct Expression {
    arr: Vec<Operator>,
    next: Branches,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct ExpressionIdentifier {
    position: Position,
    direction: Direction,
    inverse_mode: bool,
}

impl ExpressionIdentifier {
    fn new(inverse_mode: bool, position: Position, direction: Direction) -> Self {
        Self {
            position,
            direction,
            inverse_mode,
        }
    }
}

#[derive(Debug)]
enum Directions {
    Continue(Direction),
    ContinueInversed(Direction),
    Branch(Direction, Direction),
    Halt,
}

#[derive(Debug)]
enum Branches {
    None,
    One(ExpressionIdentifier),
    Two(ExpressionIdentifier, ExpressionIdentifier),
}
struct ExpressionTree {
    tree: HashMap<ExpressionIdentifier, Expression>,
    start: ExpressionIdentifier,
}

fn get_char(code: &Array2D<char>, position: Position) -> Option<&char> {
    code.get(position.1, position.0)
}

// modifies position for reading strings/numbers
fn parse_operator(
    position: &mut Position,
    direction: Direction,
    code: &Array2D<char>,
) -> (OperatorSymbol, Directions) {
    let Some(char) = get_char(&code, *position) else {
        return (OperatorSymbol::Halt, Directions::Halt);
    };
    match char {
        '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
            let mut x = char.to_digit(10).unwrap();
            loop {
                let next_char = get_char(&code, position.step(direction)).unwrap();
                if matches!(
                    next_char,
                    '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9'
                ) {
                    x = x * 10 + char.to_digit(10).unwrap();
                    *position = position.step(direction);
                } else {
                    return (
                        OperatorSymbol::Number(x as usize),
                        Directions::Continue(direction),
                    );
                }
            }
        }
        '"' => {
            let mut str = String::new();
            loop {
                let char = get_char(&code, position.step(direction)).unwrap();
                *position = position.step(direction);
                if *char != '"' {
                    str.push(*char);
                } else {
                    return (OperatorSymbol::String(str), Directions::Continue(direction));
                }
            }
        }
        ' ' => (OperatorSymbol::Blank, Directions::Continue(direction)),
        '(' => (OperatorSymbol::PushZero, Directions::Continue(direction)),
        ')' => (OperatorSymbol::PopZero, Directions::Continue(direction)),
        '[' => (
            OperatorSymbol::PopMainToControl,
            Directions::Continue(direction),
        ),
        ']' => (
            OperatorSymbol::PopControlToMain,
            Directions::Continue(direction),
        ),
        '$' => (OperatorSymbol::SwapStacks, Directions::Continue(direction)),
        'w' => (OperatorSymbol::Write, Directions::Continue(direction)),
        'r' => (OperatorSymbol::Read, Directions::Continue(direction)),
        '\'' => (OperatorSymbol::Increment, Directions::Continue(direction)),
        '`' => (OperatorSymbol::Decrement, Directions::Continue(direction)),
        '+' => (OperatorSymbol::Add, Directions::Continue(direction)),
        '-' => (OperatorSymbol::Subtract, Directions::Continue(direction)),
        '%' => (OperatorSymbol::Divide, Directions::Continue(direction)),
        '*' => (OperatorSymbol::Multiply, Directions::Continue(direction)),
        '~' => (OperatorSymbol::Not, Directions::Continue(direction)),
        '&' => (OperatorSymbol::And, Directions::Continue(direction)),
        '|' => (OperatorSymbol::Or, Directions::Continue(direction)),
        '#' => (OperatorSymbol::Xor, Directions::Continue(direction)),
        '{' => (OperatorSymbol::RotateLeft, Directions::Continue(direction)),
        '}' => (OperatorSymbol::RotateRight, Directions::Continue(direction)),
        '!' => (
            OperatorSymbol::ToggleControl,
            Directions::Continue(direction),
        ),
        '=' => (
            OperatorSymbol::EqualityCheck,
            Directions::Continue(direction),
        ),
        'l' => (
            OperatorSymbol::LessThanCheck,
            Directions::Continue(direction),
        ),
        'g' => (
            OperatorSymbol::GreaterThanCheck,
            Directions::Continue(direction),
        ),
        's' => (OperatorSymbol::SwapTop, Directions::Continue(direction)),
        'd' => (OperatorSymbol::Dig, Directions::Continue(direction)),
        'b' => (OperatorSymbol::Bury, Directions::Continue(direction)),
        'f' => (OperatorSymbol::Flip, Directions::Continue(direction)),
        'c' => (OperatorSymbol::SwapLower, Directions::Continue(direction)),
        'o' => (OperatorSymbol::Over, Directions::Continue(direction)),
        'u' => (OperatorSymbol::Under, Directions::Continue(direction)),
        ':' => (OperatorSymbol::Duplicate, Directions::Continue(direction)),
        ';' => (OperatorSymbol::Unduplicate, Directions::Continue(direction)),
        '?' => (
            OperatorSymbol::InverseMode,
            Directions::ContinueInversed(direction),
        ),
        '@' => (OperatorSymbol::Halt, Directions::Halt),

        '\\' => (
            OperatorSymbol::Mirror1,
            match direction {
                Direction::North => Directions::Continue(Direction::West),
                Direction::South => Directions::Continue(Direction::East),
                Direction::East => Directions::Continue(Direction::South),
                Direction::West => Directions::Continue(Direction::North),
            },
        ),
        '/' => (
            OperatorSymbol::Mirror2,
            match direction {
                Direction::North => Directions::Continue(Direction::East),
                Direction::South => Directions::Continue(Direction::West),
                Direction::East => Directions::Continue(Direction::North),
                Direction::West => Directions::Continue(Direction::South),
            },
        ),
        '>' => (
            OperatorSymbol::EastBranch,
            match direction {
                Direction::North => Directions::Continue(Direction::East),
                Direction::South => Directions::Continue(Direction::East),
                Direction::East => Directions::Continue(Direction::West),
                Direction::West => Directions::Branch(Direction::North, Direction::South),
            },
        ),
        '<' => (
            OperatorSymbol::WestBranch,
            match direction {
                Direction::North => Directions::Continue(Direction::West),
                Direction::South => Directions::Continue(Direction::West),
                Direction::East => Directions::Branch(Direction::South, Direction::North),
                Direction::West => Directions::Continue(Direction::East),
            },
        ),
        'v' => (
            OperatorSymbol::SouthBranch,
            match direction {
                Direction::North => Directions::Branch(Direction::East, Direction::West),
                Direction::South => Directions::Continue(Direction::North),
                Direction::East => Directions::Continue(Direction::South),
                Direction::West => Directions::Continue(Direction::South),
            },
        ),
        '^' => (
            OperatorSymbol::NorthBranch,
            match direction {
                Direction::North => Directions::Continue(Direction::South),
                Direction::South => Directions::Branch(Direction::West, Direction::East),
                Direction::East => Directions::Continue(Direction::North),
                Direction::West => Directions::Continue(Direction::North),
            },
        ),

        'J' => (
            OperatorSymbol::SwapStacks,
            Directions::Branch(Direction::South, Direction::East),
        ),
        _ => panic!("{char}: invalid char!!"),
    }
}

fn parse_expression(
    code: &Array2D<char>,
    mut position: Position,
    mut direction: Direction,
    mut inverse_mode: bool,
    data: &mut ExpressionTree,
) {
    let mut expression = vec![];
    let initial_identifier = ExpressionIdentifier::new(inverse_mode, position, direction);
    loop {
        // position is skipped forwards if reading a string/number
        let (operator, directions) = parse_operator(&mut position, direction, &code);

        expression.push(Operator {
            operator,
            in_direction: direction,
            inverse: inverse_mode,
        });

        match directions {
            Directions::Continue(dir1) => {
                direction = dir1;
                position = position.step(direction);
                continue;
            }
            Directions::ContinueInversed(dir1) => {
                direction = dir1;
                position = position.step(direction);
                inverse_mode = !inverse_mode;
                continue;
            }
            Directions::Halt => {
                println!("\n{expression:?}\n");
                data.tree.insert(
                    initial_identifier,
                    Expression {
                        arr: expression,
                        next: Branches::None,
                    },
                );
                return;
            }
            Directions::Branch(dir1, dir2) => {
                println!("\n{expression:?}\n");
                data.tree.insert(
                    initial_identifier,
                    Expression {
                        arr: expression,
                        next: Branches::Two(
                            ExpressionIdentifier {
                                position: position.step(dir1),
                                direction: dir1,
                                inverse_mode,
                            },
                            ExpressionIdentifier {
                                position: position.step(dir2),
                                direction: dir2,
                                inverse_mode,
                            },
                        ),
                    },
                );
                if !data.tree.contains_key(&ExpressionIdentifier::new(
                    inverse_mode,
                    position.step(dir1),
                    dir1,
                )) {
                    parse_expression(&code, position.step(dir1), dir1, inverse_mode, data);
                } else {
                    println!("match found");
                    println!("{:?}, {:?}, {}\n", position.step(dir1), dir1, inverse_mode);
                };
                if !data.tree.contains_key(&ExpressionIdentifier::new(
                    inverse_mode,
                    position.step(dir2),
                    dir2,
                )) {
                    parse_expression(&code, position.step(dir2), dir2, inverse_mode, data);
                } else {
                    println!("match found");
                    println!("{:?}, {:?}, {}\n", position.step(dir2), dir2, inverse_mode);
                };
                return;
            }
        }
    }
}

fn get_start_pos(code: &Array2D<char>) -> Option<Position> {
    let mut start = None;
    for (index_y, mut row) in code.rows_iter().enumerate() {
        if let Some(index_x) = row.position(|x| *x == '@') {
            start = Some(Position(index_x, index_y));
            break;
        }
    }
    start
}

// COMPILING

fn operator_to_llvm_ir(str: &mut String, operator_data: Operator) {
    let Operator {
        operator,
        inverse,
        in_direction: direction,
    } = operator_data;
    str.push_str(match operator {
        OperatorSymbol::Blank => "",

        // data
        //OperatorSymbol::Number(num) => format!("call void @push_stack(i32 {})", num),
        OperatorSymbol::Number(num) => todo!(),
        OperatorSymbol::String(str) => todo!(), // stuff between speech marks

        // stack
        OperatorSymbol::PushZero => "\ncall void @bf_PushZero()",
        OperatorSymbol::PopZero => "\ncall void @bf_PopZero()",
        OperatorSymbol::PopMainToControl => "\ncall void @bf_PopMainToControl()",
        OperatorSymbol::PopControlToMain => "\ncall void @bf_PopControlToMain()",
        OperatorSymbol::SwapStacks => "\ncall void @bf_SwapStacks()",

        // i/o
        OperatorSymbol::Write => "\ncall void @bf_Write()",
        OperatorSymbol::Read => "\ncall void @bf_Read()",
        // number
        //
        OperatorSymbol::Increment => "\ncall void @bf_Increment()",
        OperatorSymbol::Decrement => "\ncall void @bf_Decrement()",
        OperatorSymbol::Add => "\ncall void @bf_Add()",
        OperatorSymbol::Subtract => "\ncall void @bf_Subtract()",
        OperatorSymbol::Divide => "\ncall void @bf_Divide()",
        OperatorSymbol::Multiply => "\ncall void @bf_Multiply()",

        // bitwise
        OperatorSymbol::Not => "\ncall void @bf_Not()",
        OperatorSymbol::And => "\ncall void @bf_And()",
        OperatorSymbol::Or => "\ncall void @bf_Or()",
        OperatorSymbol::Xor => "\ncall void @bf_Xor()",
        OperatorSymbol::RotateLeft => "\ncall void @bf_RotateLeft()",
        OperatorSymbol::RotateRight => "\ncall void @bf_RotateRight()",

        // comparisons
        OperatorSymbol::ToggleControl => "\ncall void @bf_ToggleControl()",
        OperatorSymbol::EqualityCheck => "\ncall void @bf_EqualityCheck()",
        OperatorSymbol::LessThanCheck => "\ncall void @bf_LessThanCheck()",
        OperatorSymbol::GreaterThanCheck => "\ncall void @bf_GreaterThanCheck()",

        // stack movement
        OperatorSymbol::SwapTop => "\ncall void @bf_SwapTop()",
        OperatorSymbol::Dig => "\ncall void @bf_Dig()",
        OperatorSymbol::Bury => "\ncall void @bf_Bury()",
        OperatorSymbol::Flip => "\ncall void @bf_Flip()",
        OperatorSymbol::SwapLower => "\ncall void @bf_SwapLower()",
        OperatorSymbol::Over => "\ncall void @bf_Over()",
        OperatorSymbol::Under => "\ncall void @bf_Under()",

        // misc
        OperatorSymbol::Duplicate => "\ncall void @bf_Duplicate()",
        OperatorSymbol::Unduplicate => "\ncall void @bf_Unduplicate()",
        OperatorSymbol::InverseMode => "\ncall void @bf_InverseMode()",
        OperatorSymbol::Halt => "", // Maybe make this output some info, like the stack

        // direction changing
        // determined at parse time
        OperatorSymbol::Mirror1 => "",
        OperatorSymbol::Mirror2 => "",
        OperatorSymbol::EastBranch => "",
        OperatorSymbol::WestBranch => "",
        OperatorSymbol::SouthBranch => "",
        OperatorSymbol::NorthBranch => "",
    })
}

const PRELUDE: &str = r#"
;; globals
@int_str = private unnamed_addr constant [4 x i8] c"%d\0A\00", align 1
declare dso_local i32 @printf(i8*, ...) #1

@primary_stack = global [40 x i32]  zeroinitializer, align 4
@primary_offset = global i32 0

@control_stack = global [40 x i32]  zeroinitializer, align 4
@control_offset = global i32 0

;; general utility functions

define void @print_int(i32 %val) {
    call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @int_str, i64 0, i64 0), i32 %val)
    ret void
}

define void @increment_stack(i32 %amount) {
    %offset.0 = load i32, i32* @primary_offset
    %offset.1 = add i32 %offset.0, %amount
    store i32 %offset.1, i32* @primary_offset
    ret void
}

define void @increment_control_stack(i32 %amount) {
    %offset.0 = load i32, i32* @control_offset
    %offset.1 = add i32 %offset.0, %amount
    store i32 %offset.1, i32* @control_offset
    ret void
}

define void @push_stack(i32 %val) {
    ; increment pointer by one
    call void @increment_stack(i32 1)

    ; put val onto the stack at pointer
    %offset = load i32, i32* @primary_offset
    %ptr = getelementptr [40 x i32], i32* @primary_stack, i32 0, i32 %offset
    store i32 %val, ptr %ptr

    ret void
}

define void @push_control_stack(i32 %val) {
    ; increment pointer by one
    call void @increment_control_stack(i32 1)

    ; put val onto the stack at pointer
    %offset = load i32, i32* @control_offset
    %ptr = getelementptr [40 x i32], i32* @control_stack, i32 0, i32 %offset
    store i32 %val, ptr %ptr

    ret void
}

define i32 @pop_stack() {
    ; get val from the stack at pointer
    %offset = load i32, i32* @primary_offset
    %ptr = getelementptr [40 x i32], i32* @primary_stack, i32 0, i32 %offset
    %val = load i32, i32* %ptr

    ; decrement pointer by one
    call void @increment_stack(i32 -1)

    ret i32 %val
}

define i32 @pop_control_stack() {
    ; get val from the stack at pointer
    %offset = load i32, i32* @control_offset
    %ptr = getelementptr [40 x i32], i32* @control_stack, i32 0, i32 %offset
    %val = load i32, i32* %ptr

    ; decrement pointer by one
    call void @increment_control_stack(i32 -1)

    ret i32 %val
}

; zero = zero, everything else = 1
define i1 @pop_control_stack_i1() {
    %val = call i32 @pop_control_stack()
    ; check if control stack is zero or one
    %res = icmp ne i32 %val, 0
    ret i1 %res
}

;; specific befreak operator impls

define void @bf_Number(i32 %num) {
    %val.0 = call i32 @pop_stack()
    %val.1 = xor i32 %val.0, %num
    call void @push_stack(i32 %val.1)
    ret void
}

;define void @bf_String(String)() {
;ret void
;}

; simple stack
define void @bf_PushZero() {
    call void @push_stack(i32 0)
    ret void
}

define void @bf_PopZero() {
    call void @pop_stack()
    ret void
}

define void @bf_PopMainToControl() {
    %1 = call i32 @pop_stack()
    call void @push_control_stack(i32 %1)
    ret void
}

define void @bf_PopControlToMain() {
    %1 = call i32 @pop_control_stack()
    call void @push_stack(i32 %1)
    ret void
}

define void @bf_SwapStacks() {
    %1 = call i32 @pop_stack()
    %2 = call i32 @pop_control_stack()
    call void @push_stack(i32 %2)
    call void @push_control_stack(i32 %1)
    ret void
}

; i/o
define void @bf_Write() {
ret void
}
define void @bf_Read() {
ret void
}

; number
define void @bf_Increment() {
ret void
}
define void @bf_Decrement() {
ret void
}
define void @bf_Add() {
ret void
}
define void @bf_Subtract() {
ret void
}
define void @bf_Divide() {
ret void
}
define void @bf_Multiply() {
ret void
}

; bitwise
define void @bf_Not() {
ret void
}
define void @bf_And() {
ret void
}
define void @bf_Or() {
ret void
}
define void @bf_Xor() {
ret void
}
define void @bf_RotateLeft() {
ret void
}
define void @bf_RotateRight() {
ret void
}

; comparisons
define void @bf_ToggleControl() {
ret void
}
define void @bf_EqualityCheck() {
ret void
}
define void @bf_LessThanCheck() {
ret void
}
define void @bf_GreaterThanCheck() {
ret void
}

; complex stack
define void @bf_SwapTop() {
ret void
}
define void @bf_Dig() {
ret void
}
define void @bf_Bury() {
ret void
}
define void @bf_Flip() {
ret void
}
define void @bf_SwapLower() {
ret void
}
define void @bf_Over() {
ret void
}
define void @bf_Under() {
ret void
}

; misc
define void @bf_Duplicate() {
ret void
}
define void @bf_Unduplicate() {
ret void
}
define void @bf_InverseMode() {
ret void
}
define void @bf_Halt() {
ret void
}

;; actual codegen begin
"#;

fn identifier_to_string(identifier: ExpressionIdentifier) -> String {
    let ExpressionIdentifier {
        position,
        direction,
        inverse_mode,
    } = identifier;
    format!(
        "@bf_cg_{}_{}_{}_{}",
        position.0, position.1, direction, inverse_mode
    )
}

fn compile(data: ExpressionTree) {
    let mut llvm_ir = String::from(PRELUDE);
    for (identifier, expression) in data.tree.into_iter() {
        write!(
            llvm_ir,
            "define void {}() {{",
            identifier_to_string(identifier)
        )
        .unwrap();
        for operator in expression.arr {
            operator_to_llvm_ir(&mut llvm_ir, operator);
        }
        match expression.next {
            Branches::None => llvm_ir.push_str("\nret void"),
            Branches::One(id1) => {
                write!(llvm_ir, "call void {}()", identifier_to_string(id1)).unwrap();
                llvm_ir.push_str("\nret void");
            }
            Branches::Two(id1, id2) => {
                llvm_ir.push_str(
                    "
%cond = call i1 @pop_control_stack_i1()
br i1 %cond, label %branch_true, label %branch_false\n",
                );
                write!(
                    llvm_ir,
                    "branch_true:\ncall void {}()\nret void\n",
                    identifier_to_string(id1)
                )
                .unwrap();
                write!(
                    llvm_ir,
                    "branch_false:\ncall void {}()\nret void",
                    identifier_to_string(id2)
                )
                .unwrap();
            }
        }
        llvm_ir.push_str("\n}\n");
    }

    write!(
        llvm_ir,
        "
;; actual codegen over

define void @main() {{
    call void {}()
    ret void
}}",
        identifier_to_string(data.start)
    )
    .unwrap();
    println!("{llvm_ir}");
}

// NOTES:
// blank can be ignored completely
// all branches that are not the final operator can be ignored

fn read_string(data: &str) -> Array2D<char> {
    let mut lines = vec![];
    let max_length = data.lines().map(str::len).max().unwrap();
    for line in data.lines() {
        let mut x = line.chars().collect::<Vec<char>>();
        x.resize(max_length, ' ');
        lines.push(x);
    }
    Array2D::from_rows(&lines).unwrap()
}

fn parse_code(code: &Array2D<char>) -> ExpressionTree {
    let start_pos = get_start_pos(&code).unwrap().step(Direction::East);
    let mut data = ExpressionTree {
        tree: HashMap::new(),
        start: ExpressionIdentifier {
            position: start_pos,
            direction: Direction::East,
            inverse_mode: false,
        },
    };
    parse_expression(&code, start_pos, Direction::East, false, &mut data);
    data
}

fn main() {
    let data = r#"
    /1)@(1\         
    >)1=1(<         
    \'(v?)/         
       >'%s(\       
     ^ >*s)=/       
     >=<            
     (              
/s'0v^?w23(v`s]:(48\
[   (      )       +
)   =      =       4
0   c      c       8
1   =      =       )
%   )      (       w
\01(^      ^)01*01(/"#;
    let data = r#"@(())"#;
    let data = r#"
/"Hello world!"01\
\(13v     'wsv)@(/
    \(=13=13)/    
"#;
    let code = read_string(data);
    println!("{code:?}");
    let tree = parse_code(&code);
    println!("tree: ");
    for val in tree.tree.keys() {
        println!("{val:?}");
    }
    compile(tree);
}
