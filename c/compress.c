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

#include "compress.h"
#include <fitz.h>
#include <pdf.h>
#include <pdf/clean.h>
#include <pdf/image-rewriter.h>
#include <pdf/font.h>
#include <string.h>
#include <stdio.h>
#include <stdlib.h>

static void set_error(CompressResult* res, const char* msg) {
    res->error_code = 1;
    strncpy(res->error_msg, msg, sizeof(res->error_msg) - 1);
    res->error_msg[sizeof(res->error_msg) - 1] = '\0';
}


static void configure_image_opts(pdf_image_rewriter_options *img, CompressionLevel level) {
    memset(img, 0, sizeof(*img));

    switch (level) {
    case COMPRESS_LOW:
        img->color_lossless_image_subsample_method = FZ_SUBSAMPLE_BICUBIC;
        img->color_lossy_image_subsample_method    = FZ_SUBSAMPLE_BICUBIC;
        img->color_lossless_image_subsample_threshold = 300;
        img->color_lossless_image_subsample_to        = 200;
        img->color_lossy_image_subsample_threshold    = 300;
        img->color_lossy_image_subsample_to           = 200;
        img->color_lossless_image_recompress_method = FZ_RECOMPRESS_JPEG;
        img->color_lossy_image_recompress_method    = FZ_RECOMPRESS_SAME;
        img->color_lossless_image_recompress_quality = "85";
        img->color_lossy_image_recompress_quality    = "85";

        img->gray_lossless_image_subsample_method = FZ_SUBSAMPLE_BICUBIC;
        img->gray_lossy_image_subsample_method    = FZ_SUBSAMPLE_BICUBIC;
        img->gray_lossless_image_subsample_threshold = 300;
        img->gray_lossless_image_subsample_to        = 200;
        img->gray_lossy_image_subsample_threshold    = 300;
        img->gray_lossy_image_subsample_to           = 200;
        img->gray_lossless_image_recompress_method = FZ_RECOMPRESS_JPEG;
        img->gray_lossy_image_recompress_method    = FZ_RECOMPRESS_SAME;
        img->gray_lossless_image_recompress_quality = "85";
        img->gray_lossy_image_recompress_quality    = "85";

        img->bitonal_image_subsample_method    = FZ_SUBSAMPLE_AVERAGE;
        img->bitonal_image_subsample_threshold = 0;
        img->bitonal_image_recompress_method   = FZ_RECOMPRESS_SAME;
        break;

    case COMPRESS_MEDIUM:
        img->color_lossless_image_subsample_method = FZ_SUBSAMPLE_BICUBIC;
        img->color_lossy_image_subsample_method    = FZ_SUBSAMPLE_BICUBIC;
        img->color_lossless_image_subsample_threshold = 200;
        img->color_lossless_image_subsample_to        = 150;
        img->color_lossy_image_subsample_threshold    = 200;
        img->color_lossy_image_subsample_to           = 150;
        img->color_lossless_image_recompress_method = FZ_RECOMPRESS_JPEG;
        img->color_lossy_image_recompress_method    = FZ_RECOMPRESS_JPEG;
        img->color_lossless_image_recompress_quality = "65";
        img->color_lossy_image_recompress_quality    = "70";

        img->gray_lossless_image_subsample_method = FZ_SUBSAMPLE_BICUBIC;
        img->gray_lossy_image_subsample_method    = FZ_SUBSAMPLE_BICUBIC;
        img->gray_lossless_image_subsample_threshold = 200;
        img->gray_lossless_image_subsample_to        = 150;
        img->gray_lossy_image_subsample_threshold    = 200;
        img->gray_lossy_image_subsample_to           = 150;
        img->gray_lossless_image_recompress_method = FZ_RECOMPRESS_JPEG;
        img->gray_lossy_image_recompress_method    = FZ_RECOMPRESS_JPEG;
        img->gray_lossless_image_recompress_quality = "65";
        img->gray_lossy_image_recompress_quality    = "70";

        img->bitonal_image_subsample_method    = FZ_SUBSAMPLE_AVERAGE;
        img->bitonal_image_subsample_threshold = 300;
        img->bitonal_image_subsample_to        = 200;
        img->bitonal_image_recompress_method   = FZ_RECOMPRESS_FAX;
        break;

    case COMPRESS_HIGH:
        img->color_lossless_image_subsample_method = FZ_SUBSAMPLE_BICUBIC;
        img->color_lossy_image_subsample_method    = FZ_SUBSAMPLE_BICUBIC;
        img->color_lossless_image_subsample_threshold = 150;
        img->color_lossless_image_subsample_to        = 100;
        img->color_lossy_image_subsample_threshold    = 150;
        img->color_lossy_image_subsample_to           = 100;
        img->color_lossless_image_recompress_method = FZ_RECOMPRESS_JPEG;
        img->color_lossy_image_recompress_method    = FZ_RECOMPRESS_JPEG;
        img->color_lossless_image_recompress_quality = "40";
        img->color_lossy_image_recompress_quality    = "45";

        img->gray_lossless_image_subsample_method = FZ_SUBSAMPLE_BICUBIC;
        img->gray_lossy_image_subsample_method    = FZ_SUBSAMPLE_BICUBIC;
        img->gray_lossless_image_subsample_threshold = 150;
        img->gray_lossless_image_subsample_to        = 100;
        img->gray_lossy_image_subsample_threshold    = 150;
        img->gray_lossy_image_subsample_to           = 100;
        img->gray_lossless_image_recompress_method = FZ_RECOMPRESS_JPEG;
        img->gray_lossy_image_recompress_method    = FZ_RECOMPRESS_JPEG;
        img->gray_lossless_image_recompress_quality = "40";
        img->gray_lossy_image_recompress_quality    = "45";

        img->bitonal_image_subsample_method    = FZ_SUBSAMPLE_AVERAGE;
        img->bitonal_image_subsample_threshold = 200;
        img->bitonal_image_subsample_to        = 150;
        img->bitonal_image_recompress_method   = FZ_RECOMPRESS_FAX;
        break;
    }
}

