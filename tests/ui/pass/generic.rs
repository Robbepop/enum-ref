use enum_ref::{EnumRef, EnumMut};

#[derive(EnumRef, EnumMut)]
enum Test<'a, T, const N: usize> {
    Unit,
    Ref(&'a T),
    Mut(&'a mut T),
    Value(T),
    NamedValue { a: T },
    Bytes([u8; N]),
}

type TestRef<'lt, 'a, T, const N: usize> = <Test<'a, T, N> as EnumRef>::Ref<'a>;
type TestMut<'lt, 'a, T, const N: usize> = <Test<'a, T, N> as EnumMut>::Mut<'a>;

fn main() {}
