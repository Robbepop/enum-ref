| Continuous Integration |  Documentation   |      Crates.io       |
|:----------------------:|:----------------:|:--------------------:|
| [![ci][1]][2]          | [![docs][3]][4] | [![crates][5]][6]  |

[1]: https://github.com/Robbepop/enum-ref/actions/workflows/rust.yml/badge.svg
[2]: https://github.com/Robbepop/enum-ref/actions/workflows/rust.yml
[3]: https://docs.rs/enum-ref/badge.svg
[4]: https://docs.rs/enum-ref
[5]: https://img.shields.io/crates/v/enum-ref.svg
[6]: https://crates.io/crates/enum-ref

# `#[derive(EnumRef)]` and `#[derive(EnumMut)]`

This crate provides the `#[derive(EnumRef)]` and `#[derive(EnumMut)]` proc. macros
that generate reference wrappers for Rust `enum` types.
Deriving `EnumRef` or `EnumMut` will also implement the respective trait which
allows to access and instantiate the generated reference wrappers.

## Motivation

The generated reference types are a bit different from default references to Rust `enum` instances
in that only the `enum` data is a reference while the `enum` discriminant remains inline.
When there is a need for having `enum` reference wrappers this crate is especially useful
for when a user has `enum` types with a big number of variants that would make it labor
intense to maintain a mirroring between original `enum` type and reference wrapper.

This might even yield a performance improvement if users mostly are interested in
querying the `enum` discriminant. However, performance is not the primary use case
of this crate.

My personal motivation for this crate is to allow for more space efficient `enum` encodings.
Usually `enum` instances are encoded with an aligned discriminant and all variants share the
same `size_of` with the biggest `enum` variant.
This has the downside that the default Rust `enum` encoding potentially wastes a lot of space.
When trying to encode Rust `enum` instances space-efficiently we still want to access the encoded
`enum` instances, however we cannot use normal references to them since those references assume
the aligned `enum` encoding which won't be the case.
This is where our new reference wrapper types come into play since we can use them for our new
encoding.

## Usage

Below we demonstrate how to use the proc. macros provided by this crate.

```rust
#[derive(EnumRef, EnumMut)]
#[repr(u8)] // Rust requires this for `B = 42`
enum Test {
    A,
    B = 42,
    C(i32),
    D(i32, i64),
    E { a: i32 },
    F { a: i32, b: i64 },
}

// Access and name the generated `enum` reference wrapper types as follows:
type TestRef<'a> = <Test as EnumRef>::Ref<'a>;
type TestMut<'a> = <Test as EnumMut>::Mut<'a>;

// Creates reference wrappers of `enum` instances as follows:
let test = Test::C(42);
let test_ref: TestRef = <Test as EnumRef>::as_ref(&test);
match (&test, test_ref) {
    (Test::C(a0), TestRef::C(a1)) => assert_eq!(a0, a1),
    _ => panic!("something wen't wrong ..."),
}
```

## Generated Code

The above `#[derive(EnumRef)]` for example will generate roughly the following Rust code:

```rust
const _: () = {
    #[derive(::core::fmt::Debug)]
    #[repr(u8)]
    pub enum TestRef<'__enum_ref_lt> {
        A,
        B = 42,
        C(&'__enum_ref_lt i32),
        D(&'__enum_ref_lt i32, &'__enum_ref_lt i64),
        E {
            a: &'__enum_ref_lt i32,
        },
        F {
            a: &'__enum_ref_lt i32,
            b: &'__enum_ref_lt i64,
        },
    }

    impl ::enum_ref::EnumRef for Test {
        type Ref<'__enum_ref_lt> where Self: '__enum_ref_lt =
                TestRef<'__enum_ref_lt> where Self: '__enum_ref_lt;
        fn as_ref(&self) -> <Self as ::enum_ref::EnumRef>::Ref<'_> {
            type __enum_ref_EnumRef_Ref<'__enum_ref_lt> =
                <Test as ::enum_ref::EnumRef>::Ref<'__enum_ref_lt>;
            match self {
                Self::A => __enum_ref_EnumRef_Ref::A,
                Self::B => __enum_ref_EnumRef_Ref::B,
                Self::C(_0) => __enum_ref_EnumRef_Ref::C(_0),
                Self::D(_0, _1) => __enum_ref_EnumRef_Ref::D(_0, _1),
                Self::E { a } => __enum_ref_EnumRef_Ref::E { a },
                Self::F { a, b } => __enum_ref_EnumRef_Ref::F { a, b },
            }
        }
    }
};
```