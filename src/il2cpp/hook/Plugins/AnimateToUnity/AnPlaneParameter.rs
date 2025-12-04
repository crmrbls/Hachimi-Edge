use crate::il2cpp::types::*;

#[allow(unused)]
pub fn init(Plugins: *const Il2CppImage) {
    get_class_or_return!(Plugins, AnimateToUnity, AnPlaneParameter);
}