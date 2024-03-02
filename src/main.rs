fn main() {
    // In general, the `{}` will be automatically replaced with any
    // arguments. These will be stringified.
    println!("{} days", 31);
    println!("Today is {}", "Saturday");

    // Positional arguments can be used. Specifying an integer inside `{}`
    // determines which additional argument will be replaced. Arguments start
    // at 0 immediately after the format string.
    println!("\n\nPositional arguments: ");
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
    // Show the name in alphabet order
    println!("{1}, {0}", "Bob", "Alice");

    println!("\n\nNamed arguments: ");
    // As can named arguments.
    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );

    println!("\n\nDifferent formatting: ");
    // Different formatting can be invoked by specifying the format character
    // after a `:`.
    println!("Base 10:               {}", 69420); // 69420
    println!("Base 2 (binary):       {:b}", 69420); // 10000111100101100
    println!("Base 8 (octal):        {:o}", 69420); // 207454
    println!("Base 16 (hexadecimal): {:x}", 69420); // 10f2c
    println!("Base 16 (hexadecimal): {:X}", 69420); // 10F2C

    println!("\n\nJustifying a text to Right/Left by a specific character and width: e.g. {{str_value:padding_character>number_of_padding}}");
    // You can right-justify text with a specified width. This will
    // output "    1". (Four white spaces and a "1", for a total width of 5.)
    println!("{number:>5}", number = 1); // 00001
    println!("{number:0>6}", number = 1); // 000001
    println!("{number:0<6}", number = 1); // 100000
    println!("{number:#>6}", number = 123); // ###123
    println!("{:#>6}", "123"); // ###123
    println!("{string:#<6}", string = "ABC"); // ABC###
    println!("{:#<6}", "ABC"); // ABC###
    println!("{1:#>6} && {0:#<6}", "ABC", "123"); // ###123 && ABC###

    // You can use named arguments in the format specifier by appending a `$`.
    println!("{number:0>width$}", number = 123, width = 6); // 000123
    println!("{number:#>width$}", number = "ABC", width = 6); // ###ABC
    println!("{0:#>width$}", "ABC", width = 6); // ###ABC
    println!("{:#>width$}", "ABC", width = 6); // ###ABC

    // Rust even checks to make sure the correct number of arguments are used.
    println!("My name is {0}, {1} {0}", "James", "Bond");
    // FIXME ^ Add the missing argument: "James"

    // Only types that implement fmt::Display can be formatted with `{}`. User-
    // defined types do not implement fmt::Display by default.

    #[allow(dead_code)] // disable `dead_code` which warn against unused module
    struct Structure(i32);

    // This will not compile because `Structure` does not implement
    // fmt::Display.
    // println!("This struct `{}` won't print...", Structure(3));
    // TODO ^ Try uncommenting this line

    // For Rust 1.58 and above, you can directly capture the argument from a
    // surrounding variable. Just like the above, this will output
    // "    1", 4 white spaces and a "1".
    let number1: f64 = 1.0; // is equivalent to 1
    let number2: f64 = 1.01230; // is equivalent 1.0123
    let width: usize = 12;
    println!("{number1}{number2:#>width$}"); // 1######1.0123
}
