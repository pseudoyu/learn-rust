// 使用 include 宏来包含其他文件
include!("test.rs");

// 使用 mod 来包含其他文件
mod print;
mod variables;
mod types;
mod strings;
mod tuples;
mod arrays;
mod vectors;
mod conditionals;
mod control_flow;
mod function;
mod pointer_ref;
mod data_structure;
mod enums;
mod dag;
mod life_cycle;
mod traits;
mod complex_add;
mod dynamic_dispatching;
mod add_mod;
mod cli;

fn main() {

    // 测试 Variables
    variables::run();

    // 测试 mod
    add_mod::run();

    // 测试 Print
    print::run();

    // 测试 Types
    types::run();

    // 测试 Strings
    strings::run();

    // 测试 Tuples
    tuples::run();

    // 测试 Arrays
    arrays::run();

    // 测试 Vectors
    vectors::run();

    // 测试 Conditionals
    conditionals::run();

    // 测试 Control flow / Loops
    control_flow::run();

    //  测试 Function
    function::run();

    // 测试 Pointer & Reference
    pointer_ref::run();

    // 测试数据结构
    data_structure::run();

    // 测试枚举
    enums::run();

    //  测试 DAG 实现
    dag::run();

    // 测试生命周期
    life_cycle::run();

    // 测试 Trait
    traits::run();

    // 测试复数 Add
    complex_add::run();

    // 测试动态分派
    dynamic_dispatching::run();

    // 测试命令行
    cli::run();
}