extern crate skeptic;
#[ignore]
#[test] fn async_impl_sect_async_example_line_22() {
    let s = &format!(r####"
{}"####, r####"extern crate ext_php_rs;
extern crate php_tokio;
extern crate reqwest;
use ext_php_rs::prelude::*;
use php_tokio::{php_async_impl, EventLoop};

#[php_class]
struct Client {}

#[php_async_impl]
impl Client {
    pub fn init() -> PhpResult<u64> {
        EventLoop::init()
    }
    pub fn wakeup() -> PhpResult<()> {
        EventLoop::wakeup()
    }
    pub async fn get(url: &str) -> anyhow::Result<String> {
        Ok(reqwest::get(url).await?.text().await?)
    }
}

pub extern "C" fn request_shutdown(_type: i32, _module_number: i32) -> i32 {
    EventLoop::shutdown();
    0
}

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module.request_shutdown_function(request_shutdown)
}
"####);
    skeptic::rt::compile_test(r#"C:\Users\adria\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\ext-php-rs-0.14.2"#, r#"D:\desenvolvimento\wead\example-rust-php-ext\target\debug\build\ext-php-rs-6322a22b557eeda2\out"#, r#"x86_64-pc-windows-msvc"#, s);
}

#[test] fn exceptions_sect_throwing_exceptions_line_29() {
    let s = &format!(r####"
{}"####, r####"#![cfg_attr(windows, feature(abi_vectorcall))]
extern crate ext_php_rs;
use ext_php_rs::prelude::*;
use std::convert::TryInto;

// Trivial example - PHP represents all integers as `u64` on 64-bit systems
// so the `u32` would be converted back to `u64`, but that's okay for an example.
#[php_function]
pub fn something_fallible(n: u64) -> PhpResult<u32> {
    let n: u32 = n.try_into().map_err(|_| "Could not convert into u32")?;
    Ok(n)
}

#[php_module]
pub fn module(module: ModuleBuilder) -> ModuleBuilder {
    module
}
fn main() {}
"####);
    skeptic::rt::compile_test(r#"C:\Users\adria\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\ext-php-rs-0.14.2"#, r#"D:\desenvolvimento\wead\example-rust-php-ext\target\debug\build\ext-php-rs-6322a22b557eeda2\out"#, r#"x86_64-pc-windows-msvc"#, s);
}

#[test] fn hello_world_sect_writing_our_extension_line_67() {
    let s = &format!(r####"
{}"####, r####"#![cfg_attr(windows, feature(abi_vectorcall))]
extern crate ext_php_rs;
use ext_php_rs::prelude::*;

#[php_function]
pub fn hello_world(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module.function(wrap_function!(hello_world))
}
fn main() {}
"####);
    skeptic::rt::compile_test(r#"C:\Users\adria\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\ext-php-rs-0.14.2"#, r#"D:\desenvolvimento\wead\example-rust-php-ext\target\debug\build\ext-php-rs-6322a22b557eeda2\out"#, r#"x86_64-pc-windows-msvc"#, s);
}

#[ignore]
#[test] fn ini_builder_sect_ini_builder_line_13() {
    let s = &format!(r####"
{}"####, r####"use ext_php_rs::builder::{IniBuilder, SapiBuilder};

fn main() {
// Create a new IniBuilder instance.
let mut builder = IniBuilder::new();

// Append a single key/value pair to the INIT buffer with an unquoted value.
builder.unquoted("log_errors", "1");

// Append a single key/value pair to the INI buffer with a quoted value.
builder.quoted("default_mimetype", "text/html");

// Append INI line text as-is. A line break will be automatically appended.
builder.define("memory_limit=128MB");

// Prepend INI line text as-is. No line break insertion will occur.
builder.prepend("error_reporting=0\ndisplay_errors=1\n");

// Construct a SAPI.
let mut sapi = SapiBuilder::new("name", "pretty_name").build()
  .expect("should build SAPI");

// Dump INI entries from the builder into the SAPI.
sapi.ini_entries = builder.finish();
}
"####);
    skeptic::rt::compile_test(r#"C:\Users\adria\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\ext-php-rs-0.14.2"#, r#"D:\desenvolvimento\wead\example-rust-php-ext\target\debug\build\ext-php-rs-6322a22b557eeda2\out"#, r#"x86_64-pc-windows-msvc"#, s);
}

#[test] fn ini_settings_sect_registering_ini_settings_line_10() {
    let s = &format!(r####"
{}"####, r####"#![cfg_attr(windows, feature(abi_vectorcall))]
extern crate ext_php_rs;
use ext_php_rs::prelude::*;
use ext_php_rs::zend::IniEntryDef;
use ext_php_rs::flags::IniEntryPermission;

pub fn startup(ty: i32, mod_num: i32) -> i32 {
    let ini_entries: Vec<IniEntryDef> = vec![
        IniEntryDef::new(
            "my_extension.display_emoji".to_owned(),
            "yes".to_owned(),
            &IniEntryPermission::All,
        ),
    ];
    IniEntryDef::register(ini_entries, mod_num);

    0
}

#[php_module]
#[php(startup = "startup")]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module
}
fn main() {}
"####);
    skeptic::rt::compile_test(r#"C:\Users\adria\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\ext-php-rs-0.14.2"#, r#"D:\desenvolvimento\wead\example-rust-php-ext\target\debug\build\ext-php-rs-6322a22b557eeda2\out"#, r#"x86_64-pc-windows-msvc"#, s);
}

#[test] fn ini_settings_sect_getting_ini_settings_line_42() {
    let s = &format!(r####"
{}"####, r####"#![cfg_attr(windows, feature(abi_vectorcall))]
extern crate ext_php_rs;
use ext_php_rs::{
    prelude::*,
    zend::ExecutorGlobals,
};

pub fn startup(ty: i32, mod_num: i32) -> i32 {
    // Get all INI values
    let ini_values = ExecutorGlobals::get().ini_values(); // HashMap<String, Option<String>>
    let my_ini_value = ini_values.get("my_extension.display_emoji"); // Option<Option<String>>

    0
}

#[php_module]
#[php(startup = "startup")]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module
}
fn main() {}
"####);
    skeptic::rt::compile_test(r#"C:\Users\adria\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\ext-php-rs-0.14.2"#, r#"D:\desenvolvimento\wead\example-rust-php-ext\target\debug\build\ext-php-rs-6322a22b557eeda2\out"#, r#"x86_64-pc-windows-msvc"#, s);
}

#[test] fn classes_sect_example_line_66() {
    let s = &format!(r####"
{}"####, r####"#![cfg_attr(windows, feature(abi_vectorcall))]
extern crate ext_php_rs;
use ext_php_rs::prelude::*;

#[php_class]
pub struct Human {
    name: String,
    age: i32,
    #[php(prop)]
    address: String,
}

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module.class::<Human>()
}
fn main() {}
"####);
    skeptic::rt::compile_test(r#"C:\Users\adria\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\ext-php-rs-0.14.2"#, r#"D:\desenvolvimento\wead\example-rust-php-ext\target\debug\build\ext-php-rs-6322a22b557eeda2\out"#, r#"x86_64-pc-windows-msvc"#, s);
}

#[test] fn classes_sect_example_line_89() {
    let s = &format!(r####"
{}"####, r####"#![cfg_attr(windows, feature(abi_vectorcall))]
extern crate ext_php_rs;
use ext_php_rs::{
    prelude::*,
    exception::PhpException,
    zend::ce
};

#[php_class]
#[php(name = "Redis\\Exception\\RedisException")]
#[php(extends(ce = ce::exception, stub = "\\Exception"))]
#[derive(Default)]
pub struct RedisException;

// Throw our newly created exception
#[php_function]
pub fn throw_exception() -> PhpResult<i32> {
    Err(PhpException::from_class::<RedisException>("Not good!".into()))
}

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module
        .class::<RedisException>()
        .function(wrap_function!(throw_exception))
}
fn main() {}
"####);
    skeptic::rt::compile_test(r#"C:\Users\adria\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\ext-php-rs-0.14.2"#, r#"D:\desenvolvimento\wead\example-rust-php-ext\target\debug\build\ext-php-rs-6322a22b557eeda2\out"#, r#"x86_64-pc-windows-msvc"#, s);
}

#[test] fn classes_sect_implementing_an_interface_line_124() {
    let s = &format!(r####"
{}"####, r####"#![cfg_attr(windows, feature(abi_vectorcall))]
extern crate ext_php_rs;
use ext_php_rs::{
    prelude::*,
    exception::PhpResult,
    types::Zval,
    zend::ce,
};

#[php_class]
#[php(implements(ce = ce::arrayaccess, stub = "\\ArrayAccess"))]
#[derive(Default)]
pub struct EvenNumbersArray;

/// Returns `true` if the array offset is an even number.
/// Usage:
/// ```php
/// $arr = new EvenNumbersArray();
/// var_dump($arr[0]); // true
/// var_dump($arr[1]); // false
/// var_dump($arr[2]); // true
/// var_dump($arr[3]); // false
/// var_dump($arr[4]); // true
/// var_dump($arr[5] = true); // Fatal error:  Uncaught Exception: Setting values is not supported
/// ```
#[php_impl]
impl EvenNumbersArray {
    pub fn __construct() -> EvenNumbersArray {
        EvenNumbersArray {}
    }
    // We need to use `Zval` because ArrayAccess needs $offset to be a `mixed`
    pub fn offset_exists(&self, offset: &'_ Zval) -> bool {
        offset.is_long()
    }
    pub fn offset_get(&self, offset: &'_ Zval) -> PhpResult<bool> {
        let integer_offset = offset.long().ok_or("Expected integer offset")?;
        Ok(integer_offset % 2 == 0)
    }
    pub fn offset_set(&mut self, _offset: &'_ Zval, _value: &'_ Zval) -> PhpResult {
        Err("Setting values is not supported".into())
    }
    pub fn offset_unset(&mut self, _offset: &'_ Zval) -> PhpResult {
        Err("Setting values is not supported".into())
    }
}

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module.class::<EvenNumbersArray>()
}
fn main() {}
"####);
    skeptic::rt::compile_test(r#"C:\Users\adria\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\ext-php-rs-0.14.2"#, r#"D:\desenvolvimento\wead\example-rust-php-ext\target\debug\build\ext-php-rs-6322a22b557eeda2\out"#, r#"x86_64-pc-windows-msvc"#, s);
}

#[test] fn constant_sect_examples_line_18() {
    let s = &format!(r####"
{}"####, r####"#![cfg_attr(windows, feature(abi_vectorcall))]
extern crate ext_php_rs;
use ext_php_rs::prelude::*;

#[php_const]
const TEST_CONSTANT: i32 = 100;

#[php_const]
#[php(name = "I_AM_RENAMED")]
const TEST_CONSTANT_THE_SECOND: i32 = 42;

#[php_const]
const ANOTHER_STRING_CONST: &'static str = "Hello world!";

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module
        .constant(wrap_constant!(TEST_CONSTANT))
        .constant(wrap_constant!(TEST_CONSTANT_THE_SECOND))
        .constant(("MANUAL_CONSTANT", ANOTHER_STRING_CONST, &[]))
}
fn main() {}
"####);
    skeptic::rt::compile_test(r#"C:\Users\adria\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\ext-php-rs-0.14.2"#, r#"D:\desenvolvimento\wead\example-rust-php-ext\target\debug\build\ext-php-rs-6322a22b557eeda2\out"#, r#"x86_64-pc-windows-msvc"#, s);
}

#[test] fn extern_sect_example_line_36() {
    let s = &format!(r####"
{}"####, r####"#![cfg_attr(windows, feature(abi_vectorcall))]
extern crate ext_php_rs;
use ext_php_rs::{
    prelude::*,
    types::Zval,
};

#[php_extern]
extern "C" {
    fn strpos(haystack: &str, needle: &str, offset: Option<i64>) -> Zval;
}

#[php_function]
pub fn my_strpos() {
    assert_eq!(unsafe { strpos("Hello", "e", None) }.long(), Some(1));
}

#[php_module]
pub fn module(module: ModuleBuilder) -> ModuleBuilder {
    module.function(wrap_function!(my_strpos))
}
fn main() {}
"####);
    skeptic::rt::compile_test(r#"C:\Users\adria\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\ext-php-rs-0.14.2"#, r#"D:\desenvolvimento\wead\example-rust-php-ext\target\debug\build\ext-php-rs-6322a22b557eeda2\out"#, r#"x86_64-pc-windows-msvc"#, s);
}

#[test] fn function_sect_optional_parameters_line_16() {
    let s = &format!(r####"
{}"####, r####"#![cfg_attr(windows, feature(abi_vectorcall))]
extern crate ext_php_rs;
use ext_php_rs::prelude::*;

#[php_function]
pub fn greet(name: String, age: Option<i32>) -> String {
    let mut greeting = format!("Hello, {}!", name);

    if let Some(age) = age {
        greeting += &format!(" You are {} years old.", age);
    }

    greeting
}

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module.function(wrap_function!(greet))
}
fn main() {}
"####);
    skeptic::rt::compile_test(r#"C:\Users\adria\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\ext-php-rs-0.14.2"#, r#"D:\desenvolvimento\wead\example-rust-php-ext\target\debug\build\ext-php-rs-6322a22b557eeda2\out"#, r#"x86_64-pc-windows-msvc"#, s);
}

#[test] fn function_sect_optional_parameters_line_43() {
    let s = &format!(r####"
{}"####, r####"#![cfg_attr(windows, feature(abi_vectorcall))]
extern crate ext_php_rs;
use ext_php_rs::prelude::*;

#[php_function]
#[php(defaults(offset = 0))]
pub fn rusty_strpos(haystack: &str, needle: &str, offset: i64) -> Option<usize> {
    let haystack: String = haystack.chars().skip(offset as usize).collect();
    haystack.find(needle)
}

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module.function(wrap_function!(rusty_strpos))
}
fn main() {}
"####);
    skeptic::rt::compile_test(r#"C:\Users\adria\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\ext-php-rs-0.14.2"#, r#"D:\desenvolvimento\wead\example-rust-php-ext\target\debug\build\ext-php-rs-6322a22b557eeda2\out"#, r#"x86_64-pc-windows-msvc"#, s);
}

#[test] fn function_sect_optional_parameters_line_66() {
    let s = &format!(r####"
{}"####, r####"#![cfg_attr(windows, feature(abi_vectorcall))]
extern crate ext_php_rs;
use ext_php_rs::prelude::*;

/// `age` will be deemed required and nullable rather than optional.
#[php_function]
pub fn greet(name: String, age: Option<i32>, description: String) -> String {
    let mut greeting = format!("Hello, {}!", name);

    if let Some(age) = age {
        greeting += &format!(" You are {} years old.", age);
    }

    greeting += &format!(" {}.", description);
    greeting
}

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module.function(wrap_function!(greet))
}
fn main() {}
"####);
    skeptic::rt::compile_test(r#"C:\Users\adria\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\ext-php-rs-0.14.2"#, r#"D:\desenvolvimento\wead\example-rust-php-ext\target\debug\build\ext-php-rs-6322a22b557eeda2\out"#, r#"x86_64-pc-windows-msvc"#, s);
}

#[test] fn function_sect_optional_parameters_line_95() {
    let s = &format!(r####"
{}"####, r####"#![cfg_attr(windows, feature(abi_vectorcall))]
extern crate ext_php_rs;
use ext_php_rs::prelude::*;

/// `age` will be deemed required and nullable rather than optional,
/// while description will be optional.
#[php_function]
#[php(optional = "description")]
pub fn greet(name: String, age: Option<i32>, description: Option<String>) -> String {
    let mut greeting = format!("Hello, {}!", name);

    if let Some(age) = age {
        greeting += &format!(" You are {} years old.", age);
    }

    if let Some(description) = description {
        greeting += &format!(" {}.", description);
    }

    greeting
}

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module.function(wrap_function!(greet))
}
fn main() {}
"####);
    skeptic::rt::compile_test(r#"C:\Users\adria\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\ext-php-rs-0.14.2"#, r#"D:\desenvolvimento\wead\example-rust-php-ext\target\debug\build\ext-php-rs-6322a22b557eeda2\out"#, r#"x86_64-pc-windows-msvc"#, s);
}

#[test] fn function_sect_variadic_functions_line_131() {
    let s = &format!(r####"
{}"####, r####"#![cfg_attr(windows, feature(abi_vectorcall))]
extern crate ext_php_rs;
use ext_php_rs::{prelude::*, types::Zval};

/// This can be called from PHP as `add(1, 2, 3, 4, 5)`
#[php_function]
pub fn add(number: u32, numbers:&[&Zval]) -> u32 {
    // numbers is a slice of 4 Zvals all of type long
    number
}

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module.function(wrap_function!(add))
}
fn main() {}
"####);
    skeptic::rt::compile_test(r#"C:\Users\adria\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\ext-php-rs-0.14.2"#, r#"D:\desenvolvimento\wead\example-rust-php-ext\target\debug\build\ext-php-rs-6322a22b557eeda2\out"#, r#"x86_64-pc-windows-msvc"#, s);
}

#[test] fn impl_sect_example_line_101() {
    let s = &format!(r####"
{}"####, r####"#![cfg_attr(windows, feature(abi_vectorcall))]
extern crate ext_php_rs;
use ext_php_rs::{prelude::*, types::ZendClassObject};

#[php_class]
#[derive(Debug, Default)]
pub struct Human {
    name: String,
    age: i32,
    #[php(prop)]
    address: String,
}

#[php_impl]
impl Human {
    const MAX_AGE: i32 = 100;

    // No `#[constructor]` attribute required here - the name is `__construct`.
    pub fn __construct(name: String, age: i32) -> Self {
        Self {
            name,
            age,
            address: String::new()
        }
    }

    #[php(getter)]
    pub fn get_name(&self) -> String {
        self.name.to_string()
    }

    #[php(setter)]
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    #[php(getter)]
    pub fn get_age(&self) -> i32 {
        self.age
    }

    pub fn introduce(&self) {
        println!("My name is {} and I am {} years old. I live at {}.", self.name, self.age, self.address);
    }

    pub fn get_raw_obj(self_: &mut ZendClassObject<Human>) -> &mut ZendClassObject<Human> {
        dbg!(self_)
    }

    pub fn get_max_age() -> i32 {
        Self::MAX_AGE
    }
}
#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module.class::<Human>()
}
fn main() {}
"####);
    skeptic::rt::compile_test(r#"C:\Users\adria\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\ext-php-rs-0.14.2"#, r#"D:\desenvolvimento\wead\example-rust-php-ext\target\debug\build\ext-php-rs-6322a22b557eeda2\out"#, r#"x86_64-pc-windows-msvc"#, s);
}

#[test] fn module_sect_usage_line_25() {
    let s = &format!(r####"
{}"####, r####"#![cfg_attr(windows, feature(abi_vectorcall))]
extern crate ext_php_rs;
use ext_php_rs::{
    prelude::*,
    zend::ModuleEntry,
    info_table_start,
    info_table_row,
    info_table_end
};

#[php_const]
pub const MY_CUSTOM_CONST: &'static str = "Hello, world!";

#[php_class]
pub struct Test {
    a: i32,
    b: i32
}
#[php_function]
pub fn hello_world() -> &'static str {
    "Hello, world!"
}

/// Used by the `phpinfo()` function and when you run `php -i`.
/// This will probably be simplified with another macro eventually!
pub extern "C" fn php_module_info(_module: *mut ModuleEntry) {
    info_table_start!();
    info_table_row!("my extension", "enabled");
    info_table_end!();
}

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module
        .constant(wrap_constant!(MY_CUSTOM_CONST))
        .class::<Test>()
        .function(wrap_function!(hello_world))
        .info_function(php_module_info)
}
fn main() {}
"####);
    skeptic::rt::compile_test(r#"C:\Users\adria\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\ext-php-rs-0.14.2"#, r#"D:\desenvolvimento\wead\example-rust-php-ext\target\debug\build\ext-php-rs-6322a22b557eeda2\out"#, r#"x86_64-pc-windows-msvc"#, s);
}

#[test] fn php_sect_attributes_line_9() {
    let s = &format!(r####"
{}"####, r####"#![cfg_attr(windows, feature(abi_vectorcall))]
extern crate ext_php_rs;
use ext_php_rs::prelude::*;
#[php_function]
#[php(name = "hi_world")]
#[php(defaults(a = 1, b = 2))]
fn hello_world(a: i32, b: i32) -> i32 {
    a + b
}
fn main() {}
"####);
    skeptic::rt::compile_test(r#"C:\Users\adria\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\ext-php-rs-0.14.2"#, r#"D:\desenvolvimento\wead\example-rust-php-ext\target\debug\build\ext-php-rs-6322a22b557eeda2\out"#, r#"x86_64-pc-windows-msvc"#, s);
}

#[test] fn php_sect_attributes_line_22() {
    let s = &format!(r####"
{}"####, r####"#![cfg_attr(windows, feature(abi_vectorcall))]
extern crate ext_php_rs;
use ext_php_rs::prelude::*;
#[php_function]
#[php(name = "hi_world", defaults(a = 1, b = 2))]
fn hello_world(a: i32, b: i32) -> i32 {
    a + b
}
fn main() {}
"####);
    skeptic::rt::compile_test(r#"C:\Users\adria\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\ext-php-rs-0.14.2"#, r#"D:\desenvolvimento\wead\example-rust-php-ext\target\debug\build\ext-php-rs-6322a22b557eeda2\out"#, r#"x86_64-pc-windows-msvc"#, s);
}

#[test] fn zval_convert_sect_structs_line_16() {
    let s = &format!(r####"
{}"####, r####"#![cfg_attr(windows, feature(abi_vectorcall))]
extern crate ext_php_rs;
use ext_php_rs::prelude::*;

#[derive(ZvalConvert)]
pub struct ExampleClass<'a> {
    a: i32,
    b: String,
    c: &'a str
}

#[php_function]
pub fn take_object(obj: ExampleClass) {
    dbg!(obj.a, obj.b, obj.c);
}

#[php_function]
pub fn give_object() -> ExampleClass<'static> {
    ExampleClass {
        a: 5,
        b: "String".to_string(),
        c: "Borrowed",
    }
}

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module
        .function(wrap_function!(take_object))
        .function(wrap_function!(give_object))
}
fn main() {}
"####);
    skeptic::rt::compile_test(r#"C:\Users\adria\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\ext-php-rs-0.14.2"#, r#"D:\desenvolvimento\wead\example-rust-php-ext\target\debug\build\ext-php-rs-6322a22b557eeda2\out"#, r#"x86_64-pc-windows-msvc"#, s);
}

#[test] fn zval_convert_sect_structs_line_67() {
    let s = &format!(r####"
{}"####, r####"#![cfg_attr(windows, feature(abi_vectorcall))]
extern crate ext_php_rs;
use ext_php_rs::prelude::*;

// T must implement both `PartialEq<i32>` and `FromZval`.
#[derive(Debug, ZvalConvert)]
pub struct CompareVals<T: PartialEq<i32>> {
    a: T,
    b: T
}

#[php_function]
pub fn take_object(obj: CompareVals<i32>) {
    dbg!(obj);
}

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module
        .function(wrap_function!(take_object))
}
fn main() {}
"####);
    skeptic::rt::compile_test(r#"C:\Users\adria\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\ext-php-rs-0.14.2"#, r#"D:\desenvolvimento\wead\example-rust-php-ext\target\debug\build\ext-php-rs-6322a22b557eeda2\out"#, r#"x86_64-pc-windows-msvc"#, s);
}

#[test] fn zval_convert_sect_enums_line_112() {
    let s = &format!(r####"
{}"####, r####"#![cfg_attr(windows, feature(abi_vectorcall))]
extern crate ext_php_rs;
use ext_php_rs::prelude::*;

#[derive(Debug, ZvalConvert)]
pub enum UnionExample<'a> {
    Long(u64), // Long
    ProperStr(&'a str), // Actual string - not a converted value
    ParsedStr(String), // Potentially parsed string, i.e. a double
    None // Zval did not contain anything that could be parsed above
}

#[php_function]
pub fn test_union(val: UnionExample) {
    dbg!(val);
}

#[php_function]
pub fn give_union() -> UnionExample<'static> {
    UnionExample::Long(5)
}

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module
        .function(wrap_function!(test_union))
        .function(wrap_function!(give_union))
}
fn main() {}
"####);
    skeptic::rt::compile_test(r#"C:\Users\adria\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\ext-php-rs-0.14.2"#, r#"D:\desenvolvimento\wead\example-rust-php-ext\target\debug\build\ext-php-rs-6322a22b557eeda2\out"#, r#"x86_64-pc-windows-msvc"#, s);
}

#[test] fn binary_sect_rust_usage_line_24() {
    let s = &format!(r####"
{}"####, r####"#![cfg_attr(windows, feature(abi_vectorcall))]
extern crate ext_php_rs;
use ext_php_rs::prelude::*;
use ext_php_rs::binary::Binary;

#[php_function]
pub fn test_binary(input: Binary<u32>) -> Binary<u32> {
    for i in input.iter() {
        println!("{}", i);
    }

    vec![5, 4, 3, 2, 1]
        .into_iter()
        .collect::<Binary<_>>()
}
fn main() {}
"####);
    skeptic::rt::compile_test(r#"C:\Users\adria\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\ext-php-rs-0.14.2"#, r#"D:\desenvolvimento\wead\example-rust-php-ext\target\debug\build\ext-php-rs-6322a22b557eeda2\out"#, r#"x86_64-pc-windows-msvc"#, s);
}

#[test] fn binary_slice_sect_rust_usage_line_24() {
    let s = &format!(r####"
{}"####, r####"#![cfg_attr(windows, feature(abi_vectorcall))]
extern crate ext_php_rs;
use ext_php_rs::prelude::*;
use ext_php_rs::binary_slice::BinarySlice;

#[php_function]
pub fn test_binary_slice(input: BinarySlice<u8>) -> u8 {
    let mut sum = 0;
    for i in input.iter() {
        sum += i;
    }

    sum
}
fn main() {}
"####);
    skeptic::rt::compile_test(r#"C:\Users\adria\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\ext-php-rs-0.14.2"#, r#"D:\desenvolvimento\wead\example-rust-php-ext\target\debug\build\ext-php-rs-6322a22b557eeda2\out"#, r#"x86_64-pc-windows-msvc"#, s);
}

#[test] fn bool_sect_rust_example_line_25() {
    let s = &format!(r####"
{}"####, r####"#![cfg_attr(windows, feature(abi_vectorcall))]
extern crate ext_php_rs;
use ext_php_rs::prelude::*;
#[php_function]
pub fn test_bool(input: bool) -> String {
    if input {
        "Yes!".into()
    } else {
        "No!".into()
    }
}
fn main() {}
"####);
    skeptic::rt::compile_test(r#"C:\Users\adria\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\ext-php-rs-0.14.2"#, r#"D:\desenvolvimento\wead\example-rust-php-ext\target\debug\build\ext-php-rs-6322a22b557eeda2\out"#, r#"x86_64-pc-windows-msvc"#, s);
}

#[test] fn bool_sect_rust_example_taking_by_reference_line_51() {
    let s = &format!(r####"
{}"####, r####"#![cfg_attr(windows, feature(abi_vectorcall))]
extern crate ext_php_rs;
use ext_php_rs::prelude::*;
use ext_php_rs::types;
#[php_function]
pub fn test_bool(input: &mut types::Zval) {
    input.reference_mut().unwrap().set_bool(false);
}
fn main() {}
"####);
    skeptic::rt::compile_test(r#"C:\Users\adria\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\ext-php-rs-0.14.2"#, r#"D:\desenvolvimento\wead\example-rust-php-ext\target\debug\build\ext-php-rs-6322a22b557eeda2\out"#, r#"x86_64-pc-windows-msvc"#, s);
}

#[test] fn class_object_sect_examples_line_15() {
    let s = &format!(r####"
{}"####, r####"#![cfg_attr(windows, feature(abi_vectorcall))]
extern crate ext_php_rs;
use ext_php_rs::{prelude::*, types::ZendClassObject};

#[php_class]
pub struct Example {
    foo: i32,
    bar: i32
}

#[php_impl]
impl Example {
    // ext-php-rs treats the method as associated due to the `self_` argument.
    // The argument _must_ be called `self_`.
    pub fn builder_pattern(
        self_: &mut ZendClassObject<Example>,
    ) -> &mut ZendClassObject<Example> {
        // do something with `self_`
        self_
    }
}

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module.class::<Example>()
}
fn main() {}
"####);
    skeptic::rt::compile_test(r#"C:\Users\adria\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\ext-php-rs-0.14.2"#, r#"D:\desenvolvimento\wead\example-rust-php-ext\target\debug\build\ext-php-rs-6322a22b557eeda2\out"#, r#"x86_64-pc-windows-msvc"#, s);
}

#[test] fn class_object_sect_examples_line_47() {
    let s = &format!(r####"
{}"####, r####"#![cfg_attr(windows, feature(abi_vectorcall))]
extern crate ext_php_rs;
use ext_php_rs::prelude::*;

#[php_class]
pub struct Example {
    foo: i32,
    bar: i32
}

#[php_impl]
impl Example {
    pub fn make_new(foo: i32, bar: i32) -> Example {
        Example { foo, bar }
    }
}

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module.class::<Example>()
}
fn main() {}
"####);
    skeptic::rt::compile_test(r#"C:\Users\adria\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\ext-php-rs-0.14.2"#, r#"D:\desenvolvimento\wead\example-rust-php-ext\target\debug\build\ext-php-rs-6322a22b557eeda2\out"#, r#"x86_64-pc-windows-msvc"#, s);
}

#[test] fn functions_sect_functions_methods_line_8() {
    let s = &format!(r####"
{}"####, r####"#![cfg_attr(windows, feature(abi_vectorcall))]
extern crate ext_php_rs;
use ext_php_rs::prelude::*;

use ext_php_rs::zend::Function;

#[php_function]
pub fn test_function() -> () {
    let var_dump = Function::try_from_function("var_dump").unwrap();
    let _ = var_dump.try_call(vec![&"abc"]);
}

#[php_function]
pub fn test_method() -> () {
    let f = Function::try_from_method("ClassName", "staticMethod").unwrap();
    let _ = f.try_call(vec![&"abc"]);
}

fn main() {}
"####);
    skeptic::rt::compile_test(r#"C:\Users\adria\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\ext-php-rs-0.14.2"#, r#"D:\desenvolvimento\wead\example-rust-php-ext\target\debug\build\ext-php-rs-6322a22b557eeda2\out"#, r#"x86_64-pc-windows-msvc"#, s);
}

#[test] fn hashmap_sect_rust_example_line_19() {
    let s = &format!(r####"
{}"####, r####"#![cfg_attr(windows, feature(abi_vectorcall))]
extern crate ext_php_rs;
use ext_php_rs::prelude::*;
use std::collections::HashMap;
#[php_function]
pub fn test_hashmap(hm: HashMap<String, String>) -> Vec<String> {
    for (k, v) in hm.iter() {
        println!("k: {} v: {}", k, v);
    }

    hm.into_iter()
        .map(|(_, v)| v)
        .collect::<Vec<_>>()
}
fn main() {}
"####);
    skeptic::rt::compile_test(r#"C:\Users\adria\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\ext-php-rs-0.14.2"#, r#"D:\desenvolvimento\wead\example-rust-php-ext\target\debug\build\ext-php-rs-6322a22b557eeda2\out"#, r#"x86_64-pc-windows-msvc"#, s);
}

#[test] fn iterable_sect_rust_example_line_15() {
    let s = &format!(r####"
{}"####, r####"#![cfg_attr(windows, feature(abi_vectorcall))]
extern crate ext_php_rs;
use ext_php_rs::prelude::*;
use ext_php_rs::types::Iterable;
#[php_function]
pub fn test_iterable(mut iterable: Iterable) {
    for (k, v) in iterable.iter().expect("cannot rewind iterator") {
        println!("k: {} v: {}", k.string().unwrap(), v.string().unwrap());
    }
}
fn main() {}
"####);
    skeptic::rt::compile_test(r#"C:\Users\adria\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\ext-php-rs-0.14.2"#, r#"D:\desenvolvimento\wead\example-rust-php-ext\target\debug\build\ext-php-rs-6322a22b557eeda2\out"#, r#"x86_64-pc-windows-msvc"#, s);
}

#[test] fn iterator_sect_rust_example_line_18() {
    let s = &format!(r####"
{}"####, r####"#![cfg_attr(windows, feature(abi_vectorcall))]
extern crate ext_php_rs;
use ext_php_rs::prelude::*;
use ext_php_rs::types::ZendIterator;
#[php_function]
pub fn test_iterator(iterator: &mut ZendIterator) {
    for (k, v) in iterator.iter().expect("cannot rewind iterator") {
        // Note that the key can be anything, even an object
        // when iterating over Traversables!
        println!("k: {} v: {}", k.string().unwrap(), v.string().unwrap());
    }
}
fn main() {}
"####);
    skeptic::rt::compile_test(r#"C:\Users\adria\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\ext-php-rs-0.14.2"#, r#"D:\desenvolvimento\wead\example-rust-php-ext\target\debug\build\ext-php-rs-6322a22b557eeda2\out"#, r#"x86_64-pc-windows-msvc"#, s);
}

#[test] fn numbers_sect_rust_example_line_24() {
    let s = &format!(r####"
{}"####, r####"#![cfg_attr(windows, feature(abi_vectorcall))]
extern crate ext_php_rs;
use ext_php_rs::prelude::*;
#[php_function]
pub fn test_numbers(a: i32, b: u32, c: f32) -> u8 {
    println!("a {} b {} c {}", a, b, c);
    0
}
fn main() {}
"####);
    skeptic::rt::compile_test(r#"C:\Users\adria\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\ext-php-rs-0.14.2"#, r#"D:\desenvolvimento\wead\example-rust-php-ext\target\debug\build\ext-php-rs-6322a22b557eeda2\out"#, r#"x86_64-pc-windows-msvc"#, s);
}

#[test] fn object_sect_examples_line_20() {
    let s = &format!(r####"
{}"####, r####"#![cfg_attr(windows, feature(abi_vectorcall))]
extern crate ext_php_rs;
use ext_php_rs::{prelude::*, types::ZendObject};

// Take an object reference and also return it.
#[php_function]
pub fn take_obj(obj: &mut ZendObject) -> () {
    let _ = obj.try_call_method("hello", vec![&"arg1", &"arg2"]);
}

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module.function(wrap_function!(take_obj))
}
fn main() {}
"####);
    skeptic::rt::compile_test(r#"C:\Users\adria\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\ext-php-rs-0.14.2"#, r#"D:\desenvolvimento\wead\example-rust-php-ext\target\debug\build\ext-php-rs-6322a22b557eeda2\out"#, r#"x86_64-pc-windows-msvc"#, s);
}

#[test] fn object_sect_examples_line_40() {
    let s = &format!(r####"
{}"####, r####"#![cfg_attr(windows, feature(abi_vectorcall))]
extern crate ext_php_rs;
use ext_php_rs::{prelude::*, types::ZendObject};

// Take an object reference and also return it.
#[php_function]
pub fn take_obj(obj: &mut ZendObject) -> &mut ZendObject {
    let _ = obj.set_property("hello", 5);
    dbg!(obj)
}

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module.function(wrap_function!(take_obj))
}
fn main() {}
"####);
    skeptic::rt::compile_test(r#"C:\Users\adria\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\ext-php-rs-0.14.2"#, r#"D:\desenvolvimento\wead\example-rust-php-ext\target\debug\build\ext-php-rs-6322a22b557eeda2\out"#, r#"x86_64-pc-windows-msvc"#, s);
}

#[test] fn object_sect_examples_line_61() {
    let s = &format!(r####"
{}"####, r####"#![cfg_attr(windows, feature(abi_vectorcall))]
extern crate ext_php_rs;
use ext_php_rs::{prelude::*, types::ZendObject, boxed::ZBox};

// Create a new `stdClass` and return it.
#[php_function]
pub fn make_object() -> ZBox<ZendObject> {
    let mut obj = ZendObject::new_stdclass();
    let _ = obj.set_property("hello", 5);
    obj
}

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module.function(wrap_function!(make_object))
}
fn main() {}
"####);
    skeptic::rt::compile_test(r#"C:\Users\adria\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\ext-php-rs-0.14.2"#, r#"D:\desenvolvimento\wead\example-rust-php-ext\target\debug\build\ext-php-rs-6322a22b557eeda2\out"#, r#"x86_64-pc-windows-msvc"#, s);
}

#[test] fn option_sect_rust_example_line_21() {
    let s = &format!(r####"
{}"####, r####"#![cfg_attr(windows, feature(abi_vectorcall))]
extern crate ext_php_rs;
use ext_php_rs::prelude::*;
#[php_function]
pub fn test_option_null(input: Option<String>) -> Option<String> {
    input.map(|input| format!("Hello {}", input).into())
}
fn main() {}
"####);
    skeptic::rt::compile_test(r#"C:\Users\adria\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\ext-php-rs-0.14.2"#, r#"D:\desenvolvimento\wead\example-rust-php-ext\target\debug\build\ext-php-rs-6322a22b557eeda2\out"#, r#"x86_64-pc-windows-msvc"#, s);
}

#[test] fn str_sect_rust_example_line_20() {
    let s = &format!(r####"
{}"####, r####"#![cfg_attr(windows, feature(abi_vectorcall))]
extern crate ext_php_rs;
use ext_php_rs::prelude::*;
#[php_function]
pub fn str_example(input: &str) -> String {
    format!("Hello {}", input)
}

#[php_function]
pub fn str_return_example() -> &'static str {
    "Hello from Rust"
}
fn main() {}
"####);
    skeptic::rt::compile_test(r#"C:\Users\adria\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\ext-php-rs-0.14.2"#, r#"D:\desenvolvimento\wead\example-rust-php-ext\target\debug\build\ext-php-rs-6322a22b557eeda2\out"#, r#"x86_64-pc-windows-msvc"#, s);
}

#[test] fn string_sect_rust_example_line_19() {
    let s = &format!(r####"
{}"####, r####"#![cfg_attr(windows, feature(abi_vectorcall))]
extern crate ext_php_rs;
use ext_php_rs::prelude::*;
#[php_function]
pub fn str_example(input: String) -> String {
    format!("Hello {}", input)
}
fn main() {}
"####);
    skeptic::rt::compile_test(r#"C:\Users\adria\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\ext-php-rs-0.14.2"#, r#"D:\desenvolvimento\wead\example-rust-php-ext\target\debug\build\ext-php-rs-6322a22b557eeda2\out"#, r#"x86_64-pc-windows-msvc"#, s);
}

#[test] fn vec_sect_rust_example_line_21() {
    let s = &format!(r####"
{}"####, r####"#![cfg_attr(windows, feature(abi_vectorcall))]
extern crate ext_php_rs;
use ext_php_rs::prelude::*;
#[php_function]
pub fn test_vec(vec: Vec<String>) -> String {
    vec.join(" ")
}
fn main() {}
"####);
    skeptic::rt::compile_test(r#"C:\Users\adria\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\ext-php-rs-0.14.2"#, r#"D:\desenvolvimento\wead\example-rust-php-ext\target\debug\build\ext-php-rs-6322a22b557eeda2\out"#, r#"x86_64-pc-windows-msvc"#, s);
}

