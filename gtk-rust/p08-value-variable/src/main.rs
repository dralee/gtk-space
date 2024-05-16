/*
* generic values map for c
* 2024.05.16 by dralee

enum Value <T> {
    bool(bool),
    i8(i8),
    i32(i32),
    u32(u32),
    i64(i64),
    u64(u64),
    f32(f32),
    f64(f64),
    // boxed types
    String(Option<String>),
    Object(Option<dyn IsA<glib::Object>>),
}

Variant use for serialized data type
*/
use gtk::glib::{value::ToValue, variant::ToVariant};
use gtk::glib;

fn main() {    
    test_value();
    test_string();
    test_string2();

    test_variant();
    test_variant2();
}

fn test_value(){
    // store i32 as Value
    let integer_value = 10.to_value();

    // retrieve i32 from Value
    let integer = integer_value.get::<i32>()
            .expect("The value needs to be of type i32");

    assert_eq!(integer, 10);
}

fn test_string(){
    // store String as Value
    let string_value = "Hello".to_value();

    // retrieve String from Value
    let string = string_value.get::<String>()
        .expect("The value needs to be of type String");

    assert_eq!(string, "Hello");
}

fn test_string2(){
    // store Option<String> as value
    let string_some_value = "Hello".to_value();
    let string_none_value = None::<String>.to_value();

    // retrieve String from Value
    let string_some = string_some_value.get::<Option<String>>()
        .expect("The value needs to be of type Option<String>");
    let string_none = string_none_value.get::<Option<String>>()
        .expect("The value needs to be of type Option<String>");

    assert_eq!(string_some, Some("Hello".to_string()));
    assert_eq!(string_none, None);
}

fn test_variant(){
    // store i32 as Variant
    let integer_variant = 10.to_variant();

    // retrieve i32 from variant
    let integer = integer_variant.get::<i32>()
        .expect("The variant needs to be of type i32");

    assert_eq!(integer, 10);
}

fn test_variant2(){
    let variant = vec!["Hello", "World!"].to_variant();
    assert_eq!(variant.n_children(), 2);

    let vec = &variant.get::<Vec<String>>()
        .expect("The variant needs to be of type Vec<String>");

    assert_eq!(vec[0], "Hello");

    let bufdata = b"\xFD\x2F\xB5\x28";
    let bufv = glib::Variant::array_from_fixed_array(&bufdata[..]);
    assert_eq!(bufv.fixed_array::<u8>().unwrap(), bufdata);
    // https://gtk-rs.org/gtk-rs-core/stable/latest/docs/glib/variant/index.html

}