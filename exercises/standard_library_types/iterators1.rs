// iterators1.rs
// 
//  Make me compile by filling in the `???`s
//
// When performing operations on elements within a collection, iterators are essential.
// This module helps you get familiar with the structure of using an iterator and 
// how to go through elements within an iterable collection.
// 
// Execute `rustlings hint iterators1` for hints :D

fn main () {
    let my_fav_fruits = vec!["banana", "custard apple", "avocado", "peach", "raspberry"];

    let mut my_iterable_fav_fruits = my_fav_fruits.iter();

    println!("{:?}", my_iterable_fav_fruits);
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"banana"));
    println!("{:?}", my_iterable_fav_fruits);
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"custard apple"));
    println!("{:?}", my_iterable_fav_fruits);
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"avocado"));
    println!("{:?}", my_iterable_fav_fruits);
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"peach"));
    println!("{:?}", my_iterable_fav_fruits);
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"raspberry"));
    println!("{:?}", my_iterable_fav_fruits);
    assert_eq!(my_iterable_fav_fruits.next(), None);
}
