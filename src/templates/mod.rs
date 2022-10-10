use std::collections::HashMap;

use bytes::BytesMut;
use lazy_static::lazy_static;
use napi::Result;
use parking_lot::RwLock;
use serde_json::Value;
use tera::{Tera, Context};

use crate::request::{helpers::make_generic_error, writer::Writer};

lazy_static! {
    pub static ref TEMPLATES: RwLock<HashMap<String, Tera>> = {
        let map = HashMap::new();
        RwLock::new(map)
    };
}

#[cold]
#[inline(never)]
#[napi]
pub fn load_new_template(group_name: String, directory: String) -> Result<()> {
    let tera = Tera::new(&format!("{}/**/*", directory)).map_err(|_| make_generic_error())?;

    let mut templates = TEMPLATES.write();
    templates.insert(group_name, tera);    
    Ok(())
}

#[cold]
#[inline(never)]
#[napi]
pub fn reload_group(group_name: String) -> Result<()> {
    let mut templates = TEMPLATES.write();
    let found_template = templates.get_mut(&group_name).unwrap();    
    found_template.full_reload().map_err(|_| make_generic_error())
}

#[inline(always)]
pub(crate) fn render_value_to_writer(group_name: &str, file_name: &str, data: Value, writer: &mut Writer<BytesMut>) -> Result<()> {
    let reader = TEMPLATES.read();
    let found_template = reader.get(group_name).ok_or_else(make_generic_error)?;
    let context = &Context::from_value(data).map_err(|_| make_generic_error())?;

    found_template.render_to(file_name, context, writer).map_err(|_| make_generic_error())
}

#[inline(always)]
pub(crate) fn render_string_to_writer(group_name: &str, file_name: &str, data: &str, writer: &mut Writer<BytesMut>) -> Result<()> {
    let parsed: Value = serde_json::from_str(data).map_err(|_| make_generic_error())?;
    render_value_to_writer(group_name, file_name, parsed, writer)
}