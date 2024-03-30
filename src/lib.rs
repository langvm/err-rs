// Copyright 2024 LangVM Project
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0
// that can be found in the LICENSE file and https://mozilla.org/MPL/2.0/.

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
macro_rules! wrap_result {
    ($tag:expr, $expr:expr) => {
        match $expr {
            Ok(v) => { Ok(v) }
            Err(e) => { return Err($tag(e)); }
        }?
    };
}
