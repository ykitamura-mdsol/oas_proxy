use openapiv3::*;
use crate::error::{E, unsupported};
use anyhow::{Result};

pub fn path_to_operation3<'a>(item: &'a mut PathItem) -> (&'a mut Vec<ReferenceOr<Parameter>>,&'a mut Operation) {
        (&mut item.parameters,
        item
        .get.as_mut()
        .or(item.head.as_mut())
        .or(item.options.as_mut())
        .or(item.trace.as_mut())
        .or(item.delete.as_mut())
        .or(item.patch.as_mut())
        .or(item.post.as_mut())
        .or(item.put.as_mut()).expect("Failed")) //&format!("Failed at {:?}", &item)))

}

pub fn path_to_operation<'a>(path_matcher: &'a ReferenceOr<PathItem>) -> (&'a Vec<ReferenceOr<Parameter>>,&'a Operation) {
    match path_matcher {
        ReferenceOr::Reference { reference: _ } => {
            // TODO: move reference finder somewhere and use it from here to clean up.
            unimplemented!("TODO: path level reference found");
        }
        ReferenceOr::Item(item) => {
            (&item.parameters,
            item
            .get.as_ref()
            .or(item.head.as_ref())
            .or(item.options.as_ref())
            .or(item.trace.as_ref())
            .or(item.delete.as_ref())
            .or(item.patch.as_ref())
            .or(item.post.as_ref())
            .or(item.put.as_ref()).expect(&format!("Failed at {:?}", &item)))}

    }
}

pub fn parameter_to_parameter_data(parameter: &mut Parameter) -> &mut ParameterData {
    match parameter {
        Parameter::Query { parameter_data, .. } => parameter_data,
        Parameter::Header { parameter_data, .. } => parameter_data,
        Parameter::Path { parameter_data, .. } => parameter_data,
        Parameter::Cookie { parameter_data, .. } => parameter_data,
    }
}

pub fn used(description: &mut Option<String>) {
    *description = Some("1".to_string());
}


