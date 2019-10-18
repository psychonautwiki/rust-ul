#[macro_export]
macro_rules! config_item (
    ($name:ident, $type:ty, $comment:expr) => (
        #[doc = $comment]
        pub fn $name(&mut self, flag: $type) {
           self.$name = Some(flag);
        }
    )
);

#[macro_export]
macro_rules! set_config (
    ($config: expr, $self: expr, $name:ident, $ffiName:ident) => (
        if $self.$name.is_some() {
            unsafe {
                ul::$ffiName($config, $self.$name.unwrap());
            }
        }
    )
);

#[macro_export]
macro_rules! set_config_str (
    ($config: expr, $self: expr, $name:ident, $ffiName:ident) => (
        if $self.$name.is_some() {
            unsafe {
                let str = ul::ulCreateString(
                    std::ffi::CString::new(
                        $self.$name.clone().unwrap()
                    ).unwrap().as_ptr()
                );

                ul::$ffiName($config, str);
            }
        }
    )
);
