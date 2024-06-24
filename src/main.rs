#![allow(dead_code, clippy::needless_raw_string_hashes)]
use array2d::Array2D;
use std::collections::HashMap;
use std::fmt::Write;

// TODO:
// implement all of the bf_ functions
// fix num/string going over edges
// figure out what to do with inverted write and inverted read
// implement inverted string (just pop n elements off the stack cuz who needs validating?)
// automatically pipe the output into `clang -O3 -x "ir" -`

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Position(usize, usize);

impl Position {
    const fn step(&self, dir: Direction) -> Self {
        match dir {
            Direction::North => Self(self.0, self.1 - 1),
            Direction::South => Self(self.0, self.1 + 1),
            Direction::East => Self(self.0 + 1, self.1),
            Direction::West => Self(self.0 - 1, self.1),
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
    operation: OperatorSymbol,
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
    const fn new(inverse_mode: bool, position: Position, direction: Direction) -> Self {
        Self {
            position,
            direction,
            inverse_mode,
        }
    }

    fn to_codegen_function_name(&self) -> String {
        let Self {
            position,
            direction,
            inverse_mode,
        } = *self;
        format!(
            "@bf_cg_{}_{}_{}_{}",
            position.0,
            position.1,
            match direction {
                Direction::North => "N",
                Direction::South => "S",
                Direction::East => "E",
                Direction::West => "W",
            },
            if inverse_mode { "inverse" } else { "normal" }
        )
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

#[allow(clippy::match_same_arms, clippy::too_many_lines)]
fn parse_operator(
    position: &mut Position, // modifies position for reading strings/numbers
    direction: Direction,
    code: &Array2D<char>,
) -> (OperatorSymbol, Directions) {
    let Some(char) = get_char(code, *position) else {
        return (OperatorSymbol::Halt, Directions::Halt);
    };
    match char {
        '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
            let mut x = char.to_digit(10).unwrap();
            loop {
                let next_char = get_char(code, position.step(direction)).unwrap();
                if matches!(
                    next_char,
                    '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9'
                ) {
                    x = x * 10 + next_char.to_digit(10).unwrap();
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
                let char = get_char(code, position.step(direction)).unwrap();
                *position = position.step(direction);
                if *char == '"' {
                    return (OperatorSymbol::String(str), Directions::Continue(direction));
                }
                str.push(*char);
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
                Direction::East => Directions::ContinueInversed(Direction::West),
                // north if one, south if zero
                Direction::West => Directions::Branch(Direction::North, Direction::South),
            },
        ),
        '<' => (
            OperatorSymbol::WestBranch,
            match direction {
                Direction::North => Directions::Continue(Direction::West),
                Direction::South => Directions::Continue(Direction::West),
                Direction::East => Directions::Branch(Direction::South, Direction::North),
                Direction::West => Directions::ContinueInversed(Direction::East),
            },
        ),
        'v' => (
            OperatorSymbol::SouthBranch,
            match direction {
                Direction::North => Directions::Branch(Direction::East, Direction::West),
                Direction::South => Directions::ContinueInversed(Direction::North),
                Direction::East => Directions::Continue(Direction::South),
                Direction::West => Directions::Continue(Direction::South),
            },
        ),
        '^' => (
            OperatorSymbol::NorthBranch,
            match direction {
                Direction::North => Directions::ContinueInversed(Direction::South),
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
        let (operator, directions) = parse_operator(&mut position, direction, code);

        expression.push(Operator {
            operation: operator,
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
                let one = ExpressionIdentifier {
                    position: position.step(dir1),
                    direction: dir1,
                    inverse_mode,
                };
                let two = ExpressionIdentifier {
                    position: position.step(dir2),
                    direction: dir2,
                    inverse_mode,
                };
                if inverse_mode {
                    data.tree.insert(
                        initial_identifier,
                        Expression {
                            arr: expression,
                            next: Branches::Two(two, one),
                        },
                    );
                } else {
                    data.tree.insert(
                        initial_identifier,
                        Expression {
                            arr: expression,
                            next: Branches::Two(one, two),
                        },
                    );
                }

                if !data.tree.contains_key(&ExpressionIdentifier::new(
                    inverse_mode,
                    position.step(dir1),
                    dir1,
                )) {
                    parse_expression(code, position.step(dir1), dir1, inverse_mode, data);
                };
                if !data.tree.contains_key(&ExpressionIdentifier::new(
                    inverse_mode,
                    position.step(dir2),
                    dir2,
                )) {
                    parse_expression(code, position.step(dir2), dir2, inverse_mode, data);
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

fn string_to_i32_arr(str: &str) -> String {
    let mut res = str.chars().fold(String::new(), |mut acc, char| {
        write!(acc, "i32 {}, ", char as u64).unwrap();
        acc
    });
    res.pop();
    res.pop();
    res
}

fn string_llvm_ir(str: &str, string_name: &str) -> String {
    format!(
        "
    ; STRING CODE BEGIN
    call void @increment_stack(i32 1)

    ; paste string onto the stack
    %offset = load i32, i32* @primary_offset
    %ptr = getelementptr [40 x i32], i32* @primary_stack, i32 0, i32 %offset
    %str = load [{length} x i32], i32* @{var}
    store [{length} x i32] %str, ptr %ptr

    call void @increment_stack(i32 {length_minus_one}) ; len - 1
    ; STRING CODE END
",
        var = string_name,
        length = str.len(),
        length_minus_one = str.len() - 1
    )
}

#[allow(clippy::match_same_arms, clippy::too_many_lines)]
fn operator_to_llvm_ir(str: &mut String, epilogue: &mut String, operator_data: Operator) {
    let Operator {
        operation: operator,
        inverse,
        in_direction: direction,
    } = operator_data;

    let tmp;

    let addition = match (inverse, operator) {
        (_, OperatorSymbol::Blank) => "",

        // data
        (_, OperatorSymbol::Number(num)) => {
            tmp = format!("call void @bf_Number(i32 {num})");
            &tmp
        }
        (false, OperatorSymbol::String(str)) => {
            let string_name = "wasd";
            // add string data to prologue of function
            write!(
                epilogue,
                "\n@{var} = private unnamed_addr constant [{length} x i32] [{arr}], align 4\n",
                var = string_name,
                length = str.len(),
                arr = string_to_i32_arr(&str)
            )
            .unwrap();

            // insert the string code inline
            tmp = string_llvm_ir(&str, string_name);
            &tmp
        } // stuff between speech marks
        (true, OperatorSymbol::String(_)) => "call void @print_int(i32 40300)\ncall void @unimplemented()",

        // stack
        (false, OperatorSymbol::PushZero) => "call void @bf_PushZero()",
        (true, OperatorSymbol::PushZero) => "call void @bf_PopZero()",

        (false, OperatorSymbol::PopZero) => "call void @bf_PopZero()",
        (true, OperatorSymbol::PopZero) => "call void @bf_PushZero()",

        (false, OperatorSymbol::PopMainToControl) => "call void @bf_PopMainToControl()",
        (true, OperatorSymbol::PopMainToControl) => "call void @bf_PopControlToMain()",

        (false, OperatorSymbol::PopControlToMain) => "call void @bf_PopControlToMain()",
        (true, OperatorSymbol::PopControlToMain) => "call void @bf_PopMainToControl()",

        (_, OperatorSymbol::SwapStacks) => "call void @bf_SwapStacks()",

        // i/o
        (false, OperatorSymbol::Write) => "call void @bf_Write()",
        (true, OperatorSymbol::Write) => "call void @print_int(i32 40400)\ncall void @unimplemented()",

        (false, OperatorSymbol::Read) => "call void @bf_Read()",
        (true, OperatorSymbol::Read) => "call void @print_int(i32 40500)\ncall void @unimplemented()",

        // number
        (false, OperatorSymbol::Increment) => "call void @bf_Increment()",
        (true, OperatorSymbol::Increment) => "call void @bf_Decrement()",

        (false, OperatorSymbol::Decrement) => "call void @bf_Decrement()",
        (true, OperatorSymbol::Decrement) => "call void @bf_Increment()",

        (false, OperatorSymbol::Add) => "call void @bf_Add()",
        (true, OperatorSymbol::Add) => "call void @bf_Subtract()",

        (false, OperatorSymbol::Subtract) => "call void @bf_Subtract()",
        (true, OperatorSymbol::Subtract) => "call void @bf_Add()",

        (false, OperatorSymbol::Divide) => "call void @bf_Divide()",
        (true, OperatorSymbol::Divide) => "call void @bf_Multiply()",

        (false, OperatorSymbol::Multiply) => "call void @bf_Multiply()",
        (true, OperatorSymbol::Multiply) => "call void @bf_Divide()",

        // bitwise
        (_, OperatorSymbol::Not) => "call void @bf_Not()",

        (_, OperatorSymbol::And) => "call void @bf_And()",

        (_, OperatorSymbol::Or) => "call void @bf_Or()",

        (_, OperatorSymbol::Xor) => "call void @bf_Xor()",

        (false, OperatorSymbol::RotateLeft) => "call void @bf_RotateLeft()",
        (true, OperatorSymbol::RotateLeft) => "call void @bf_RotateRight()",

        (false, OperatorSymbol::RotateRight) => "call void @bf_RotateRight()",
        (true, OperatorSymbol::RotateRight) => "call void @bf_RotateLeft()",

        // comparisons
        (_, OperatorSymbol::ToggleControl) => "call void @bf_ToggleControl()",

        (_, OperatorSymbol::EqualityCheck) => "call void @bf_EqualityCheck()",

        (_, OperatorSymbol::LessThanCheck) => "call void @bf_LessThanCheck()",

        (_, OperatorSymbol::GreaterThanCheck) => "call void @bf_GreaterThanCheck()",

        // stack movement
        (_, OperatorSymbol::SwapTop) => "call void @bf_SwapTop()",

        (false, OperatorSymbol::Dig) => "call void @bf_Dig()",
        (true, OperatorSymbol::Dig) => "call void @bf_Bury()",

        (false, OperatorSymbol::Bury) => "call void @bf_Bury()",
        (true, OperatorSymbol::Bury) => "call void @bf_Dig()",

        (_, OperatorSymbol::Flip) => "call void @bf_Flip()",

        (_, OperatorSymbol::SwapLower) => "call void @bf_SwapLower()",

        (false, OperatorSymbol::Over) => "call void @bf_Over()",
        (true, OperatorSymbol::Over) => "call void @bf_Under()",

        (false, OperatorSymbol::Under) => "call void @bf_Under()",
        (true, OperatorSymbol::Under) => "call void @bf_Over()",

        // misc
        (false, OperatorSymbol::Duplicate) => "call void @bf_Duplicate()",
        (true, OperatorSymbol::Duplicate) => "call void @bf_Unduplicate()",

        (false, OperatorSymbol::Unduplicate) => "call void @bf_Unduplicate()",
        (true, OperatorSymbol::Unduplicate) => "call void @bf_Duplicate()",

        (_, OperatorSymbol::InverseMode) => "", // handled at parse time
        (_, OperatorSymbol::Halt) => "",        // Maybe make this output some info, like the stack

        // direction changing
        (_, OperatorSymbol::Mirror1) => "", // handled at parse time
        (_, OperatorSymbol::Mirror2) => "", // handled at parse time
        (false, OperatorSymbol::EastBranch) => match direction {
            Direction::North => "call void @push_control_stack(i32 1)",
            Direction::South => "call void @push_control_stack(i32 0)",
            Direction::East => "call void @toggle_control_stack()",
            Direction::West => "", // dealt with elsewhere
        },
        (true, OperatorSymbol::EastBranch) => match direction {
            Direction::North => "call void @push_control_stack(i32 0)",
            Direction::South => "call void @push_control_stack(i32 1)",
            Direction::East => "call void @toggle_control_stack()",
            Direction::West => "", // dealt with elsewhere
        },

        (false, OperatorSymbol::WestBranch) => match direction {
            Direction::North => "call void @push_control_stack(i32 0)",
            Direction::South => "call void @push_control_stack(i32 1)",
            Direction::East => "", // dealt with elsewhere
            Direction::West => "call void @toggle_control_stack()",
        },
        (true, OperatorSymbol::WestBranch) => match direction {
            Direction::North => "call void @push_control_stack(i32 1)",
            Direction::South => "call void @push_control_stack(i32 0)",
            Direction::East => "", // dealt with elsewhere
            Direction::West => "call void @toggle_control_stack()",
        },

        (false, OperatorSymbol::SouthBranch) => match direction {
            Direction::North => "", // dealt with elsewhere
            Direction::South => "call void @toggle_control_stack()",
            Direction::East => "call void @push_control_stack(i32 1)",
            Direction::West => "call void @push_control_stack(i32 0)",
        },
        (true, OperatorSymbol::SouthBranch) => match direction {
            Direction::North => "", // dealt with elsewhere
            Direction::South => "call void @toggle_control_stack()",
            Direction::East => "call void @push_control_stack(i32 0)",
            Direction::West => "call void @push_control_stack(i32 1)",
        },

        (false, OperatorSymbol::NorthBranch) => match direction {
            Direction::North => "call void @toggle_control_stack()",
            Direction::South => "", // dealt with elsewhere
            Direction::East => "call void @push_control_stack(i32 0)",
            Direction::West => "call void @push_control_stack(i32 1)",
        },
        (true, OperatorSymbol::NorthBranch) => match direction {
            Direction::North => "call void @toggle_control_stack()",
            Direction::South => "", // dealt with elsewhere
            Direction::East => "call void @push_control_stack(i32 1)",
            Direction::West => "call void @push_control_stack(i32 0)",
        },
    };
    if !addition.is_empty() {
        str.push_str("\n    ");
        str.push_str(addition);
    }
}

const PRELUDE: &str = r#"
;; globals
@int_str = private unnamed_addr constant [4 x i8] c"%d\0A\00", align 1
@char_str = private unnamed_addr constant [3 x i8] c"%c\00", align 1
@stack_str = private unnamed_addr constant [8 x i8] c"stack:\0A\00", align 1
@newline_str = private unnamed_addr constant [3 x i8] c"\0A\0A\00", align 1
@unimplemented_str = private unnamed_addr constant [15 x i8] c"unimplemented!\00", align 1

declare dso_local i32 @printf(i8*, ...) #1
;declare dso_local i32 @sleep(i32) #1
declare dso_local void @exit(i32) #1

declare dso_local i32 @llvm.fshl.i32(i32, i32, i32) #1
declare dso_local i32 @llvm.fshr.i32(i32, i32, i32) #1

; offsets point at the most recent value inserted
; so must be incremented if you want to add
; but can be used directly for peek
@primary_stack = global [40 x i32]  zeroinitializer, align 4
@primary_offset = global i32 -1

@control_stack = global [40 x i32]  zeroinitializer, align 4
@control_offset = global i32 -1

;; general utility functions

define void @print_int(i32 %val) {
    call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @int_str, i64 0, i64 0), i32 %val)
    ret void
}

define void @print_stack() {
    call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([8 x i8], [8 x i8]* @stack_str, i64 0, i64 0))
    %arr = alloca [40 x i32], align 16
    %i = alloca i32, align 4
    store i32 0, i32* %i, align 4
    %stack_offset = load i32, i32* @primary_offset
    %stack_size = add i32 %stack_offset, 1
    br label %for.cond

for.cond:
  %x = load i32, i32* %i, align 4
  %cmp = icmp slt i32 %x, %stack_size ; 40 is length of stack
  br i1 %cmp, label %for.body, label %for.end

for.body:
    ; print stack value at i
    %i. = load i32, i32* %i, align 4
    %ptr = getelementptr [40 x i32], i32* @primary_stack, i32 0, i32 %i.
    %val = load i32, i32* %ptr
    call void @print_int(i32 %val)

    ; increment i
    %i.0 = load i32, i32* %i, align 4
    %i.1 = add nsw i32 %i.0, 1
    store i32 %i.1, i32* %i, align 4
    br label %for.cond

for.end:
    call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([3 x i8], [3 x i8]* @newline_str, i64 0, i64 0))
    ret void
}

define void @debug(i32 %x) {
    call void @print_stack()
    call void @print_int(i32 %x)
    ret void
}

define void @unimplemented() {
    call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([15 x i8], [15 x i8]* @unimplemented_str, i64 0, i64 0))
    call void @exit(i32 1)
    unreachable
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

define i32 @peek_stack(i32 %depth) {
    %offset.0 = load i32, i32* @primary_offset
    %offset.1 = sub i32 %offset.0, %depth
    %ptr = getelementptr [40 x i32], i32* @primary_stack, i32 0, i32 %offset.1
    %val = load i32, i32* %ptr

    ret i32 %val
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

define void @toggle_control_stack() {
    %val = call i32 @pop_control_stack()
    ; check if control stack is zero or one
    %cond = icmp eq i32 %val, 0
    br i1 %cond, label %zero, label %not_zero
zero:
    call void @push_control_stack(i32 1)
    ret void
not_zero:
    call void @push_control_stack(i32 0)
    ret void
}

;; specific befreak operator impls

define void @bf_Number(i32 %num) {
    call void @debug(i32 13)
    %val.0 = call i32 @pop_stack()
    %val.1 = xor i32 %val.0, %num
    call void @push_stack(i32 %val.1)
    ret void
}

; simple stack
define void @bf_PushZero() {
    call void @debug(i32 14)
    call void @push_stack(i32 0)
    ret void
}

define void @bf_PopZero() {
    call void @debug(i32 15)
    call void @pop_stack()
    ret void
}

define void @bf_PopMainToControl() {
    call void @debug(i32 16)
    %1 = call i32 @pop_stack()
    call void @push_control_stack(i32 %1)
    ret void
}

define void @bf_PopControlToMain() {
    call void @debug(i32 17)
    %1 = call i32 @pop_control_stack()
    call void @push_stack(i32 %1)
    ret void
}

define void @bf_SwapStacks() {
    call void @debug(i32 18)
    %1 = call i32 @pop_stack()
    %2 = call i32 @pop_control_stack()
    call void @push_stack(i32 %2)
    call void @push_control_stack(i32 %1)
    ret void
}

; i/o
define void @bf_Write() {
    call void @debug(i32 19)
    %1 = call i32 @pop_stack()
    call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([3 x i8], [3 x i8]* @char_str, i64 0, i64 0), i32 %1)
    ret void
}

define void @bf_Read() {
    call void @debug(i32 20)
    call void @unimplemented()
    ret void
}

; number
define void @bf_Increment() {
    call void @debug(i32 21)
    %1 = call i32 @pop_stack()
    %2 = add i32 %1, 1
    call void @push_stack(i32 %2)
    ret void
}

define void @bf_Decrement() {
    call void @debug(i32 22)
    %1 = call i32 @pop_stack()
    %2 = sub i32 %1, 1
    call void @push_stack(i32 %2)
    ret void
}

define void @bf_Add() {
    call void @debug(i32 23)
    %1 = call i32 @pop_stack()
    %2 = call i32 @pop_stack()
    %sum = add i32 %2, %1
    call void @push_stack(i32 %sum)
    call void @push_stack(i32 %1)
    ret void
}

define void @bf_Subtract() {
    call void @debug(i32 23)
    %1 = call i32 @pop_stack()
    %2 = call i32 @pop_stack()
    %sum = sub i32 %2, %1
    call void @push_stack(i32 %sum)
    call void @push_stack(i32 %1)
    ret void
}

define void @bf_Divide() {
    call void @debug(i32 24)
    %x = call i32 @pop_stack()
    %y = call i32 @pop_stack()
    %div = sdiv i32 %y, %x
    %rem = srem i32 %y, %x
    call void @push_stack(i32 %div)
    call void @push_stack(i32 %rem)
    call void @push_stack(i32 %x)
    ret void
}

define void @bf_Multiply() {
    call void @debug(i32 25)
    %x = call i32 @pop_stack()
    %rem = call i32 @pop_stack()
    %div = call i32 @pop_stack()
    %y.0 = mul i32 %x, %div
    %y.1 = add i32 %y.0, %rem
    call void @push_stack(i32 %y.1)
    call void @push_stack(i32 %x)
    ret void
}

; bitwise
define void @bf_Not() {
    call void @debug(i32 26)
    %1 = call i32 @pop_stack()
    %2 = xor i32 %1, -1 ; the docs say this is okay :)
    call void @push_stack(i32 %2)
    ret void
}

define void @bf_And() {
    call void @debug(i32 27)
    call void @unimplemented()
    ret void
}

define void @bf_Or() {
    call void @debug(i32 28)
    call void @unimplemented()
    ret void
}

define void @bf_Xor() {
    call void @debug(i32 29)
    call void @unimplemented()
    ret void
}

define void @bf_RotateLeft() {
    call void @debug(i32 30)
    %x = call i32 @pop_stack()
    %y = call i32 @pop_stack()
    %y.1 = call i32 @llvm.fshl.i32(i32 %y, i32 %y, i32 %x)
    call void @push_stack(i32 %y.1)
    call void @push_stack(i32 %x)
    ret void
}

define void @bf_RotateRight() {
    call void @debug(i32 31)
    %x = call i32 @pop_stack()
    %y = call i32 @pop_stack()
    %y.1 = call i32 @llvm.fshr.i32(i32 %y, i32 %y, i32 %x)
    call void @push_stack(i32 %y.1)
    call void @push_stack(i32 %x)
    ret void
}

; comparisons
define void @bf_ToggleControl() {
    call void @debug(i32 32)
    call void @unimplemented()
    ret void
}

define void @bf_EqualityCheck() {
    call void @debug(i32 33)
    %x = call i32 @peek_stack(i32 0)
    %y = call i32 @peek_stack(i32 1)
    %cond = icmp eq i32 %y, %x
    br i1 %cond, label %equal, label %not_equal
equal:
    call void @toggle_control_stack()
    ret void
not_equal:
    ret void
}

define void @bf_LessThanCheck() {
    call void @debug(i32 34)
    %x = call i32 @peek_stack(i32 0)
    %y = call i32 @peek_stack(i32 1)
    %cond = icmp slt i32 %y, %x
    br i1 %cond, label %equal, label %not_equal
equal:
    call void @toggle_control_stack()
    ret void
not_equal:
    ret void
}

define void @bf_GreaterThanCheck() {
    call void @debug(i32 35)
    %x = call i32 @peek_stack(i32 0)
    %y = call i32 @peek_stack(i32 1)
    %cond = icmp sgt i32 %y, %x
    br i1 %cond, label %equal, label %not_equal
equal:
    call void @toggle_control_stack()
    ret void
not_equal:
    ret void
}

; complex stack
define void @bf_SwapTop() {
    call void @debug(i32 36)
    %1 = call i32 @pop_stack()
    %2 = call i32 @pop_stack()
    call void @push_stack(i32 %1)
    call void @push_stack(i32 %2)
    ret void
}

define void @bf_Dig() {
    call void @debug(i32 37)
    %x = call i32 @pop_stack();
    %y = call i32 @pop_stack();
    %z = call i32 @pop_stack();
    call void @push_stack(i32 %y)
    call void @push_stack(i32 %x)
    call void @push_stack(i32 %z)
    ret void
}

define void @bf_Bury() {
    call void @debug(i32 38)
    %x = call i32 @pop_stack();
    %y = call i32 @pop_stack();
    %z = call i32 @pop_stack();
    call void @push_stack(i32 %x)
    call void @push_stack(i32 %z)
    call void @push_stack(i32 %y)
    ret void
}

define void @bf_Flip() {
    call void @debug(i32 39)
    call void @unimplemented()
    ret void
}

define void @bf_SwapLower() {
    call void @debug(i32 40)
    %x = call i32 @pop_stack();
    %y = call i32 @pop_stack();
    %z = call i32 @pop_stack();
    call void @push_stack(i32 %y)
    call void @push_stack(i32 %z)
    call void @push_stack(i32 %x)
    ret void
}

define void @bf_Over() {
    call void @debug(i32 41)
    %x = call i32 @pop_stack();
    %y = call i32 @pop_stack();
    call void @push_stack(i32 %y)
    call void @push_stack(i32 %x)
    call void @push_stack(i32 %y)
    ret void
}

define void @bf_Under() {
    call void @debug(i32 42)
    %y.0 = call i32 @pop_stack();
    %x = call i32 @pop_stack();
    ; assumes y.1 = y
    %y.1 = call i32 @pop_stack();
    call void @push_stack(i32 %y.1)
    call void @push_stack(i32 %x)
    ret void
}

; misc
define void @bf_Duplicate() {
    call void @debug(i32 43)
    ; assumes top two are same
    %x = call i32 @pop_stack()
    call void @push_stack(i32 %x)
    call void @push_stack(i32 %x)
    ret void
}

define void @bf_Unduplicate() {
    call void @debug(i32 44)
    call void @pop_stack()
    ret void
}

define void @bf_Halt() {
    call void @debug(i32 45)
    call void @exit(i32 0)
    unreachable
}

;; actual codegen begin

"#;

fn compile(data: ExpressionTree) {
    let mut llvm_ir = String::from(PRELUDE);
    for (identifier, expression) in data.tree {
        let mut epilogue = String::new();
        write!(
            llvm_ir,
            "define void {}() {{",
            identifier.to_codegen_function_name()
        )
        .unwrap();
        for operator in expression.arr {
            operator_to_llvm_ir(&mut llvm_ir, &mut epilogue, operator);
        }
        match expression.next {
            Branches::None => llvm_ir.push_str("\n      ret void"),
            Branches::One(id1) => {
                write!(
                    llvm_ir,
                    "    call void {}()",
                    id1.to_codegen_function_name()
                )
                .unwrap();
                llvm_ir.push_str("\n    ret void");
            }
            Branches::Two(id1, id2) => {
                llvm_ir.push_str(
                    "\n
    %cond = call i1 @pop_control_stack_i1()
    br i1 %cond, label %branch_1, label %branch_0\n",
                );
                write!(
                    llvm_ir,
                    "branch_1:\n    call void {}()\n    ret void\n",
                    id1.to_codegen_function_name()
                )
                .unwrap();
                write!(
                    llvm_ir,
                    "branch_0:\n    call void {}()\n    ret void",
                    id2.to_codegen_function_name()
                )
                .unwrap();
            }
        }
        llvm_ir.push_str("\n}\n");
        llvm_ir.push_str(&epilogue);
        llvm_ir.push('\n');
    }

    write!(
        llvm_ir,
        "
;; actual codegen over

define void @main() {{
    call void {}()
    ret void
}}",
        data.start.to_codegen_function_name()
    )
    .unwrap();
    println!("{llvm_ir}");
}

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
    let start_pos = get_start_pos(code).unwrap().step(Direction::East);
    let mut data = ExpressionTree {
        tree: HashMap::new(),
        start: ExpressionIdentifier {
            position: start_pos,
            direction: Direction::East,
            inverse_mode: false,
        },
    };
    parse_expression(code, start_pos, Direction::East, false, &mut data);
    data
}

fn print_tree(tree: &ExpressionTree) {
    for (identifier, expression) in &tree.tree {
        println!("\nid: {identifier:?}");
        println!("expression: {expression:?}");
    }
    println!("starts at {:?}", tree.start);
}

#[allow(unused_variables)]
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
    let data2 = r#"@(())"#;
    let data2 = r#"
/"Hello world!"01\
\(13v     'wsv)@(/
    \(=13=13)/    
"#;
    let data2 = "
(     
     3
@((1[<
     4
(          
";
    let data = r#"
    /2)@(2\         
    >)2=2(<         
    \'(v?)/         
       s            
       (            
       1            
       >(1=1\       
       )            
       1    o       
       {    *       
       1    b       
       (    l       
       >)u%d/       
       c            
       >b'%s(= \    
     ^ >dc=c*s)/    
     >=<            
     d              
     (              
/s'0v^?w23(v`s]:(48\
[   (      )       +
)   =      =       4
0   c      c       8
1   =      =       )
%   )      (       w
\01(^      ^)01*01(/"#;
    //let data = "@((123(512/";

    let code = read_string(data);
    //println!("{code:?}");
    let tree = parse_code(&code);
    //print_tree(&tree);
    compile(tree);
}
