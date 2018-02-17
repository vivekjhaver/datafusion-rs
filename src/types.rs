// Copyright 2018 Grove Enterprises LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::fmt::Debug;

pub trait Value : Debug {
}

pub trait DataType {
    fn name() -> &'static str;
    fn decode(&self, bytes: &[u8]) -> Box<Value>;
}

/// Definition of a column in a relation (data set).
//#[derive(Debug,Clone)]
//pub struct Field {
//    pub name: String,
//    pub data_type: Box<DataType>,
//    pub nullable: bool
//}
//
//impl Field {
//    pub fn new(name: &str, data_type: Box<DataType>, nullable: bool) -> Self {
//        Field {
//            name: name.to_string(),
//            data_type: data_type,
//            nullable: nullable
//        }
//    }
//
//    pub fn to_string(&self) -> String {
//        format!("{}: {:?}", self.name, self.data_type)
//    }
//}


/// Definition of a relation (data set) consisting of one or more columns.
//#[derive(Debug,Clone)]
//pub struct Schema {
//    pub columns: Vec<Field>
//}

trait Row<'a> {
    fn value(i: usize) -> &'a Value;
}

impl Value for f64 {
}

//impl Clone for Box<f64> {
//    fn clone(&self) -> Self {
//        Box::new(self)
//    }
//}


impl DataType for f64 {
    fn name() -> &'static str {
        "f64"
    }

    fn decode(&self, bytes: &[u8]) -> Box<Value> {
        //TODO: parse bytes
        let n = 12.3;
        Box::new(n)
    }
}