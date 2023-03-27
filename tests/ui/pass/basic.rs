use enum_ref::{EnumRef, EnumMut};

#[derive(Clone, EnumRef, EnumMut)]
#[repr(u8)]
enum Test {
    A,
    B = 42,
    C(i32),
    D(i32, i64),
    E { a: i32 },
    F { a: i32, b: i64 },
    Ref,
}

type TestRef<'a> = <Test as EnumRef>::Ref<'a>;
type TestMut<'a> = <Test as EnumMut>::Mut<'a>;

fn test_ref(test: &Test) {
    let tref = <Test as EnumRef>::as_ref(test);
    match (test, tref) {
        (Test::A, TestRef::A) |
        (Test::B, TestRef::B) => {},
        (Test::C(a0), TestRef::C(a1)) => {
            let a1: &i32 = a1; // test proper types
            assert_eq!(a0, a1)
        },
        (Test::D(a0, b0), TestRef::D(a1, b1)) => {
            let a1: &i32 = a1; // test proper types
            let b1: &i64 = b1; // test proper types
            assert_eq!(a0, a1);
            assert_eq!(b0, b1);
        },
        (Test::E { a: a0 }, TestRef::E { a: a1 }) => {
            let a1: &i32 = a1; // test proper types
            assert_eq!(a0, a1)
        },
        (Test::F { a: a0, b: b0 }, TestRef::F { a: a1, b: b1 }) => {
            let a1: &i32 = a1; // test proper types
            let b1: &i64 = b1; // test proper types
            assert_eq!(a0, a1);
            assert_eq!(b0, b1);
        },
        _ => panic!(),
    }
}

fn test_mut(test: &Test) {
    let mut test0 = test.clone();
    let mut test1 = test.clone();
    let tmut = <Test as EnumMut>::as_mut(&mut test1);
    match (&mut test0, tmut) {
        (Test::A, TestMut::A) |
        (Test::B, TestMut::B) => {},
        (Test::C(a0), TestMut::C(a1)) => {
            let a1: &mut i32 = a1; // test proper types
            assert_eq!(a0, a1)
        },
        (Test::D(a0, b0), TestMut::D(a1, b1)) => {
            let a1: &mut i32 = a1; // test proper types
            let b1: &mut i64 = b1; // test proper types
            assert_eq!(a0, a1);
            assert_eq!(b0, b1);
        },
        (Test::E { a: a0 }, TestMut::E { a: a1 }) => {
            let a1: &mut i32 = a1; // test proper types
            assert_eq!(a0, a1)
        },
        (Test::F { a: a0, b: b0 }, TestMut::F { a: a1, b: b1 }) => {
            let a1: &mut i32 = a1; // test proper types
            let b1: &mut i64 = b1; // test proper types
            assert_eq!(a0, a1);
            assert_eq!(b0, b1);
        },
        _ => panic!(),
    }
}

fn test_for(test: Test) {
    test_ref(&test);
    test_mut(&test);
}

fn main() {
    test_for(Test::A);
    test_for(Test::B);
    test_for(Test::C(0));
    test_for(Test::D(1, 2));
    test_for(Test::E { a: 3 });
    test_for(Test::F { a: 4, b: 5 });
}
