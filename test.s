;; globals
@int_str = private unnamed_addr constant [4 x i8] c"%d\0A\00", align 1
@char_str = private unnamed_addr constant [3 x i8] c"%c\00", align 1

declare dso_local i32 @printf(i8*, ...) #1
declare dso_local void @exit(i32) #2

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
  %arr = alloca [40 x i32], align 16
  %i = alloca i32, align 4
  store i32 0, i32* %i, align 4
  br label %for.cond

for.cond:
  %1 = load i32, i32* %i, align 4
  %cmp = icmp slt i32 %1, 40 ; 40 is length of stack
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

define i32 @peek_stack(i32 %depth) {
    ; get val from the stack at pointer
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
    %val.0 = call i32 @pop_stack()
    %val.1 = xor i32 %val.0, %num
    call void @push_stack(i32 %val.1)
    ret void
}

define void @bf_String() {
ret void
}

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
    %1 = call i32 @pop_stack()
    call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([3 x i8], [3 x i8]* @char_str, i64 0, i64 0), i32 %1)
    ret void
}
define void @bf_Read() {
ret void
}

; number
define void @bf_Increment() {
    %1 = call i32 @pop_stack()
    %2 = add i32 %1, 1
    call void @push_stack(i32 %2)
    ret void
}
define void @bf_Decrement() {
    %1 = call i32 @pop_stack()
    %2 = sub i32 %1, 1
    call void @push_stack(i32 %2)
    ret void
}
define void @bf_Add() {
    call void @print_int(i32 22)
    ret void
}
define void @bf_Subtract() {
    call void @print_int(i32 23)
    ret void
}
define void @bf_Divide() {
    call void @print_int(i32 24)
    ret void
}
define void @bf_Multiply() {
    call void @print_int(i32 25)
    ret void
}

; bitwise
define void @bf_Not() {
    call void @print_int(i32 26)
    ret void
}
define void @bf_And() {
    call void @print_int(i32 27)
    ret void
}
define void @bf_Or() {
    call void @print_int(i32 28)
    ret void
}
define void @bf_Xor() {
    call void @print_int(i32 29)
    ret void
}
define void @bf_RotateLeft() {
    call void @print_int(i32 30)
    ret void
}
define void @bf_RotateRight() {
    call void @print_int(i32 31)
    ret void
}

; comparisons
define void @bf_ToggleControl() {
    call void @print_int(i32 32)
    ret void
}
define void @bf_EqualityCheck() {
    %1 = call i32 @peek_stack(i32 0)
    %2 = call i32 @peek_stack(i32 1)
    %cond = icmp eq i32 %1, %2
    br i1 %cond, label %equal, label %not_equal
equal:
    call void @toggle_control_stack()
    ret void
not_equal:
    ret void
}
define void @bf_LessThanCheck() {
    call void @print_int(i32 34)
    ret void
}
define void @bf_GreaterThanCheck() {
    call void @print_int(i32 35)
    ret void
}

; complex stack
define void @bf_SwapTop() {
    %1 = call i32 @pop_stack()
    %2 = call i32 @pop_stack()
    call void @push_stack(i32 %1)
    call void @push_stack(i32 %2)
    ret void
}
define void @bf_Dig() {
    call void @print_int(i32 37)
    ret void
}
define void @bf_Bury() {
    call void @print_int(i32 38)
    ret void
}
define void @bf_Flip() {
    call void @print_int(i32 39)
ret void
}
define void @bf_SwapLower() {
    call void @print_int(i32 40)
    ret void
}
define void @bf_Over() {
    call void @print_int(i32 41)
    ret void
}
define void @bf_Under() {
    call void @print_int(i32 42)
    ret void
}

; misc
define void @bf_Duplicate() {
    call void @print_int(i32 43)
    ret void
}
define void @bf_Unduplicate() {
    call void @print_int(i32 44)
    ret void
}
define void @bf_InverseMode() {
    call void @print_int(i32 45)
    ret void
}
define void @bf_Halt() {
    call void @exit(i32 0)
    unreachable
}

;; actual codegen begin

define void @bf_cg_14_2_E_false() {
    call void @bf_PopZero()
      ret void
}

define void @bf_cg_16_2_E_false() {
    call void @bf_PushZero()
    call void @bf_Number(i32 10)
    ; STRING CODE BEGIN
    call void @increment_stack(i32 1)

    ; paste string onto the stack
    %offset = load i32, i32* @primary_offset
    %ptr = getelementptr [40 x i32], i32* @primary_stack, i32 0, i32 %offset
    %str = load [12 x i32], i32* @wasd
    store [12 x i32] %str, ptr %ptr

    call void @increment_stack(i32 11) ; len - 1
    ; STRING CODE END

    call void @bf_PushZero()
    call void @bf_Number(i32 13)
    call void @push_control_stack(i32 1)
    call void @bf_PushZero()
    call void @bf_EqualityCheck()
    call void @bf_Number(i32 13)
    call void @bf_EqualityCheck()
    call void @bf_Number(i32 13)
    call void @bf_PopZero()

    %cond = call i1 @pop_control_stack_i1()
    br i1 %cond, label %branch_true, label %branch_false
branch_true:
    call void @bf_cg_14_2_E_false()
    ret void
branch_false:
    call void @bf_cg_12_2_W_false()
    ret void
}

@wasd = private unnamed_addr constant [12 x i32] [i32 33, i32 100, i32 108, i32 114, i32 111, i32 119, i32 32, i32 111, i32 108, i32 108, i32 101, i32 72], align 4

define void @bf_cg_12_2_W_false() {
    call void @bf_SwapTop()
    call void @bf_Write()
    call void @bf_Increment()
    call void @push_control_stack(i32 0)
    call void @bf_PushZero()
    call void @bf_EqualityCheck()
    call void @bf_Number(i32 13)
    call void @bf_EqualityCheck()
    call void @bf_Number(i32 13)
    call void @bf_PopZero()

    %cond = call i1 @pop_control_stack_i1()
    br i1 %cond, label %branch_true, label %branch_false
branch_true:
    call void @bf_cg_14_2_E_false()
    ret void
branch_false:
    call void @bf_cg_12_2_W_false()
    ret void
}


;; actual codegen over

define void @main() {
    call void @bf_cg_16_2_E_false()
    ret void
}
