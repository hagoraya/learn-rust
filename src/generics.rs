pub fn run() {
    //* Enums with generics
    enum Option<T> {
        Some(T),
        None,
    };

    // *Structs with generics
    struct Point<T> {
        x: T,
        y: T,
    }

    struct DiffPoint<T, U> {
        x: T,
        y: U,
    }

    // * functions with generics
    fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
        let mut largest = &list[0];
        for item in list {
            if item > largest {
                largest = item;
            }
        }
        largest
    }

    let list = &vec![1, 2, 3];
}
