trait Foo {
    fn foo(&self, f: impl Fn() -> ());
    fn foo_twice(&self, f: impl Fn() -> () + Copy) {
        self.foo(f);
        self.foo(f)
    }
}

trait DynFoo {
    fn dyn_foo(&self, f: &dyn Fn() -> ());
}

impl<F: Foo> DynFoo for F {
    fn dyn_foo(&self, f: &dyn Fn() -> ()) {
        self.foo(f)
    }
}

macro_rules! impl_dyn_with_markers {
    ($($marker:ident),*) => {
        impl<'a> Foo for dyn DynFoo + 'a$( + $marker)* {
            fn foo(&self, f: impl Fn() -> ()) {
                self.dyn_foo(&f)
            }
        }
    };
}
impl_dyn_with_markers! {}
impl_dyn_with_markers! {Send}
impl_dyn_with_markers! {Sync}
impl_dyn_with_markers! {Send, Sync}

use std::cell::Cell;
struct FooImpl {
    cnt: Cell<u32>,
}
impl Foo for FooImpl {
    fn foo(&self, _: impl Fn() -> ()) {
        self.cnt.set(self.cnt.get() + 1)
    }
}
impl FooImpl {
    fn new() -> FooImpl {
        FooImpl { cnt: Cell::new(0) }
    }
}

#[test]
fn test_simple() {
    let foo = FooImpl::new();
    foo.foo(&|| {});
    assert_eq!(foo.cnt.get(), 1);
    foo.foo_twice(&|| {});
    assert_eq!(foo.cnt.get(), 3);
    let dyn_foo: &dyn DynFoo = &foo;
    dyn_foo.dyn_foo(&|| {});
    assert_eq!(foo.cnt.get(), 4);
    dyn_foo.foo_twice(&|| {});
    assert_eq!(foo.cnt.get(), 6);
}

use std::rc::Rc;
#[test]
fn test_complex<'a>() {
    let foo = Rc::new(FooImpl::new());
    foo.foo(&|| {});
    assert_eq!(foo.cnt.get(), 1);
    foo.foo_twice(&|| {});
    assert_eq!(foo.cnt.get(), 3);
    let foo_cloned = Rc::clone(&foo);
    let dyn_foo: Rc<dyn DynFoo + Send + 'a> = foo_cloned;
    dyn_foo.dyn_foo(&|| {});
    assert_eq!(foo.cnt.get(), 4);
    dyn_foo.foo_twice(&|| {});
    assert_eq!(foo.cnt.get(), 6);
}
