/* -------------------------------------------------------------------
Closures and Iterators
Lecture 8
CS392
 */

fn main() { }

/* -------------------------------------------------------------------
Closures are anonymous functions, like in OCaml (except not really).
 */

// fn main() {
//     fn  square (x : i32) -> i32 { x * x }
//     let square_cls = |x| { x * x };
//     assert_eq!(square(2), 4);
//     assert_eq!(square_cls(2), 4);
// }

/* -------------------------------------------------------------------
The big difference is that closures can capture variables, which can
sometimes be difficult to reason about.
 */

// fn main() {
//     let v : Vec<i32> = vec![1, 2, 3];
//     let f = |a: i32| {
//         v[0] + a
//     };
//     assert_eq!(f(10), 11);
//     assert_eq!(v[0], 1);
// }

/* -------------------------------------------------------------------
The most common use-case of closures in functional patters like
iterators.
 */

// fn main() {
//     let v: Vec<i32> = vec![1, 2, 3, 4, 5];
//     for s in v.into_iter().map(|x| x * x) {
//         print!("{s}")
//     }
// }

/* -------------------------------------------------------------------
Here's a simple example of a counter. Note that the types get a bit
wonky.
 */

// fn mk_counter() -> impl FnMut() -> i32 {
//     let mut count = 0;
//     return move || { count += 1; count }
// }
// fn main() {
//     let mut f = mk_counter();
//     assert_eq!(f(), 1);
//     assert_eq!(f(), 2);
// }

/* -------------------------------------------------------------------
Rust does type inference for closures, but closures must be
monomorphic.
 */

// fn main() {
//     let v = vec![1, 2, 3];
//     let w = vec![1, 2, 3];
//     let id = |x| x;
//     assert_eq!(id(v), w);
//     // let x = id(2);
// }

/* -------------------------------------------------------------------
Rust does some borrow inference, in the sense that it checks how a
 */

// fn main() {
//     let mut v = vec![1, 2, 3, 4, 5];
//     let mut f = || { v.push(6) };
//     v.push(8);
//     f();
// }

// struct Foo<'a> {
//     bar: &'a mut Vec<i32>,
// }

// fn main() {
//     let mut v = vec![1, 2, 3, 4, 5];
//     let borrow = &mut v;
//     // let foo = Foo { bar: borrow };
//     let mut foo = || { borrow.push(6); borrow[0] };
//     assert_eq!(foo(), 1);
//     println!("{}", borrow[0]);
//     v.clear();
// }

/* -------------------------------------------------------------------
If a closure implements `FnOnce` then it cannot be called a second
time. The call moves the value.
 */

// struct Foo {
//     bar: Vec<i32>
// }

// fn main() {
//     let v = vec![1, 2, 3];

// }

/* -------------------------------------------------------------------
There are three kinds of closures.
 */

// fn main() {
//     let mut v = vec![1, 2, 3];
//     // let f = || v; // FnOnce only
//     //let f = || v.push(4); // Not Fn
//     let f = || println!("{}", v[0]); // All three
// }

/* -------------------------------------------------------------------
We don't really have higher order functions
 */

// fn curry<X, S, T, U, V, W>(f : T) -> impl Fn(U) -> impl Fn(V) -> W where
//     T : Fn(U, V) -> W,
//     S : Fn(U) -> X,
//     X : Fn(V) -> W,
// {
//     |x| { |y| f(x, y) }
// }

// fn uncurry<S, T, U, V, W>(f : T) -> impl FnOnce(U, V) -> W
// where
//     T : FnOnce(U) -> S,
//     S : FnOnce(V) -> W,
// {
//     |x, y| f(x)(y)
// }

// fn main() {
//     let foo = |x| { move |y| x + y };
//     let bar = uncurry(foo);
//     assert_eq!(bar(1, 2), 3)
// }

// fn curry<T, U, V, W>(f : T) -> impl FnOnce(U) -> impl FnOnce(V) -> W
// where
//     T : FnOnce(U, V) -> W,
// {
//     todo!()
// }

// fn uncurry<S, T, U, V, W>(f : T) -> impl FnOnce(U, V) -> W
// where
//     T : FnOnce(U) -> S,
//     S : FnOnce(V) -> W,
// {
//     |x, y| { f(x)(y) }
// }
//  fn main() {

// }

/* -------------------------------------------------------------------
Basic iterator example
 */

// fn main() {
//     (0..5).flat_map(|x| x * 100 .. x * 110)
//         .enumerate()
//         .filter(|&(i, x)| (i + x) % 3 == 0)
//         .for_each(|(i, x)| println!("{i}:{x}"));
// }

/* -------------------------------------------------------------------
A more interesting example
 */

// struct TripZip<U, V, W> {
//     into_iter1: U,
//     into_iter2: V,
//     into_iter3: W,
// }

