#include "openjpeg.h"

// returns 0 on success, positive error number otherwise.
// codec_in = OPJ_CODEC_J2K or OPJ_CODEC_JP2 or ...
// TODO: output struct
int wrapper_read_buffer(char *data_in, int data_len, int codec_in) {
    opj_image_t *p_image = NULL;

    // Setup the stream.
    opj_stream_t *p_stream = opj_stream_create(data_len, 1);
    opj_stream_set_user_data(p_stream, data_in, NULL);
    opj_stream_set_user_data_length(p_stream, data_len);

    // Setup the decoder.
    opj_codec_t *p_codec = opj_create_decompress(codec_in);
    if (!p_codec) {
        opj_destroy_codec(p_codec);
        return 1;
    }
    opj_dparameters_t p_params;
    opj_set_default_decoder_parameters(&p_params);
    if (!opj_setup_decoder(p_codec, &p_params)) {
        opj_stream_destroy(p_stream);
        opj_destroy_codec(p_codec);
        return 2;
    }

    if (!opj_read_header(p_stream, p_codec, &p_image) || (p_image->numcomps == 0)) {
        opj_stream_destroy(p_stream);
        opj_destroy_codec(p_codec);
        if (p_image)
            opj_image_destroy(p_image);
        return 3;
    }



    return 0;
}
