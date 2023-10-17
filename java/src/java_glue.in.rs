use crate::jni_c_header::*;

pub struct Foo {
    val: i32
}

impl Foo {
    pub fn new(val: i32) -> Self {
        Self {
            val
        }
    }

    pub fn setField(&mut self, new_val: i32) {
        self.val = new_val;
    }

    pub fn val(&self) -> i32 {
        self.val
    }
}

foreign_class!(class Foo {
    self_type Foo;
    constructor Foo::new(_: i32) -> Foo;
    fn Foo::setField(&mut self, _: i32);
    fn Foo::val(&self) -> i32;
    foreign_code r#"
    static {
        try {
            NativeUtils.loadLibraryFromJar("/engine_java.dll");
        } catch (java.io.IOException e) {
            e.printStackTrace();
        }
    }
"#;
});
