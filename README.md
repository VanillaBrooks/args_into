# Args Into

This crate defines the `#[args_into]` attribute procedural macro for changing all argument types 
`T` to `V: Into<T>`.


## Use Cases

Lets say you have a function with many arguments. You want to make all of the arguments `Into<T>`
to make life easier for the consumer of your api. You could manually edit every function and 
method signature as follows:

```
pub fn print_details(first_name: String, last_name: String, age: usize) {
    println!(
        "The first name is {}, the last name is {}, and the age is {}",
        first_name, last_name, age
    )
}
```

to

```
pub fn print_details <T, V> (first_name: T, last_name: T, age: V) 
where
	T: Into<String>,
	V: Into<usize>
{
	let first_name = first_name.into();
	let last_name = last_name.into();
	let age = age.into();

    println!(
        "The first name is {}, the last name is {}, and the age is {}",
        first_name, last_name, age
    )
}
```

However, this is time consuming and makes the function signature harder to read. Instead, you 
can apply the `#[args_into]` proc macro:


```
#[args_into]
pub fn print_details(first_name: String, last_name: String, age: usize) {
    println!(
        "The first name is {}, the last name is {}, and the age is {}",
        first_name, last_name, age
    )
}
```

The following code will be automatically generated:

```
#[allow(non_camel_case_types)]
pub fn print_details<__FIRST_NAME: Into<String>, __LAST_NAME: Into<String>, __AGE: Into<usize>>(
    first_name: __FIRST_NAME,
    last_name: __LAST_NAME,
    age: __AGE,
) {
    let age = age.into();
    let last_name = last_name.into();
    let first_name = first_name.into();

    println!(
        "The first name is {}, the last name is {}, and the age is {}",
        first_name, last_name, age
    )
}
```

## Applicability

This attribute macro can be applied to **any function or method signature**. It intentionally does not
make the method `self` argument `Into` since this would be a step backwards in usability. It does
not, however, attempt to work with `struct` or `enum` members;

## Codegen Samples


### Function
```
#[args_into]
pub(crate) fn generic_function(string_input: String) {
    use_string(string_input);
}

fn use_string(_x: String) {}
```

becomes:

```
#[allow(non_camel_case_types)]
pub(crate) fn generic_function
    <__STRING_INPUT: Into<String>>
    (string_input: __STRING_INPUT) 
{
    let string_input = string_input.into();
    use_string(string_input);
}

fn use_string(_x: String) {}
```



### Method
```
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
```

becomes:

```
struct Foo;

impl Foo {
    #[allow(non_camel_case_types)]
    fn foo_method <'a, __INPUT: Into<&'a mut String>> ( self, input: __INPUT) -> usize {
        let input = input.into();
        input.insert_str(0, "some string operation");
        input.len()
    }
}
```

## Drawbacks
So far, I have discovered that you will be required to manually add lifetime constraints
when dealing with references. However, you would manually have to generate these signatures
if you were writing the code by hand.
