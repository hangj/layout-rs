#![doc = include_str!("../README.md")]

use std::fmt::Display;

pub use layout_macro::Layout;

#[derive(Debug)]
pub struct LayoutInfo {
    /// type name
    pub name: &'static str,
    // todo: type id
    // pub id: std::any::TypeId,
    pub size: usize,
    pub align: usize,
    pub fields: Vec<Field>,
}

impl LayoutInfo {
    pub fn new(name: &'static str, size: usize, align: usize, fields: Vec<Field>) -> Self {
        Self {
            name,
            // id,
            size,
            align,
            fields
        }
    }
}

#[derive(Debug)]
pub struct Field {
    pub name: &'static str,
    pub offset: usize,
    pub layout: LayoutInfo,
}

pub trait Layout {
    fn get_layout() -> LayoutInfo;
}

#[macro_export]
macro_rules! offset_of_struct {
    ($struct_name: ty, $field_name: tt) => {
        {
            // let p: *const $struct_name = std::ptr::null();
            let p = 0 as *const $struct_name;
            unsafe {&(*p).$field_name as *const _ as usize}
        }
    };
}

impl Display for LayoutInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{} (size: {}, align: {})\n", self.name, self.size, self.align))?;

        let name_width = 8;
        let offset_width = 6;
        let size_width = 6;
        let ty_width = 10;

        // center-aligned
        f.write_fmt(format_args!("| {:^name_width$} | {:^offset_width$} | {:^size_width$} | {:^ty_width$} |\n", "field", "offset", "size", "type"))?;
        // left-aligned. if the value being formatted is smaller than width, then '-' will be printed
        f.write_fmt(format_args!("| {:-<name_width$} | {:-<offset_width$} | {:-<size_width$} | {:-<ty_width$} |\n",
            "-",
            "-",
            "-",
            "-",
        ))?;

        for field in self.fields.iter() {
            f.write_fmt(format_args!("| {:<name_width$} | {:<offset_width$} | {:<size_width$} | {:<ty_width$} |\n", 
                field.name, 
                field.offset, 
                field.layout.size,
                format!("{} (align: {})", 
                    field.layout.name,
                    field.layout.align,),
            ))?;
        }
        f.write_fmt(format_args!(""))
    }
}