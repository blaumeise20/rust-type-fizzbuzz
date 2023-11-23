//! A minified version of type level FizzBuzz, just for fun.

type FizzBuzzUntil100=<<<I<I<I<I<I<J>>>>>as M<I<I<I<I<I<J>>>>>>>::R as M<I<I<I<I<J>>>>>>::R as F<W>>::R;struct J;struct I<N>(N);struct P;struct Q;struct V<H,T>(H,T);struct W;struct E;struct G;struct H;struct K<N>(N);trait A<Y> {type R;}impl<Y>A<Y>for J{type R=Y;}impl<X,Y>A<Y>for I<X>where X:A<Y>{type R=I<<X as A<Y>>::R>;}trait S<Y>{type R;}impl<Y>S<Y>for J{type R=J;}impl<X>S<J>for I<X>{type R=I<X>;}impl<X,Y>S<I<Y>>for I<X>where X:S<Y>{type R=<X as S<Y>>::R;}trait M<Y>{type R;}impl<Y>M<Y>for J{type R=J;}impl<X>M<J>for I<X>{type R=J;}impl<X,Y>M<I<Y>>for I<X>where I<X>:M<Y>,<I<X>as M<Y>>::R:A<I<X>>{type R=<<I<X>as M<Y>>::R as A<I<X>>>::R;}trait D<N>{type R;}impl<N>D<N>for J{type R=P;}impl<X,N>D<N>for I<X>where I<I<X>>:S<N>,I<X>:S<N>,N:C<<I<I<X>>as S<N>>::R,<I<X>as S<N>>::R>{type R=<N as C<<I<I<X>>as S<N>>::R,<I<X>as S<N>>::R>>::R;}trait C<Y,Z>{type R;}impl<N>C<I<J>,J>for N{type R=P;}impl<N>C<J,J>for N{type R=Q;}impl<N,Y,Z>C<I<Y>,I<Z>>for N where I<Z>:D<N>{type R=<I<Z>as D<N>>::R;}trait F<L>{type R;}impl<L>F<L>for J{type R=L;}impl<X,L>F<L>for I<X>where X:F<V<<I<X>as B<<I<X>as D<I<I<I<I<I<J>>>>>>>::R,<I<X>as D<I<I<I<J>>>>>::R>>::R,L>>,I<X>:B<<I<X>as D<I<I<I<I<I<J>>>>>>>::R,<I<X>as D<I<I<I<J>>>>>::R>,I<X>:D<I<I<I<I<I<J>>>>>>,I<X>:D<I<I<I<J>>>>{type R=<X as F<V<<I<X>as B<<I<X>as D<I<I<I<I<I<J>>>>>>>::R,<I<X>as D<I<I<I<J>>>>>::R>>::R,L>>>::R;}trait B<X,Y>{type R;}impl<N>B<P,P>for N{type R=H;}impl<N>B<Q,P>for N{type R=E;}impl<N>B<P,Q>for N{type R=G;}impl<N>B<Q,Q>for N{type R=K<N>;}

trait Value<O> {
    fn value() -> O;
}
impl Value<u32> for J {
    fn value() -> u32 { 0 }
}
impl<N: Value<u32>> Value<u32> for I<N> {
    fn value() -> u32 { 1 + N::value() }
}
impl<H: Value<E>, T: Value<Vec<E>>, E> Value<Vec<E>> for V<H, T> {
    fn value() -> Vec<E> {
        let mut v = T::value();
        v.insert(0, H::value());
        v
    }
}
impl<E> Value<Vec<E>> for W {
    fn value() -> Vec<E> { vec![] }
}
impl Value<String> for E {
    fn value() -> String { "Fizz".to_string() }
}
impl Value<String> for G {
    fn value() -> String { "Buzz".to_string() }
}
impl Value<String> for H {
    fn value() -> String { "FizzBuzz".to_string() }
}
impl<N: Value<u32>> Value<String> for K<N> {
    fn value() -> String { N::value().to_string() }
}

#[allow(unused)]
fn main() {
    println!("{:?}", FizzBuzzUntil100::value());
}
