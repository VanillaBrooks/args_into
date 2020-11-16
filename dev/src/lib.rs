use args_into::args_into;

#[args_into]
pub(crate) fn generic_function(string_input: String) {
    use_string(string_input);
}

fn use_string(_x: String) {}

struct Foo;

impl Foo {
    #[args_into]
    fn foo_method<'a>(self, input: &'a mut String) -> usize {
        input.insert_str(0, "some string operation");
        input.len()
    }
}

#[args_into]
pub fn print_details(first_name: String, last_name: String, age: usize) {
    println!(
        "The first name is {}, the last name is {}, and the age is {}",
        first_name, last_name, age
    )
}
