#ifndef WIIUSE_RUST_SHIM_H
#define WIIUSE_RUST_SHIM_H

#ifdef __cplusplus
extern "C" {
#endif

void *rust_wiiuse_init(int count);

int rust_wiiuse_find(void *devices, int max_devices, int timeout);

int rust_wiiuse_connect(void *devices, int count);

int rust_wiiuse_poll(void *devices, int count);

void rust_wiiuse_motion_sensing(void *devices, int index, int enabled);

void rust_wiiuse_get_gforce(void *devices, int index, float *x, float *y,
                            float *z);

void rust_wiiuse_cleanup(void *devices, int count);

#ifdef __cplusplus
}
#endif

#endif
