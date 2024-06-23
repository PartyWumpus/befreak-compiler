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

define void @push_stack(i32 %value) {
    ; increment pointer by one
    call void @increment_stack(i32 1)

    ; put value onto the stack at pointer
    %offset = load i32, i32* @primary_offset
    %ptr = getelementptr [40 x i32], i32* @primary_stack, i32 0, i32 %offset
    store i32 %value, ptr %ptr

    ret void
}

define void @push_control_stack(i32 %value) {
    ; increment pointer by one
    call void @increment_control_stack(i32 1)

    ; put value onto the stack at pointer
    %offset = load i32, i32* @control_offset
    %ptr = getelementptr [40 x i32], i32* @control_stack, i32 0, i32 %offset
    store i32 %value, ptr %ptr

    ret void
}

define i32 @pop_stack() {
    ; get value from the stack at pointer
    %offset = load i32, i32* @primary_offset
    %ptr = getelementptr [40 x i32], i32* @primary_stack, i32 0, i32 %offset
    %value = load i32, i32* %ptr

    ; decrement pointer by one
    call void @increment_stack(i32 -1)

    ret i32 %value
}

define i32 @pop_control_stack() {
    ; get value from the stack at pointer
    %offset = load i32, i32* @control_offset
    %ptr = getelementptr [40 x i32], i32* @control_stack, i32 0, i32 %offset
    %value = load i32, i32* %ptr

    ; decrement pointer by one
    call void @increment_control_stack(i32 -1)

    ret i32 %value
}

;; specific befreak operator impls

define void @bf_PushZero() {
    call void @push_stack(i32 0)
    ret void
}

define void @bf_PopZero() {
    call void @pop_stack()
    ret void
}

;; actual codegen begin
define void @bf_cg_1_0_E_false() {
call void @bf_PushZero()
call void @bf_PushZero()
call void @bf_PopZero()
call void @bf_PopZero()
ret void
}
;; actual codegen over

define void @main() {
    call void @bf_cg_1_0_E_false()
    ret void
}