// impl<U, V, W> Iterator for TripZip<U, V, W>
// where
//     U : Iterator,
//     V : Iterator,
//     W : Iterator,
// {
//     type Item = (U::Item, V::Item, W::Item);

//     fn next(&mut self) -> Option<Self::Item> {

//         Some((
//             self.into_iter1.next()?,
//             self.into_iter2.next()?,
//             self.into_iter3.next()?,
//         ))
//     }
// }

// impl<U, V, W> TripZip<U, V, W> {
//     fn new(
//         into_iter1: U,
//         into_iter2: V,
//         into_iter3: W,
//     ) -> TripZip<U, V, W> {
//         TripZip { into_iter1, into_iter2, into_iter3 }
//     }
// }
// fn main() {
//     let v1 = vec![1, 2, 3];
//     let v2 = vec![4, 5, 6];
//     let v3 = vec![7, 8, 9];
//     for (x, y, z) in TripZip::new(
//         v1.into_iter(),
//         v2.into_iter(),
//         v3.into_iter()) {
//         println!("{} {} {}", x, y, z)
//     }
// }

/* -------------------------------------------------------------------
Another interesting example
 */

// use std::slice::Iter;
// use std::vec::IntoIter;
// use std::iter::Rev;

// struct GapBuffer<T> {
//     front: Vec<T>,
//     back: Vec<T>,
// }

// struct GapBufferIter<'a, T> {
//     front_iter: Iter<'a, T>,
//     back_iter: Rev<Iter<'a, T>>,
// }

// impl<'a, T> Iterator for GapBufferIter<'a, T> {
//     type Item = &'a T;

//     fn next(&mut self) -> Option<&'a T> {
//         match self.front_iter.next() {
//             None => self.back_iter.next(),
//             Some(v) => Some(v),
//         }
//     }
// }

// impl<T> GapBuffer<T> {
//     fn new() -> GapBuffer<T> {
//         GapBuffer {
//             front: Vec::new(),
//             back: Vec::new(),
//         }
//     }

//     fn iter(&self) -> GapBufferIter<'_, T> {
//         GapBufferIter {
//             front_iter: self.front.iter(),
//             back_iter: self.back.iter().rev(),
//         }
//     }
// }

// struct GapBufferIntoIter<T> {
//     front_iter: IntoIter<T>,
//     back_iter: Rev<IntoIter<T>>,
// }

// impl<T> Iterator for GapBufferIntoIter<T> {
//     type Item = T;

//     fn next(&mut self) -> Option<T> {
//         match self.front_iter.next() {
//             None => self.back_iter.next(),
//             Some(v) => Some(v),
//         }
//     }
// }

// impl<T> IntoIterator for GapBuffer<T> {
//     type Item = T;
//     type IntoIter = GapBufferIntoIter<T>;

//     fn into_iter(self) -> GapBufferIntoIter<T> {
//         GapBufferIntoIter {
//             front_iter: self.front.into_iter(),
//             back_iter: self.back.into_iter().rev(),
//         }
//     }
// }

// impl<'a, T> IntoIterator for &'a GapBuffer<T> {
//     type Item = &'a T;
//     type IntoIter = GapBufferIter<'a, T>;

//     fn into_iter(self) -> GapBufferIter<'a, T> {
//         self.iter()
//     }

// }

// fn main() {
//     let mut b = GapBuffer::new();
//     b.front = vec![1, 2, 3];
//     b.back = vec![4, 5, 6];
//     for x in &b {
//         println!("{}", x);
//     }
//     for x in b {
//         println!("{}", x);
//     }
//     // for x in b {
//     //     println!("{}", x);
//     // }
// }

// use std::iter::repeat_n;

// struct Matrix<T> {
//     shape: (usize, usize),
//     data: Vec<T>,
// }

// impl Matrix<f64> {
//     fn zeros(shape: (usize, usize)) -> Matrix<f64> {
//         Matrix {
//             shape,
//             data: repeat_n(0.0, shape.0 * shape.1).collect(),
//         }
//     }
// }

// struct MatrixIntoIter<T> {
//     into_iter: std::vec::IntoIter<T>,
// }

// impl<T> Iterator for MatrixIntoIter<T> {
//     type Item = T;

//     fn next(&mut self) -> Option<T> {
//         self.into_iter.next()
//     }
// }

// impl<T> Matrix<T> {
//     fn into_iter(self) -> MatrixIntoIter<T> {
//         MatrixIntoIter { into_iter: self.data.into_iter() }
//     }
// }

// fn main() {
//     let mut a = Matrix::zeros((3, 3));
//     a.data[5] = 1.0;
//     for entry in a.into_iter() {
//         println!("{}", entry)
//     }

// }
