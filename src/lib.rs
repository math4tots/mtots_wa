#![allow(unused_imports)]
extern crate mtots_core;
extern crate stdweb;

mod m;

use mtots_core::BuiltinClasses;
use mtots_core::BuiltinExceptions;
use mtots_core::Class;
use mtots_core::ClassKind;
use mtots_core::ErrorIndicator;
use mtots_core::Eval;
use mtots_core::EvalError;
use mtots_core::EvalResult;
use mtots_core::Exception;
use mtots_core::ExceptionKind;
use mtots_core::Function;
use mtots_core::GMap;
use mtots_core::GeneratorResult;
use mtots_core::Globals;
use mtots_core::HMap;
use mtots_core::Module;
use mtots_core::NativeFunction;
use mtots_core::NativeFunctions;
use mtots_core::NativeIterator;
use mtots_core::Opaque;
use mtots_core::ParameterInfo;
use mtots_core::ParameterKind;
use mtots_core::ParseError;
use mtots_core::RcPath;
use mtots_core::RcStr;
use mtots_core::Symbol;
use mtots_core::SymbolRegistryHandle;
use mtots_core::Table;
use mtots_core::VMap;
use mtots_core::Value;
use mtots_core::ValueKind;
use mtots_core::SOURCE_FILE_EXTENSION;

pub use m::main;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
