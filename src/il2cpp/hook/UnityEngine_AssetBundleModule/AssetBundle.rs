use std::sync::Mutex;

use fnv::FnvHashMap;
use once_cell::sync::Lazy;
use widestring::Utf16Str;

use crate::{core::{ext::Utf16StringExt, hachimi::{AssetMetadata, Hachimi}}, il2cpp::{
    api::il2cpp_resolve_icall, ext::{Il2CppObjectExt, Il2CppStringExt}, hook::{
        umamusume::{StoryRaceTextAsset, StoryTimelineData, TextDotData, TextRubyData},
        Cute_UI_Assembly::AtlasReference,
        UnityEngine_CoreModule::{GameObject, Texture2D}
    }, symbols::GCHandle, types::*
}};

#[cfg(target_os = "windows")]
use windows::{Win32::System::Diagnostics::Debug::OutputDebugStringW, core::PCWSTR};

fn debug_output(msg: &str) {
    // Only output if debug_mode is enabled in runtime config
    if Hachimi::is_initialized() && Hachimi::instance().config.load().debug_mode {
        #[cfg(target_os = "windows")]
        {
            // Use Windows Debug Output for DebugView++
            let wide_msg = widestring::WideCString::from_str(msg).unwrap_or_default();
            unsafe {
                OutputDebugStringW(PCWSTR(wide_msg.as_ptr()));
            }
        }
        #[cfg(not(target_os = "windows"))]
        {
            // Use println for other platforms (Android, etc.)
            println!("[Hachimi Bundle] {}", msg);
        }
    }
}

pub const ASSET_PATH_PREFIX: &str = "assets/_gallopresources/bundle/resources/";

pub struct RequestInfo {
    pub name_handle: GCHandle,
    pub bundle: usize // *mut Il2CppObject (this)
}
impl RequestInfo {
    pub fn name(&self) -> *mut Il2CppString {
        self.name_handle.target() as _
    }
}
pub static REQUEST_INFOS: Lazy<Mutex<FnvHashMap<usize, RequestInfo>>> = Lazy::new(|| Mutex::default());

static BUNDLE_PATHS: Lazy<Mutex<FnvHashMap<usize, GCHandle>>> = Lazy::new(|| Mutex::default());
pub fn get_bundle_path(this: *mut Il2CppObject) -> Option<*mut Il2CppString> {
    Some(BUNDLE_PATHS.lock().unwrap().get(&(this as usize))?.target() as _)
}

pub fn check_asset_bundle_name(this: *mut Il2CppObject, metadata: &AssetMetadata) -> bool {
    if let Some(meta_bundle_name) = &metadata.bundle_name {
        if let Some(bundle_path) = get_bundle_path(this) {
            let bundle_name = unsafe { (*bundle_path).as_utf16str().path_filename() };
            debug_output(&format!(
                "[BUNDLE] Check: expected='{}' vs actual='{}'",
                meta_bundle_name,
                bundle_name
            ));
            if !bundle_name.str_eq(&meta_bundle_name) {
                debug_output(&format!(
                    "[BUNDLE] [FAIL] [bundle mismatch: expected {} got {}]",
                    meta_bundle_name,
                    bundle_name
                ));
                warn!("Expected bundle {}, got {}", meta_bundle_name, bundle_name);
                return false;
            }
        }
    } else {
        debug_output("[BUNDLE] No bundle_name in metadata, skipping check");
    }

    true
}

type LoadAssetFn = extern "C" fn(this: *mut Il2CppObject, name: *mut Il2CppString, type_: *mut Il2CppObject) -> *mut Il2CppObject;
extern "C" fn LoadAsset_Internal(this: *mut Il2CppObject, name: *mut Il2CppString, type_: *mut Il2CppObject) -> *mut Il2CppObject {
    let asset_name = unsafe { (*name).as_utf16str().to_string() };
    debug_output(&format!("[ASSET] call: {}", asset_name));
    
    let asset = get_orig_fn!(LoadAsset_Internal, LoadAssetFn)(this, name, type_);
    
    if !asset.is_null() {
        debug_output(&format!("[ASSET] [OK] {}", asset_name));
    } else {
        debug_output(&format!("[ASSET] [FAIL] [asset not found in bundle] {}", asset_name));
    }
    
    on_LoadAsset(this, asset, name);
    asset
}

pub fn LoadAsset_Internal_orig(this: *mut Il2CppObject, name: *mut Il2CppString, type_: *mut Il2CppObject) -> *mut Il2CppObject {
    get_orig_fn!(LoadAsset_Internal, LoadAssetFn)(this, name, type_)
}

