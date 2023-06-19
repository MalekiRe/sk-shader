//#pragma once

#include <stdbool.h>
#include "sksc.h"
///////////////////////////////////////////

typedef struct compiler_settings_t {
	bool replace_ext;
	bool output_header;
	bool output_zipped;
	bool output_raw_shaders;
	bool only_if_changed;
	char *out_folder;

	sksc_settings_t shaderc;
} compiler_settings_t;

extern "C" void* compile_file_2(const char*, const char *, compiler_settings_t *, size_t *);
extern "C" void init_sk_shader();
///////////////////////////////////////////