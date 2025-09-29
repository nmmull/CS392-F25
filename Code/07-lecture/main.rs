/* -------------------------------------------------------------------
Generics and Traits
Lecture 7
CAS CS 392
Fall 2025
 */

fn main() { }

/* -------------------------------------------------------------------
Generic types allow us to write parametrically polymorphic functions
 */

// fn is_singleton<T>(l: &[T]) -> bool {
//     l.len() == 1
// }

// fn main() {
//     let v = vec![42];
//     assert!(is_singleton(&v))
// }

/* -------------------------------------------------------------------
Generic type can be used in lots of places, including structures and
enumerations
 */

// type ('a, 'b) result = Ok of 'a | Err of 'b

// #[derive(PartialEq, Debug)]
// enum MyResult<T, E> {
//     Ok(T),
//     Err(E),
// }

// fn main() {
//     let x : MyResult<i32, i32> = MyResult::Err(0);
//     assert_eq!(x, MyResult::Err(0));
// }

/* -------------------------------------------------------------------
They can even be used for methods and impl blocks
 */

// struct Point<T> {x : T, y : T}

// impl<T> Point<T> {
//     fn x_coo(&self) -> &T { &self.x }
//     fn y_coo(&self) -> &T { &self.y }
// }

// fn main() {
//     let p = Point {x: 0, y: 1};
//     assert_eq!(*p.x_coo(), 0);
//     assert_eq!(*p.y_coo(), 1);
// }

/* -------------------------------------------------------------------
A neat trick: we can concretize the type we use in an impl block to
give implementations specific to certain types
 */

// struct Point<T> {x : T, y : T}

// impl Point<f32> {
//     fn norm(&self) -> f32 {
//         (self.x * self.x + self.y * self.y).sqrt()
//     }
// }

// fn main() {
//     let p: Point<f32> = Point { x: 3., y: 4. };
//     assert_eq!(p.norm(), 5.)
// }

/* -------------------------------------------------------------------
But not all functions can be completely generic. Especially in Rust...
 */

// fn reverse<T: Copy>(l: &mut [T]) {
//     let mut temp: T;
//     for i in 0..(l.len() / 2) {
//         temp = l[i];
//         l[i] = l[l.len() - i - 1];
//         l[l.len() - i - 1] = temp;
//     }
// }

// fn main() {
//     let mut v = vec![1,2,3,4,5];
//     reverse(&mut v);
//     println!("{:?}", v)
// }

/* -------------------------------------------------------------------
Traits give us the ability to define an INTERFACE for types

Which allows us to restrict the kind of type used in a given setting
 */

// fn sum<T>(l: &[T]) -> T
// {
//     let mut out = 0;
//     for i in l.iter() {
//         out += i;
//     }
//     out
// }

// fn main() {
//     let v = vec![1, 2, 3, 4, 5];
//     assert_eq!(sum(&v), 15);
// }

/* -------------------------------------------------------------------
We can define our own traits by specifying a set of functions that we
want a type to define
 */

// trait Semiring {                        // A semiring has an addition
//     fn add(&mut self, rhs: &Self);      // operator and a zero
//     fn zero() -> Self;
// }

// fn sum<T: Semiring>(l : &[T]) -> T {    // ": Semiring" is called a
//     let mut out: T = Semiring::zero();  // TRAIT BOUND

//     for i in l.iter() {
//         out.add(i);
//     }
//     out
// }

// impl Semiring for i32 {                 // We can implement the semiring
//     fn add(&mut self, rhs: &i32) {      // trait for i32
//         *self += rhs;
//     }
//     fn zero() -> i32 { 0 }
// }

// fn main() {
//     let v = vec![1, 2, 3, 4, 5];
//     assert_eq!(sum(&v), 15);
// }

/* -------------------------------------------------------------------
Again, we have to be careful with Rust...
 */

// fn reverse(l: &mut [i32]) {
//     let mut temp: i32;
//     for i in 0..(l.len() / 2) {
//         temp = l[i];
//         l[i] = l[l.len() - i - 1];
//         l[l.len() - i - 1] = temp;
//     }
// }

// fn main() {
//     let mut v = vec![1,2,3,4,5];
//     reverse(&mut v);
//     println!("{:?}", v)
// }

/* -------------------------------------------------------------------
Derived Traits give us "obvious" implementations
 */

// #[derive(Clone, Copy, Debug, PartialEq)]
// struct Point {x: Vec<i32>, y: f32}

// fn main() { }
// fn main() {
//     let mut p1 = Point {x: 1., y: 1.};
//     let mut p2 = p1;
//     p1.x = 2.;
//     p2.y = 2.;
//     assert_eq!(p1.x, p2.y);
//     assert_eq!(p1.y, p2.x);
//     let p3 = Point {x: 1., y: 2.};
//     assert_eq!(p2, p3);
// }

/* -------------------------------------------------------------------
where syntax gives us more space to specify trait bounds
 */

// use std::ops::Add;

// fn foo<S, T, U>(x: S, y: &T) -> U
// where
//     S : Add<Output = U> + From<T>,
//     T : Clone,
// {
//     x + S::from(y.clone())
// }

// fn main() {
//     assert_eq!(foo(2, &3), 5);
// }

/* -------------------------------------------------------------------
All most everything that "happens in the background" in Rust can be
controlled via the Trait system

+ Copy
+ Clone
+ Display
+ Debug
+ PartialEq
+ PartialOrd
+ Iterator
+ IntoIterator
+ From
+ Into
+ TryFrom
+ TryInto
+ FromStr
+ Add, AddAssign, Sub, SubAssign,...
+ FnOnce, Fn, FnMut
 */

/* -------------------------------------------------------------------
One of the most complicated trait is the Deref trait

This controls the Deref operator *
 */

// fn add_one(l : &mut [i32]) {
//     for i in l.iter_mut() {
//         *i += 1;
//     }
// }



// fn main() {
//     let mut v : Vec<i32> = vec![1, 2, 3];
//     add_one(&mut v);
//     assert_eq!(v, vec![2, 3, 4]);
// }

/* -------------------------------------------------------------------
There are three interesting rules which come along with this:

+ *v implicitely becomes *deref(&v) (if v is not already a reference).
+ &T is coerced to &U
+ T implicitly implements all methods of U which take &self

This gives us cool things like the NEWTYPE PATTERN
 */

// use std::ops::Deref;

// struct ExprM<T> {
//     meta: T,
//     expr: Expr<T>,
// }

// enum Expr<T> {
//     Num(i32),
//     Add(Box<ExprM<T>>, Box<ExprM<T>>),
// }

// impl<T> Deref for ExprM<T> {
//     type Target = Expr<T>;

//     fn deref(&self) -> &Expr<T> {
//         &self.expr
//     }
// }

// fn eval<T>(e : &ExprM<T>) -> i32 {
//     match &e.expr {
//         Expr::Num(n) => *n,
//         Expr::Add(e1, e2) => eval(e1) + eval(e2),
//     }
// }

// fn main() {
//     let e = ExprM {
//         meta: 1,
//         expr: Expr::Add(
//             Box::new(ExprM {
//                 meta: 2,
//                 expr: Expr::Num(45),
//             }),
//             Box::new(ExprM {
//                 meta: 3,
//                 expr: Expr::Num(33),
//             }),
//         ),
//     };
//     println!("{}", e.meta);
//     println!("{}", eval(&e));
// }
