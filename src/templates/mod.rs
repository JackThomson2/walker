use std::collections::HashMap;

use ntex::util::{BytesMut, Buf, BufMut};
use lazy_static::lazy_static;
use napi::Result;
use parking_lot::RwLock;
use serde_json::Value;
use tera::{Context, Tera};

use crate::request::helpers::make_js_error;

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
    let tera = Tera::new(&format!("{}/**/*", directory)).map_err(|_| make_js_error("Error loading directory."))?;

    let mut templates = TEMPLATES.write();
    templates.insert(group_name, tera);
    Ok(())
}

#[cold]
#[inline(never)]
#[napi]
pub fn reload_group(group_name: String) -> Result<()> {
    let mut templates = TEMPLATES.write();
    let found_template = templates.get_mut(&group_name).ok_or_else(|| make_js_error("Error with reloading the group."))?;
    found_template
        .full_reload()
        .map_err(|_| make_js_error("Error with reloading the group."))
}

#[inline(always)]
pub(crate) fn render_value_to_writer(
    group_name: &str,
    file_name: &str,
    data: Value,
    writer: &mut BytesMut,
) -> Result<()> {
    let reader = TEMPLATES.read();
    let found_template = reader.get(group_name).ok_or_else(|| make_js_error("Error finding the template file."))?;
    let context = &Context::from_value(data).map_err(|_| make_js_error("Error reading data value."))?;

    found_template
        .render_to(file_name, context, writer.writer())
        .map_err(|_| make_js_error("Error rendering the text"))
}

#[inline(always)]
pub(crate) fn render_string_to_writer(
    group_name: &str,
    file_name: &str,
    data: &str,
    writer: &mut BytesMut,
) -> Result<()> {
    let parsed: Value = serde_json::from_str(data).map_err(|_| make_js_error("Error parsing json data."))?;
    render_value_to_writer(group_name, file_name, parsed, writer)
}

#[inline(always)]
pub(crate) fn store_in_bytes_buffer(
    group_name: &str,
    file_name: &str,
    data: &str,
) -> Result<BytesMut> {
    let mut buffer = BytesMut::with_capacity(1024);
    render_string_to_writer(group_name, file_name, data, &mut buffer)?;
    Ok(buffer)
}