CompressResult compress_pdf(const unsigned char* input, size_t input_size, CompressionLevel level) {
    CompressResult res = {0};
    fz_context *ctx = NULL;
    fz_stream *stm = NULL;
    pdf_document *doc = NULL;
    fz_buffer *out_buf = NULL;
    fz_output *out = NULL;

    ctx = fz_new_context(NULL, NULL, FZ_STORE_UNLIMITED);
    if (!ctx) {
        set_error(&res, "Failed to create MuPDF context");
        return res;
    }

    fz_register_document_handlers(ctx);

    fz_try(ctx) {
        stm = fz_open_memory(ctx, input, input_size);
        doc = pdf_open_document_with_stream(ctx, stm);

        {
            pdf_image_rewriter_options img_opts;
            configure_image_opts(&img_opts, level);
            pdf_rewrite_images(ctx, doc, &img_opts);
        }

        if (level >= COMPRESS_MEDIUM) {
            fz_try(ctx) {
                pdf_subset_fonts(ctx, doc, 0, NULL);
            }
            fz_catch(ctx) {
            }
        }

        if (level == COMPRESS_HIGH) {
            int i, n = pdf_count_pages(ctx, doc);
            for (i = 0; i < n; i++) {
                pdf_page *page = pdf_load_page(ctx, doc, i);
                fz_try(ctx) {
                    pdf_annot *annot = pdf_first_annot(ctx, page);
                    while (annot) {
                        pdf_delete_annot(ctx, page, annot);
                        annot = pdf_first_annot(ctx, page);
                    }
                }
                fz_catch(ctx) { }
                fz_drop_page(ctx, (fz_page *)page);
            }
        }

        pdf_write_options opts = pdf_default_write_options;
        opts.do_compress       = 1;
        opts.do_compress_images = 1;
        opts.do_compress_fonts  = 1;
        opts.do_garbage         = 4;
        opts.do_sanitize        = 1;
        opts.do_clean           = 1;
        opts.do_use_objstms     = 1;
        opts.compression_effort = 100;

        if (level == COMPRESS_HIGH) {
            opts.do_decompress = 1;
        }

        out_buf = fz_new_buffer(ctx, input_size);
        out = fz_new_output_with_buffer(ctx, out_buf);
        pdf_write_document(ctx, doc, out, &opts);

        fz_close_output(ctx, out);
        fz_drop_output(ctx, out);
        out = NULL;

        size_t out_len = fz_buffer_storage(ctx, out_buf, &res.data);
        res.size = out_len;
        
        unsigned char* persistent_data = (unsigned char*)malloc(res.size);
        if (persistent_data) {
            memcpy(persistent_data, res.data, res.size);
            res.data = persistent_data;
        } else {
            set_error(&res, "Memory allocation failed for result buffer");
        }

    } fz_catch(ctx) {
        set_error(&res, fz_caught_message(ctx));
    }

    if (out) {
        fz_try(ctx) fz_close_output(ctx, out); fz_catch(ctx) {}
        fz_drop_output(ctx, out);
    }
    if (out_buf) fz_drop_buffer(ctx, out_buf);
    if (doc) pdf_drop_document(ctx, doc);
    if (stm) fz_drop_stream(ctx, stm);
    if (ctx) fz_drop_context(ctx);

    return res;
}

void free_compress_result(CompressResult* result) {
    if (result && result->data) {
        free(result->data);
        result->data = NULL;
        result->size = 0;
    }
}