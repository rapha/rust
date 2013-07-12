// xfail-fast
// aux-build:trait_default_method_xc_aux.rs

#[allow(default_methods)];

extern mod aux(name = "trait_default_method_xc_aux");
use aux::{A, B, TestEquality, Something};


fn f<T: aux::A>(i: T) {
    assert_eq!(i.g(), 10);
}

mod stuff {
    pub struct thing { x: int }
}

impl A for stuff::thing {
    fn f(&self) -> int { 10 }
}

fn g<T, U, V: B<T>>(i: V, j: T, k: U) -> (T, U) {
    i.thing(j, k)
}

fn eq<T: TestEquality>(lhs: &T, rhs: &T) -> bool {
    lhs.test_eq(rhs)
}
fn neq<T: TestEquality>(lhs: &T, rhs: &T) -> bool {
    lhs.test_neq(rhs)
}


impl TestEquality for stuff::thing {
    fn test_eq(&self, rhs: &stuff::thing) -> bool {
        //self.x.test_eq(&rhs.x)
        eq(&self.x, &rhs.x)
    }
}


fn main () {
    // Some tests of random things
    f(0);

    let a = stuff::thing { x: 0 };
    let b = stuff::thing { x: 1 };
    let c = Something { x: 1 };

    assert_eq!(0i.g(), 10);
    assert_eq!(a.g(), 10);
    assert_eq!(a.h(), 10);
    assert_eq!(c.h(), 10);

    0i.thing(3.14, 1);
    assert_eq!(0i.thing(3.14, 1), (3.14, 1));

    assert_eq!(g(0i, 3.14, 1), (3.14, 1));
    assert_eq!(g(false, 3.14, 1), (3.14, 1));

    let obj = @0i as @A;
    assert_eq!(obj.h(), 10);


    // Trying out a real one
    assert!(12.test_neq(&10));
    assert!(!10.test_neq(&10));
    assert!(a.test_neq(&b));
    assert!(!a.test_neq(&a));

    assert!(neq(&12, &10));
    assert!(!neq(&10, &10));
    assert!(neq(&a, &b));
    assert!(!neq(&a, &a));
}