// tuipdf
// ------
// A beautifully crafted, terminal-native PDF tool built in Rust.
// It aims to make compressing PDF files as fast, efficient and flexible
// as possible directly from your terminal.
//
// Authors: KnightShadows Team and individual contributors (see CONTRIBUTORS file)
//          Aditya Anand <aditya19study@gmail.com> (c) 2025
// Website: https://github.com/KnightShadows/tuipdf
// License: MPL-2.0 (see LICENSE file)

#ifndef COMPRESS_H
#define COMPRESS_H
#include <stddef.h>

typedef enum {
    COMPRESS_LOW    = 0,
    COMPRESS_MEDIUM = 1,
    COMPRESS_HIGH   = 2
} CompressionLevel;

typedef struct {
    unsigned char* data;
    size_t size;
    int error_code;
    char error_msg[256];
} CompressResult;

CompressResult compress_pdf(const unsigned char* input, size_t input_size, CompressionLevel level);
void free_compress_result(CompressResult* result);

#endif