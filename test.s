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
define void @bf_cg_1_1_E_false() {
call void @bf_PushZero()
call void @bf_PushZero()
%cond = call i1 @pop_control_stack_i1()
br i1 %cond, label %branch_true, label %branch_false
branch_true:
call void @bf_cg_2_2_W_false()
ret void
branch_false:
call void @bf_cg_4_2_E_false()
call void @print_int(i32 21)
ret void
}
define void @bf_cg_2_2_W_false() {
call void @bf_PopZero()
call void @bf_PopZero()
ret void
}
define void @bf_cg_4_2_E_false() {
call void @bf_PopZero()
call void @bf_PopZero()
ret void
}

;; actual codegen over

define void @main() {
    call void @bf_cg_1_1_E_false()
    ret void
}