type LoadAssetAsyncFn = extern "C" fn(this: *mut Il2CppObject, name: *mut Il2CppString, type_: *mut Il2CppObject) -> *mut Il2CppObject;
extern "C" fn LoadAssetAsync_Internal(this: *mut Il2CppObject, name: *mut Il2CppString, type_: *mut Il2CppObject) -> *mut Il2CppObject {
    let request = get_orig_fn!(LoadAssetAsync_Internal, LoadAssetAsyncFn)(this, name, type_);
    let info = RequestInfo {
        name_handle: GCHandle::new(name as _, false), // is name even guaranteed to survive in memory..?
        bundle: this as usize
    };
    REQUEST_INFOS.lock().unwrap().insert(request as usize, info);
    request
}

type OnLoadAssetFn = fn(bundle: *mut Il2CppObject, asset: *mut Il2CppObject, name: &Utf16Str);
pub fn on_LoadAsset(bundle: *mut Il2CppObject, asset: *mut Il2CppObject, name: *mut Il2CppString) {
    let class = unsafe { (*asset).klass() };
    //debug!("{} {}", unsafe { std::ffi::CStr::from_ptr((*class).name).to_str().unwrap() }, unsafe { (*name).as_utf16str() });

    let handler: OnLoadAssetFn = if class == GameObject::class() {
        GameObject::on_LoadAsset
    }
    else if class == StoryTimelineData::class() {
        StoryTimelineData::on_LoadAsset
    }
    else if class == Texture2D::class() {
        Texture2D::on_LoadAsset
    }
    else if class == AtlasReference::class() {
        AtlasReference::on_LoadAsset
    }
    else if class == StoryRaceTextAsset::class() {
        StoryRaceTextAsset::on_LoadAsset
    }
    else if class == TextRubyData::class() {
        TextRubyData::on_LoadAsset
    }
    else if class == TextDotData::class() {
        TextDotData::on_LoadAsset
    }
    else {
        return;
    };

    handler(bundle, asset, unsafe { (*name).as_utf16str() });
}

type LoadFromFileInternalFn = extern "C" fn(path: *mut Il2CppString, crc: u32, offset: u64) -> *mut Il2CppObject;
extern "C" fn LoadFromFile_Internal(path: *mut Il2CppString, crc: u32, offset: u64) -> *mut Il2CppObject {
    let path_str = unsafe { (*path).as_utf16str().to_string() };
    // Extract just the bundle hash from the path
    let bundle_hash = path_str.split('/').last().unwrap_or(&path_str);
    debug_output(&format!("[BUNDLE] load: {}", path_str));
    
    let bundle = get_orig_fn!(LoadFromFile_Internal, LoadFromFileInternalFn)(path, crc, offset);
    
    if !bundle.is_null() {
        debug_output(&format!("[BUNDLE] [OK] [bundle found: {}]", bundle_hash));
        BUNDLE_PATHS.lock().unwrap().insert(bundle as usize, GCHandle::new(path as _, false));
    } else {
        debug_output(&format!("[BUNDLE] [FAIL] [bundle not found: {}]", bundle_hash));
    }
    bundle
}

pub fn LoadFromFile_Internal_orig(path: *mut Il2CppString, crc: u32, offset: u64) -> *mut Il2CppObject {
    get_orig_fn!(LoadFromFile_Internal, LoadFromFileInternalFn)(path, crc, offset)
}

type UnloadFn = extern "C" fn(this: *mut Il2CppObject, unload_all_loaded_objects: bool);
extern "C" fn Unload(this: *mut Il2CppObject, unload_all_loaded_objects: bool) {
    BUNDLE_PATHS.lock().unwrap().remove(&(this as usize));
    get_orig_fn!(Unload, UnloadFn)(this, unload_all_loaded_objects);
}

pub fn init(_UnityEngine_AssetBundleModule: *const Il2CppImage) {
    //get_class_or_return!(UnityEngine_AssetBundleModule, UnityEngine, AssetBundle);

    let LoadAsset_Internal_addr = il2cpp_resolve_icall(
        c"UnityEngine.AssetBundle::LoadAsset_Internal(System.String,System.Type)".as_ptr()
    );
    let LoadAssetAsync_Internal_addr = il2cpp_resolve_icall(
        c"UnityEngine.AssetBundle::LoadAssetAsync_Internal(System.String,System.Type)".as_ptr()
    );
    let LoadFromFile_Internal_addr = il2cpp_resolve_icall(
        c"UnityEngine.AssetBundle::LoadFromFile_Internal(System.String,System.UInt32,System.UInt64)".as_ptr()
    );
    let Unload_addr = il2cpp_resolve_icall(
        c"UnityEngine.AssetBundle::Unload(System.Boolean)".as_ptr()
    );

    new_hook!(LoadAsset_Internal_addr, LoadAsset_Internal);
    new_hook!(LoadAssetAsync_Internal_addr, LoadAssetAsync_Internal);
    new_hook!(LoadFromFile_Internal_addr, LoadFromFile_Internal);
    new_hook!(Unload_addr, Unload);
}