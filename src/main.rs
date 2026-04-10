use rust_project::{
    abstractions, basics, collections, concurrency, iterators_demo, macros_demo, models, ownership,
    text,
};

fn main() {
    basics::run();
    ownership::run();
    let user = models::run();
    collections::run();
    iterators_demo::run();
    text::run();
    abstractions::run(&user);
    concurrency::run();
    macros_demo::run(&user);
}
