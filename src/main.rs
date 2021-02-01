use std::collections::HashMap;
use std::str::from_utf8;
use std::thread;
use std::time::Duration;

fn main() {
    struct Cacher<T>
    where
        T: Fn(u32) -> u32,
    {
        calculation: T,
        value: Option<u32>,
    }

    impl<T> Cacher<T>
    where
        T: Fn(u32) -> u32,
    {
        fn new(calculation: T) -> Cacher<T> {
            Cacher {
                calculation,
                value: None,
            }
        }

        fn value(&mut self, arg: u32) -> u32 {
            match self.value {
                Some(v) => v,
                None => {
                    let v = (self.calculation)(arg);
                    self.value = Some(v);
                    v
                }
            }
        }
    }

    let mut cacher = Cacher::new(|num| {
        println!("Calculating slowly...");
        thread::sleep(Duration::from_millis(300));
        num
    });

    println!(
        "At this point, cacher has no value in cacher.value.
              Only when we call cacher.value will the 'expensive' calculation be run"
    );

    cacher.value(5);
    println!("Now the calculation has been run and the value is set. Further calls to cacher.value will only return the value 'cached', and not rerun the calculation. This is done through the match statement in the value getter. the value of cacher.value is {}", cacher.value(3));

    println!("(should print nothing below)");
    cacher.value(55);

    println!("The problem is that we cannot set the cached value again. We can fix this by modifying cacher to hold a hash map rather than a single value. the keys of the hashmap with be the arg values that are passed in, and the values will be the result of calling the closure on that key.");

    struct Cacher2<T>
    where
        T: Fn(u32) -> u32,
    {
        calculation: T,
        values: HashMap<u32, u32>,
    }

    impl<T> Cacher2<T>
    where
        T: Fn(u32) -> u32,
    {
        // store the closure in the calculation field as before.
        // instead of having one corresponding field, have a hashmap of K:V pairs
        fn new(calculation: T) -> Cacher2<T> {
            Cacher2 {
                calculation,
                values: HashMap::new(),
            }
        }

        fn value(&mut self, arg: u32) -> u32 {
            match self.values.contains_key(&arg) {
                true => self.values[&arg],
                false => {
                    self.values.insert(arg, (self.calculation)(arg));
                    self.values[&arg]
                }
            }
        }
    }

    // this closure is not run yet.. only stored in the calculation field
    let mut cacher_2 = Cacher2::new(|num| {
        println!("Some super cool long calculation is running... Hopefully only once per key!");
        thread::sleep(Duration::from_millis(300));
        num * 3
    });
    // corresponding value for 1 does not exist yet
    cacher_2.value(1); // now it does, and calculation has been run
                       // since calculation has been and value is set, calling value again will only get that value and not re-run calculation
    cacher_2.value(1);

    // also possible to insert more values because of HashMap data structure

    cacher_2.value(2);
    println!("cacher_2.value(2) is: {}", cacher_2.value(2));
    println!("cacher_2.value(1) remains: {}", cacher_2.value(1));

    struct Cacher3<T, U, V>
    where
        T: Fn(U) -> V,
        U: std::hash::Hash + Eq + Copy,
        V: Copy,
    {
        calculation: T,
        values: HashMap<U, V>,
    }

    impl<T, U, V> Cacher3<T, U, V>
    where
        T: Fn(U) -> V, // type T takes parameter of type U and stores the calculation run from the closure in type V
        U: std::hash::Hash + Eq + Copy,
        V: Copy,
    {
        fn new(calculation: T) -> Cacher3<T, U, V> {
            Cacher3 {
                calculation,
                values: HashMap::new(),
            }
        }

        fn value(&mut self, arg: U) -> V {
            match self.values.contains_key(&arg) {
                true => self.values[&arg],
                false => {
                    self.values.insert(arg, (self.calculation)(arg));
                    self.values[&arg]
                }
            }
        }
    }

    let mut cacher_3 = Cacher3::new(|arg: u32| {
        println!("{} is the best number!", arg);
        // we can return any type that implements copy, as specified by the trait bounds
        arg.is_power_of_two(); // bool type!
    });

    cacher_3.value(22);

    println!(
        "value stored for key '22' in cacher_3 is: {:#?}",
        cacher_3.value(22)
    );

    let mut cacher_3_num_2 = Cacher3::new(|arg: &[u8]| {
        println!("Converting arg: {:?} into &str", arg);
        if let Ok(arg) = std::str::from_utf8(&arg) {
            println!("{}", arg);
            Ok(())
        } else {
            Err("cannot parse to str")
        }
    });

    cacher_3_num_2
        .value("Hello, World!".as_bytes())
        .unwrap_or_else(|err| {
            println!("Err is {}", err);
        });
}
