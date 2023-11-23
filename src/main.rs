//! Type level FizzBuzz implemented using traits.

#![allow(non_camel_case_types)]

mod minified;

struct Zero;
struct Inc<N>(N);

trait Add<Y> {
    type Result;
}
impl<Y> Add<Y> for Zero {
    type Result = Y;
}
impl<X, Y> Add<Y> for Inc<X> where X: Add<Y> {
    type Result = Inc<<X as Add<Y>>::Result>;
}

trait Sub<Y> {
    type Result;
}
impl<Y> Sub<Y> for Zero {
    type Result = Zero;
}
impl<X> Sub<Zero> for Inc<X> {
    type Result = Inc<X>;
}
impl<X, Y> Sub<Inc<Y>> for Inc<X> where X: Sub<Y> {
    type Result = <X as Sub<Y>>::Result;
}

trait Mul<Y> {
    type Result;
}
impl<Y> Mul<Y> for Zero {
    type Result = Zero;
}
impl<X> Mul<Zero> for Inc<X> {
    type Result = Zero;
}
impl<X, Y> Mul<Inc<Y>> for Inc<X> where Inc<X>: Mul<Y>, <Inc<X> as Mul<Y>>::Result: Add<Inc<X>> {
    type Result = <<Inc<X> as Mul<Y>>::Result as Add<Inc<X>>>::Result;
}

trait DivisibleBy<D> {
    type Result;
}
impl<D> DivisibleBy<D> for Zero {
    type Result = True;
}
impl<X, D> DivisibleBy<D> for Inc<X> where Inc<Inc<X>>: Sub<D>, Inc<X>: Sub<D>, D: TryDivide<<Inc<Inc<X>> as Sub<D>>::Result, <Inc<X> as Sub<D>>::Result> {
    type Result = <D as TryDivide<<Inc<Inc<X>> as Sub<D>>::Result, <Inc<X> as Sub<D>>::Result>>::Result;
}

trait TryDivide<Y, Z> {
    type Result;
}
impl<D> TryDivide<Inc<Zero>, Zero> for D {
    type Result = True;
}
impl<D> TryDivide<Zero, Zero> for D {
    type Result = False;
}
impl<D, Y, Z> TryDivide<Inc<Y>, Inc<Z>> for D where Inc<Z>: DivisibleBy<D> {
    type Result = <Inc<Z> as DivisibleBy<D>>::Result;
}

struct True;
struct False;

type N_2 = Inc<Inc<Zero>>;
type N_3 = Inc<N_2>;
type N_5 = Inc<Inc<N_3>>;
type N_10 = <N_5 as Mul<N_2>>::Result;
type N_100 = <N_10 as Mul<N_10>>::Result;

struct Cons<H, T>(H, T);
struct Nil;

trait FizzBuzz<L> {
    type Result;
}
impl<L> FizzBuzz<L> for Zero {
    type Result = L;
}
impl<X, L> FizzBuzz<L> for Inc<X> where X: FizzBuzz<Cons<<Inc<X> as FizzBuzzStep<<Inc<X> as DivisibleBy<N_5>>::Result, <Inc<X> as DivisibleBy<N_3>>::Result>>::Result, L>>, Inc<X>: FizzBuzzStep<<Inc<X> as DivisibleBy<N_5>>::Result, <Inc<X> as DivisibleBy<N_3>>::Result>, Inc<X>: DivisibleBy<N_5>, Inc<X>: DivisibleBy<N_3> {
    type Result = <X as FizzBuzz<Cons<<Inc<X> as FizzBuzzStep<<Inc<X> as DivisibleBy<N_5>>::Result, <Inc<X> as DivisibleBy<N_3>>::Result>>::Result, L>>>::Result;
}

trait FizzBuzzStep<X, Y> {
    type Result;
}
impl<N> FizzBuzzStep<True, True> for N {
    type Result = ResFizzBuzz;
}
impl<N> FizzBuzzStep<False, True> for N {
    type Result = ResFizz;
}
impl<N> FizzBuzzStep<True, False> for N {
    type Result = ResBuzz;
}
impl<N> FizzBuzzStep<False, False> for N {
    type Result = ResNum<N>;
}

struct ResFizz;
struct ResBuzz;
struct ResFizzBuzz;
struct ResNum<N>(N);


/// Converts a type-level value to a runtime value. Does not do any computation.
/// ```
/// assert_eq!(Zero::value(), 0);
/// assert_eq!(Inc::<Inc<Zero>>::value(), 2);
/// assert_eq!(ResFizz::value(), "Fizz".to_string());
/// assert_eq!(Cons::<ResFizz, Cons<ResBuzz, Cons<ResNum<Inc<Zero>>, Nil>>>::value(), vec!["Fizz".to_string(), "Buzz".to_string(), "1".to_string()]);
/// ```
trait Value<O> {
    fn value() -> O;
}
impl Value<u32> for Zero {
    fn value() -> u32 { 0 }
}
impl<N: Value<u32>> Value<u32> for Inc<N> {
    fn value() -> u32 { 1 + N::value() }
}
impl<H: Value<E>, T: Value<Vec<E>>, E> Value<Vec<E>> for Cons<H, T> {
    fn value() -> Vec<E> {
        let mut v = T::value();
        v.insert(0, H::value());
        v
    }
}
impl<E> Value<Vec<E>> for Nil {
    fn value() -> Vec<E> { vec![] }
}
impl Value<String> for ResFizz {
    fn value() -> String { "Fizz".to_string() }
}
impl Value<String> for ResBuzz {
    fn value() -> String { "Buzz".to_string() }
}
impl Value<String> for ResFizzBuzz {
    fn value() -> String { "FizzBuzz".to_string() }
}
impl<N: Value<u32>> Value<String> for ResNum<N> {
    fn value() -> String { N::value().to_string() }
}

fn main() {
    println!("{:?}", <N_100 as FizzBuzz<Nil>>::Result::value());
}
