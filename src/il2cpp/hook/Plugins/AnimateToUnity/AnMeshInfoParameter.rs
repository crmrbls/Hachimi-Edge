use crate::il2cpp::{ext::{StringExt, Il2CppStringExt}, symbols::{get_field_from_name, get_field_object_value, set_field_object_value}, types::*};
use std::ptr::null_mut;

// String _textureName
static mut _TEXTURENAME_FIELD: *mut FieldInfo = null_mut();
pub fn get__textureName(this: *mut Il2CppObject) -> String {
    unsafe {
        let field_value = get_field_object_value(this, _TEXTURENAME_FIELD) as *mut Il2CppString;
        if field_value.is_null() {
            String::new()
        } else {
            (*field_value).as_utf16str().to_string()
        }
    }
}

#[allow(dead_code)]
pub fn set__textureName(this: *mut Il2CppObject, value: &str) {
    set_field_object_value(this, unsafe { _TEXTURENAME_FIELD }, value.to_il2cpp_string());
}

// String _fixTextureName
static mut _FIXTEXTURENAME_FIELD: *mut FieldInfo = null_mut();
#[allow(dead_code)]
pub fn get__fixTextureName(this: *mut Il2CppObject) -> String {
    unsafe {
        let field_value = get_field_object_value(this, _FIXTEXTURENAME_FIELD) as *mut Il2CppString;
        if field_value.is_null() {
            String::new()
        } else {
            (*field_value).as_utf16str().to_string()
        }
    }
}

#[allow(dead_code)]
pub fn set__fixTextureName(this: *mut Il2CppObject, value: &str) {
    set_field_object_value(this, unsafe { _FIXTEXTURENAME_FIELD }, value.to_il2cpp_string());
}

// Vector2 _uvSize
static mut _UVSIZE_FIELD: *mut FieldInfo = null_mut();
#[allow(dead_code)]
pub fn get__uvSize(this: *mut Il2CppObject) -> Vector2_t {
    unsafe {
        let field_value = get_field_object_value(this, _UVSIZE_FIELD) as *mut Vector2_t;
        if field_value.is_null() {
            Vector2_t { x: 0.0, y: 0.0 }
        } else {
            *field_value
        }
    }
}

pub fn set__uvSize(this: *mut Il2CppObject, value: &Vector2_t) {
    set_field_object_value(this, unsafe { _UVSIZE_FIELD }, value as *const Vector2_t as *mut Vector2_t);
}

// Vector2 _uvOffset
static mut _UVOFFSET_FIELD: *mut FieldInfo = null_mut();
#[allow(dead_code)]
pub fn get__uvOffset(this: *mut Il2CppObject) -> Vector2_t {
    unsafe {
        let field_value = get_field_object_value(this, _UVOFFSET_FIELD) as *mut Vector2_t;
        if field_value.is_null() {
            Vector2_t { x: 0.0, y: 0.0 }
        } else {
            *field_value
        }
    }
}

pub fn set__uvOffset(this: *mut Il2CppObject, value: &Vector2_t) {
    set_field_object_value(this, unsafe { _UVOFFSET_FIELD }, value as *const Vector2_t as *mut Vector2_t);
}

// Boolean _rotated
static mut _ROTATED_FIELD: *mut FieldInfo = null_mut();
#[allow(dead_code)]
pub fn get__rotated(this: *mut Il2CppObject) -> bool {
    unsafe {
        let field_value = get_field_object_value(this, _ROTATED_FIELD) as *const i32;
        if field_value.is_null() {
            false
        } else {
            *field_value != 0
        }
    }
}

pub fn set__rotated(this: *mut Il2CppObject, value: bool) {
    let bool_val = if value { 1i32 } else { 0i32 };
    set_field_object_value(this, unsafe { _ROTATED_FIELD }, &bool_val as *const i32 as *mut i32);
}

// Vector2 _size
static mut _SIZE_FIELD: *mut FieldInfo = null_mut();
#[allow(dead_code)]
pub fn get__size(this: *mut Il2CppObject) -> Vector2_t {
    unsafe {
        let field_value = get_field_object_value(this, _SIZE_FIELD) as *mut Vector2_t;
        if field_value.is_null() {
            Vector2_t { x: 0.0, y: 0.0 }
        } else {
            *field_value
        }
    }
}

pub fn set__size(this: *mut Il2CppObject, value: &Vector2_t) {
    set_field_object_value(this, unsafe { _SIZE_FIELD }, value as *const Vector2_t as *mut Vector2_t);
}

// Vector2 _offset
static mut _OFFSET_FIELD: *mut FieldInfo = null_mut();
#[allow(dead_code)]
pub fn get__offset(this: *mut Il2CppObject) -> Vector2_t {
    unsafe {
        let field_value = get_field_object_value(this, _OFFSET_FIELD) as *mut Vector2_t;
        if field_value.is_null() {
            Vector2_t { x: 0.0, y: 0.0 }
        } else {
            *field_value
        }
    }
}

pub fn set__offset(this: *mut Il2CppObject, value: &Vector2_t) {
    set_field_object_value(this, unsafe { _OFFSET_FIELD }, value as *const Vector2_t as *mut Vector2_t);
}

pub fn init(Plugins: *const Il2CppImage) {
    get_class_or_return!(Plugins, AnimateToUnity, AnMeshInfoParameter);

    unsafe {
        _TEXTURENAME_FIELD = get_field_from_name(AnMeshInfoParameter, c"_textureName");
        _FIXTEXTURENAME_FIELD = get_field_from_name(AnMeshInfoParameter, c"_fixTextureName");
        _UVSIZE_FIELD = get_field_from_name(AnMeshInfoParameter, c"_uvSize");
        _UVOFFSET_FIELD = get_field_from_name(AnMeshInfoParameter, c"_uvOffset");
        _ROTATED_FIELD = get_field_from_name(AnMeshInfoParameter, c"_rotated");
        _SIZE_FIELD = get_field_from_name(AnMeshInfoParameter, c"_size");
        _OFFSET_FIELD = get_field_from_name(AnMeshInfoParameter, c"_offset");
    }
}
