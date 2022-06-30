#[macro_export]
macro_rules! getter {
    ($field:ident -> $typ:ty) => {
        pub fn $field(&self) -> $typ {
            self.$field.clone()
        }
    };
    (&$field:ident -> $typ:ty) => {
        pub fn $field<'a>(&'a self) -> &'a $typ {
            &self.$field
        }
    };

    ($getter_name:ident -> unwrap_or $field:ident $fallback:expr => $typ:ty) => {
        pub fn $getter_name(&self) -> $typ {
            self.$field.unwrap_or($fallback).clone()
        }
    };
    ($getter_name:ident -> $field:ident -> $typ:ty) => {
        pub fn $getter_name(&self) -> $typ {
            self.$field.clone()
        }
    };

    ($getter_name:ident -> &$field:ident => $typ:ty) => {
        pub fn $getter_name<'a>(&'a self) -> &'a $typ {
            &self.$field
        }
    };
}
