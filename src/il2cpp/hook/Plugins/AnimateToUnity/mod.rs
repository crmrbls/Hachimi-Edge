use crate::il2cpp::types::Il2CppImage;

mod AnText;
mod AnMeshInfoParameterGroup;
mod AnMeshInfoParameter;
mod AnMeshParameter;
pub mod AnRoot;
mod AnMeshParameterGroup;
mod AnRootParameter;
mod AnPlaneParameter;
mod AnMotionParameterGroup;
mod AnMotionParameter;
mod AnTextParameter;
mod AnObjectParameterBase;
mod AnGlobalData;

pub fn init(image: *const Il2CppImage) {
    AnText::init(image);
    AnMeshInfoParameterGroup::init(image);
    AnMeshInfoParameter::init(image);
    AnMeshParameter::init(image);
    AnRoot::init(image);
    AnMeshParameterGroup::init(image);
    AnRootParameter::init(image);
    AnMotionParameterGroup::init(image);
    AnPlaneParameter::init(image);
    AnMotionParameter::init(image);
    AnTextParameter::init(image);
    AnObjectParameterBase::init(image);
    AnGlobalData::init(image);
}