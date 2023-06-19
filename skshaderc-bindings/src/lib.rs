use std::ffi::CString;
use std::fs::OpenOptions;
use std::io::Write;
use std::ptr::null_mut;
//include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct compiler_settings_t {
    pub replace_ext: bool,
    pub output_header: bool,
    pub output_zipped: bool,
    pub output_raw_shaders: bool,
    pub only_if_changed: bool,
    pub out_folder: *mut ::std::os::raw::c_char,
    pub shaderc: sksc_settings_t,
}

extern "C" {
    pub fn compile_file_2(
        filename: *const ::std::os::raw::c_char,
        contents: *const ::std::os::raw::c_char,
        settings: *mut compiler_settings_t,
        size: *mut usize,
    ) -> *mut ::std::os::raw::c_void;
}

extern "C" {
    pub fn init_sk_shader();
}


pub type CompilerSettings = compiler_settings_t;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sksc_settings_t {
    pub debug: bool,
    pub row_major: bool,
    pub silent_info: bool,
    pub silent_err: bool,
    pub silent_warn: bool,
    pub optimize: ::std::os::raw::c_int,
    pub folder: [::std::os::raw::c_char; 512usize],
    pub vs_entrypoint: [::std::os::raw::c_char; 64usize],
    pub vs_entry_require: bool,
    pub ps_entrypoint: [::std::os::raw::c_char; 64usize],
    pub ps_entry_require: bool,
    pub cs_entrypoint: [::std::os::raw::c_char; 64usize],
    pub cs_entry_require: bool,
    pub shader_model: [::std::os::raw::c_char; 64usize],
    pub shader_model_str: [::std::os::raw::c_char; 16usize],
    pub gl_version: i32,
    pub include_folders: *mut *mut ::std::os::raw::c_char,
    pub include_folder_ct: i32,
    pub target_langs: [bool; 5usize],
}


impl CompilerSettings {
    pub fn new(sk_shader_c_settings: SkShaderCSettings) -> Self {
        Self {
            replace_ext: false,
            output_header: false,
            output_zipped: false,
            output_raw_shaders: false,
            only_if_changed: false,
            out_folder: null_mut(),
            shaderc: sksc_settings_t {
                debug: sk_shader_c_settings.debug,
                row_major: sk_shader_c_settings.row_major,
                silent_info: sk_shader_c_settings.silent_info,
                silent_err: sk_shader_c_settings.silent_err,
                silent_warn: sk_shader_c_settings.silent_warn,
                optimize: sk_shader_c_settings.optimize_level as i32,
                folder: [0; 512],
                vs_entrypoint: unsafe { let c = CString::new("vs").unwrap(); let b = c.as_bytes_with_nul(); let mut result: [u8; 64] = [0; 64]; result[..b.len()].copy_from_slice(b); std::mem::transmute(result) },
                vs_entry_require: false,
                ps_entrypoint: unsafe { let c = CString::new("ps").unwrap(); let b = c.as_bytes_with_nul(); let mut result: [u8; 64] = [0; 64]; result[..b.len()].copy_from_slice(b); std::mem::transmute(result) },
                ps_entry_require: false,
                cs_entrypoint: unsafe { let c = CString::new("cs").unwrap(); let b = c.as_bytes_with_nul(); let mut result: [u8; 64] = [0; 64]; result[..b.len()].copy_from_slice(b); std::mem::transmute(result) },
                cs_entry_require: false,
                shader_model: unsafe { let c = CString::new("5_0").unwrap(); let b = c.as_bytes_with_nul(); let mut result: [u8; 64] = [0; 64]; result[..b.len()].copy_from_slice(b); std::mem::transmute(result) },
                shader_model_str: unsafe { let c = CString::new("").unwrap(); let b = c.as_bytes_with_nul(); let mut result: [u8; 16] = [0; 16]; result[..b.len()].copy_from_slice(b); std::mem::transmute(result) },
                gl_version: 432,
                include_folders: null_mut(),
                include_folder_ct: 0,
                target_langs: [true, true, true, true, true],
            }
        }
    }
}

#[repr(C)]
pub struct SkShaderCSettings {
    pub debug: bool,
    pub row_major: bool,
    pub silent_info: bool,
    pub silent_err: bool,
    pub silent_warn: bool,
    pub optimize_level: u32,
}

// extern "C" {
//     pub fn sksc_log_clear();
// }

pub fn compile_shader_file(file_name: impl AsRef<str>, contents: impl AsRef<str>, mut settings: CompilerSettings) -> Vec<u8> {
    let file_name = CString::new(file_name.as_ref()).unwrap();
    let contents = CString::new(contents.as_ref()).unwrap();
    unsafe {
        let mut sks_size: usize = 0;
        let bytes: &[u8] = std::slice::from_raw_parts(compile_file_2(file_name.as_ptr(), contents.as_ptr(), &mut settings as *mut CompilerSettings, &mut sks_size as *mut usize) as *const u8, sks_size);
        bytes.to_vec()
    }
}

pub struct Test1 {
    a: u32,
}
impl Drop for Test1 {
    #[track_caller]
    fn drop(&mut self) {
        panic!()
    }
}

#[test]
fn test_compiler() {
    let contents = include_str!("../skshaderc/shaders/test.hlsl");
    let filename = "test.hlsl";
    unsafe { init_sk_shader(); }
    let bytes = compile_shader_file(filename, contents, CompilerSettings::new(SkShaderCSettings{
        debug: false,
        row_major: false,
        silent_info: false,
        silent_err: false,
        silent_warn: false,
        optimize_level: 0,
    }));

    let mut file = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open("temp.test").unwrap();
    file.write(&bytes).expect("TODO: panic message");
}