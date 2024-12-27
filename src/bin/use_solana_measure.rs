use solana_measure::*;

fn foo() {
    println!("foo");
}

fn bar(x: i32) {
    println!("bar: {}", x);
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn main(){
    // Measure functions
    let (result, measure) = measure_time!(foo(), "foo takes no parameters");
    println!("foo takes no parameters: {:?}", measure);
    let (result, measure) = measure_time!(bar(42), "bar takes one parameter");
    println!("bar takes one parameter: {:?}", measure);
    let (result, measure) = measure_time!(add(1, 2), "add takes two parameters and returns a value");
    println!("add takes two parameters and returns a value: {:?}", measure);
    let (result, measure_us) = measure_us!(add(1, 2));
    println!("add takes two parameters and returns a value: {:?}", measure_us);
    let (result, duration) = meas_dur!(add(1, 2));
    println!("add takes two parameters and returns a value: {:?}", duration);
}