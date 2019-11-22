macro_rules! c_string {
    ( $string:tt ) => {
        concat!($string, "\0")
    };
}

macro_rules! to_ptr {
    ( $expr:expr ) => {
        $expr.as_ptr() as *const i8
    };
}

macro_rules! printk {
    ( $string:tt $(, $arg:expr )* ) => {
        printk( concat!($string, "\n\0").as_ptr() as *const i8, $( $arg ),* )
    };
}
