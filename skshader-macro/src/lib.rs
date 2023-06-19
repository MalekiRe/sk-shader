extern crate proc_macro;

use proc_macro::TokenStream;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;
use proc_macro2::{Literal, TokenTree};
use std::ffi::CString;
use std::fs::OpenOptions;
use std::hash::{Hash, Hasher, SipHasher};
use std::io::{Read, Write};
use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};

use syn::{parse_macro_input, DeriveInput, LitStr};
use quote::{quote, ToTokens};
use syn::__private::TokenStream2;
use syn::token::Token;
use skshaderc_bindings::{compile_shader_file, CompilerSettings, SkShaderCSettings};

static SHADER_NAMES: OnceLock<Mutex<HashSet<String>>> = OnceLock::new();

static OUT_DIR: OnceLock<String> = OnceLock::new();

fn get_name_set() -> &'static Mutex<HashSet<String>> {
    SHADER_NAMES.get_or_init(|| Mutex::new(HashSet::new()))
}

fn check_init() {
    OUT_DIR.get_or_init(|| {
        unsafe {
            skshaderc_bindings::init_sk_shader();
        }
        std::env::var("OUT_DIR").expect("need to add a build.rs file for the skshader macro to function").to_string()
    });
}

#[proc_macro]
pub fn include_shader(input: TokenStream) -> TokenStream {

    check_init();

    let input_stuff: TokenStream2 = parse_macro_input!(input as TokenStream2);

    let vec: Vec<_> = input_stuff.into_iter().collect();

    match vec.len() {
        1 => include_shader_file(vec.first().unwrap().clone()),
        3 => include_shader_string(vec.get(0).unwrap().clone(), vec.get(2).unwrap().clone()),
        _ => panic!("stereokit include_shader macro did not have the right number of arguments (1 or 2)"),
    }.into()
}

fn include_shader_file(file: proc_macro2::TokenTree) -> TokenStream2 {
    todo!()
}

fn include_shader_string(name: proc_macro2::TokenTree, contents: proc_macro2::TokenTree) -> TokenStream2 {
    let name = match name {
        TokenTree::Literal(literal) => literal,

        _ => panic!(),
    };
    let contents = match contents {
        TokenTree::Literal(literal) => literal,
        _ => panic!(),
    };

    let contents: LitStr = syn::parse2(contents.to_token_stream()).unwrap();
    let name: LitStr = syn::parse2(name.to_token_stream()).unwrap();

    let contents = contents.value();
    let name = name.value();

    let contents = cached_compile_shader_file(name, contents, CompilerSettings::new(SkShaderCSettings{
        debug: false,
        row_major: false,
        silent_info: false,
        silent_err: false,
        silent_warn: false,
        optimize_level: 3,
    }));

    let s = Literal::byte_string(&contents);
    quote! {
        #s
    }
}

fn cached_compile_shader_file(name: impl AsRef<str>, contents: impl AsRef<str>, settings: CompilerSettings) -> Vec<u8> {
    let name = name.as_ref();
    let content = contents.as_ref();

    let build_dir = OUT_DIR.get().unwrap();
    let mut path_buf = PathBuf::from(build_dir);
    path_buf.push("cached_shaders");
    std::fs::create_dir_all(path_buf.clone()).unwrap();
    let mut sks_path_buf = path_buf.clone();
    sks_path_buf.push(String::from(name) + ".sks");
    if get_name_set().lock().unwrap().contains(name) {
        panic!("skshader inline file: {} already named earlier, you have it in multiple places, name your skshader file something different", name);
    }

    let mut hasher = DefaultHasher::new();
    content.hash(&mut hasher);
    content.hash(&mut hasher);
    let hash = hasher.finish();
    let mut hash_path = path_buf.clone();
    hash_path.push(name.to_string() + ".hash");

    if sks_path_buf.exists() {
        if !hash_path.exists() {
            let mut hash_file = OpenOptions::new().create(true).write(true).truncate(true).open(hash_path.clone()).unwrap();
            hash_file.write_all(hash.to_string().as_bytes()).unwrap();
        } else {
            let mut hash_file = OpenOptions::new().create(true).append(true).write(true).read(true).open(hash_path.clone()).unwrap();
            let mut hash_file_bytes = String::new();
            hash_file.read_to_string(&mut hash_file_bytes).unwrap();
            if hash_file_bytes.parse::<u64>().unwrap() == hash {
                let mut file = OpenOptions::new().create(true).append(true).read(true).open(sks_path_buf).unwrap();
                let mut bytes = Vec::new();
                file.read_to_end(&mut bytes).unwrap();
                return bytes;
            }
        }
    }

    let mut hash_file = OpenOptions::new().create(true).write(true).truncate(true).read(true).open(hash_path.clone()).unwrap();
    hash_file.write_all(hash.to_string().as_bytes()).unwrap();
    let mut file = OpenOptions::new().create(true).write(true).truncate(true).open(sks_path_buf).unwrap();
    let bytes = compile_shader_file(name, content, settings);
    file.write_all(&bytes).unwrap();
    bytes
}