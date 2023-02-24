use structe_test;

struct Configuration<'a> {
    name: &'a str
}

fn run_core(mut configuration: Configuration) -> () {
    println!("{name} is up", name=configuration.name);
    return;
}

fn main() -> () {
    structe_test::test();
    // This is just for testing abstraction
    let testConfiguration = Configuration {
        name: "config test"
    };
    run_core(testConfiguration);
    return;
}