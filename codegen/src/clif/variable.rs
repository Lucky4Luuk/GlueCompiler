use std::collections::HashMap;

use cranelift::prelude::types::*;
use cranelift::prelude::Type;
use cranelift::prelude::ExternalName;
use cranelift::prelude::{EntityRef, Variable};

pub fn temp_func_name_map(func_ident: &str) -> ExternalName {
    ExternalName::user(0,0) //should be indices into a symbol table
}

pub fn temp_type_map(type_ident: &str) -> Type {
    match type_ident {
        "u8"  => I8,
        "u16" => I16,
        "u32" => I32,
        "u64" => I64,

        "i8"  => I8,
        "i16" => I16,
        "i32" => I32,
        "i64" => I64,
        _ => panic!("unsupported type! attempted conversion: `{}`", type_ident)
    }
}

pub struct VariableMap {
    var_lut: HashMap<usize, String>,
    vars: HashMap<String, Variable>,
    next_idx: usize,
}

impl VariableMap {
    pub fn new() -> Self {
        Self {
            var_lut: HashMap::new(),
            vars: HashMap::new(),
            next_idx: 0,
        }
    }

    pub fn add_var(&mut self, name: &str) {
        self.var_lut.insert(self.next_idx, name.to_string());
        self.vars.insert(name.to_string(), Variable::new(self.next_idx));
        self.next_idx += 1;
    }

    pub fn get_var(&self, name: &str) -> Option<Variable> {
        match self.vars.get(name) {
            Some(v) => Some(*v),
            None => None,
        }
    }
}
