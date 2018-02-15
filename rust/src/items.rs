// Copyright 2016 Mozilla
//
// Licensed under the Apache License, Version 2.0 (the "License"); you may not use
// this file except in compliance with the License. You may obtain a copy of the
// License at http://www.apache.org/licenses/LICENSE-2.0
// Unless required by applicable law or agreed to in writing, software distributed
// under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR
// CONDITIONS OF ANY KIND, either express or implied. See the License for the
// specific language governing permissions and limitations under the License.

use std::os::raw::{
    c_char,
};

use ffi_utils::strings::{
    c_char_to_string,
};

use libc::size_t;

use time::Timespec;

use mentat::{
    Uuid,
};

use utils::{
    Entity,
};

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Item {
    pub id: Option<Entity>,
    pub uuid: Uuid,
    pub name: String,
    pub due_date: Option<Timespec>,
    pub completion_date: Option<Timespec>,
}

#[derive(Debug)]
pub struct Items {
    pub vec: Vec<Item>
}

impl Items {
    pub fn new(vec: Vec<Item>) -> Items {
        Items {
            vec: vec
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn item_set_name(item: *mut Item, name: *const c_char) {
    let item = &mut*item;
    item.name = c_char_to_string(name);
}

#[no_mangle]
pub unsafe extern "C" fn item_set_due_date(item: *mut Item, due_date: *const size_t) {
    let item = &mut*item;
    if !due_date.is_null() {
        item.due_date = Some(Timespec::new(due_date as i64, 0));
    } else {
        item.due_date = None;
    }
}

#[no_mangle]
pub unsafe extern "C" fn item_set_completion_date(item: *mut Item, completion_date: *const size_t) {
    let item = &mut*item;
    if !completion_date.is_null() {
        item.completion_date = Some(Timespec::new(completion_date as i64, 0));
    } else {
        item.completion_date = None;
    }
}
