# id-wrapper

A simple generic wrapper type to add an ID field to any struct.

## Quick Start

```rust
use id_wrapper::prelude::*;

#[derive(Clone, Debug)]
struct MyStruct {
    pub count: usize,
}

#[generate_overwrites]
impl MyStruct {
    #[skip]
    pub fn increment(&mut self) {
        self.count += 1;
    }

    pub fn increment_by(&mut self, amount: usize) {
        self.count += amount;
    }
}

impl MyStructOverwrites for WithId<MyStruct> {
    fn increment_by(&mut self, amount: usize) {
        println!(
            "OVERWRITTEN: Incrementing id '{}' by amount: {amount}",
            self.id()
        );
        self.inner.increment_by(amount);
    }
}

fn main() {
    // Structs can be converted to WithId<_> through `From`/`Into`
    let mut my_struct: WithId<_> = MyStruct { count: 0 }.into();
    println!("Initial Value:");
    println!("{my_struct:#?}");
    println!();

    // WithId automatically dereferences to its inner value
    my_struct.count += 1;
    println!("Updating the value directly through DerefMut:");
    println!("{my_struct:#?}");
    println!();

    // This also works for function calls
    my_struct.increment();
    println!("Updating by calling a function of the inner type:");
    println!("{my_struct:#?}");
    println!();

    // This function was overwritten by creating an impl for `WithId<MyStruct>`
    // and implementing a function with the same name as the original
    my_struct.increment_by(3);
    println!("Updating by calling the 'overwritten' function:");
    println!("{my_struct:#?}");
    println!();

    // The original function can still be accessed by manually dereferencing
    (*my_struct).increment_by(5);
    println!("Updating by calling the 'original' function, by dereferencing first:");
    println!("{my_struct:#?}");
    println!();

    // cloning assigns a new id to the clone
    let my_other_struct = my_struct.clone();
    println!("New id after cloning:");
    println!("{my_other_struct:#?}");
    assert_ne!(my_struct.id(), my_other_struct.id());
}
```

If you only want to overwrite a small number of functions,
you can use `#[generate_overwrites(all = false)]` and then explicitly include the functions you want with `#[overwrite]`.

```rust
#[generate_overwrites(all = false)]
impl MyStruct {
    pub fn increment(&mut self) {
        self.count += 1;
    }

    #[overwrite]
    pub fn increment_by(&mut self, amount: usize) {
        self.count += amount;
    }
}
```
