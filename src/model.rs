use crate::{c, error::Error};
use std::{ffi, ptr};

pub type SchemaID = u32;
pub type SchemaUID = u64;

/// Model is used to define a database model. Use as a fluent interface (builder pattern)
pub struct Model {
    c_ptr: *mut c::OBX_model,
    error: Option<Error>,
}

pub struct Entity {
    model: Model,
}

impl Model {
    pub fn new() -> Model {
        match c::new_mut(unsafe { c::obx_model() }) {
            Ok(c_ptr) => Model { c_ptr, error: None },
            Err(e) => Model {
                c_ptr: ptr::null_mut(),
                error: Some(e),
            },
        }
    }

    /// Create an entity.
    pub fn entity(mut self, name: &str, id: SchemaID, uid: SchemaUID) -> Entity {
        if self.error.is_none() {
            let c_name = ffi::CString::new(name).unwrap();
            self.error =
                c::call(unsafe { c::obx_model_entity(self.c_ptr, c_name.as_ptr(), id, uid) }).err();
        }

        return Entity { model: self };
    }

    /// Inform the model about the last entity that was ever defined in the model.
    pub fn last_entity_id(self, id: SchemaID, uid: SchemaUID) -> Model {
        if self.error.is_none() {
            unsafe { c::obx_model_last_entity_id(self.c_ptr, id, uid) }
        }

        return self;
    }

    /// Inform the model about the last index that was ever defined in the model.
    pub fn last_index_id(self, id: SchemaID, uid: SchemaUID) -> Model {
        if self.error.is_none() {
            unsafe { c::obx_model_last_index_id(self.c_ptr, id, uid) }
        }

        return self;
    }

    /// Inform the model about the last relation that was ever defined in the model.
    pub fn last_relation_id(self, id: SchemaID, uid: SchemaUID) -> Model {
        if self.error.is_none() {
            unsafe { c::obx_model_last_relation_id(self.c_ptr, id, uid) }
        }

        return self;
    }
}

impl Entity {
    /// Inform the model about the last property that was ever defined on the entity.
    /// Finishes building the entity, returning the parent Model.
    pub fn last_property_id(self, id: SchemaID, uid: SchemaUID) -> Model {
        let mut model = self.model;
        if model.error.is_none() {
            model.error =
                c::call(unsafe { c::obx_model_entity_last_property_id(model.c_ptr, id, uid) })
                    .err();
        }

        return model;
    }

    /// Create a property.
    pub fn property(
        mut self,
        name: &str,
        typ: c::OBXPropertyType,
        flags: c::OBXPropertyFlags,
        id: SchemaID,
        uid: SchemaUID,
    ) -> Entity {
        if self.model.error.is_none() {
            let c_name = ffi::CString::new(name).unwrap();
            self.model.error = c::call(unsafe {
                c::obx_model_property(self.model.c_ptr, c_name.as_ptr(), typ, id, uid)
            })
            .err();
        }

        if flags > 0 && self.model.error.is_none() {
            self.model.error =
                c::call(unsafe { c::obx_model_property_flags(self.model.c_ptr, flags) }).err();
        }

        return self;
    }

    /// Declare an index on the last created property.
    pub fn property_index(mut self, id: SchemaID, uid: SchemaUID) -> Entity {
        if self.model.error.is_none() {
            self.model.error =
                c::call(unsafe { c::obx_model_property_index_id(self.model.c_ptr, id, uid) }).err();
        }
        return self;
    }

    /// Declare a to-one relation on the last created property.
    /// No need to declare the index separately using property_index(), it's created automatically.
    pub fn property_relation(
        mut self,
        target_entity_name: &str,
        index_id: SchemaID,
        index_uid: SchemaUID,
    ) -> Entity {
        if self.model.error.is_none() {
            let c_name = ffi::CString::new(target_entity_name).unwrap();
            self.model.error = c::call(unsafe {
                c::obx_model_property_relation(
                    self.model.c_ptr,
                    c_name.as_ptr(),
                    index_id,
                    index_uid,
                )
            })
            .err();
        }
        return self;
    }

    /// Declare a standalone to-many relation between this entity and another one
    pub fn relation(
        mut self,
        relation_id: SchemaID,
        relation_uid: SchemaUID,
        target_entity_id: SchemaID,
        target_entity_uid: SchemaUID,
    ) -> Entity {
        if self.model.error.is_none() {
            self.model.error = c::call(unsafe {
                c::obx_model_relation(
                    self.model.c_ptr,
                    relation_id,
                    relation_uid,
                    target_entity_id,
                    target_entity_uid,
                )
            })
            .err();
        }
        return self;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn model_builder_positive() {
        let model = Model::new()
            .entity("A", 1, 1)
            .property(
                "id",
                c::OBXPropertyType_Long,
                c::OBXPropertyFlags_ID,
                1,
                101,
            )
            .property("text", c::OBXPropertyType_String, 0, 2, 102)
            .property_index(1, 1021)
            .last_property_id(2, 102)
            .entity("B", 2, 2)
            .property(
                "id",
                c::OBXPropertyType_Long,
                c::OBXPropertyFlags_ID,
                1,
                201,
            )
            .property("number", c::OBXPropertyType_Int, 0, 2, 202)
            .last_property_id(2, 202)
            .last_entity_id(2, 2)
            .last_index_id(1, 1021);

        assert!(model.error.is_none());
    }

    #[test]
    fn model_builder_negative() {
        let model = Model::new().entity("A", 1, 1).last_property_id(0, 0);

        let expected_err = format!(
            "{} Argument condition \"property_id\" not met",
            c::OBX_ERROR_ILLEGAL_ARGUMENT
        );
        let actual_err = format!("{}", model.error.unwrap());
        println!("expected: {}", &expected_err);
        println!("actual: {}", &actual_err);
        assert!(actual_err.starts_with(&expected_err));
    }
}
