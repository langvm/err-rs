// Copyright 2024 Jelly Terra
// Use of this source code form is governed under the MIT license.

#[macro_export]
macro_rules! err {
    ($tag:expr, $expr:expr) => {
        return Err($tag($expr));
    };
    ($expr:expr) => {
        return Err($expr);
    };
}

#[macro_export]
macro_rules! ok {
    ($expr:expr) => {
        return Ok($expr);
    };
}

#[macro_export]
macro_rules! wrap_result {
    ($tag:expr, $expr:expr) => {
        match $expr {
            Ok(v) => v,
            Err(e) => err!($tag, e)
        }
    };
}

#[macro_export]
macro_rules! on_err {
    ($expr:expr, $err_name:ident => $err_handle:expr) => {
        match $expr {
            Ok(v) => v,
            Err($err_name) => $err_handle
        }
    };
}
