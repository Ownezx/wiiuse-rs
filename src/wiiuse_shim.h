#ifndef WIIUSE_RUST_SHIM_H
#define WIIUSE_RUST_SHIM_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif


/*
 * Stable, copy-only snapshot of the Wii Remote and Nunchuk input state.
 * These fields deliberately avoid exposing wiiuse's internal structs/unions
 * across the Rust FFI boundary.
 */
typedef struct rust_wiiuse_state_t {
    uint16_t wiimote_buttons_down;
    uint16_t wiimote_buttons_held;
    uint16_t wiimote_buttons_released;

    uint16_t wiimote_accel_x;
    uint16_t wiimote_accel_y;
    uint16_t wiimote_accel_z;

    float wiimote_gforce_x;
    float wiimote_gforce_y;
    float wiimote_gforce_z;

    uint8_t has_nunchuk;
    uint8_t nunchuk_buttons_down;
    uint8_t nunchuk_buttons_held;
    uint8_t nunchuk_buttons_released;

    uint16_t nunchuk_accel_x;
    uint16_t nunchuk_accel_y;
    uint16_t nunchuk_accel_z;

    float nunchuk_gforce_x;
    float nunchuk_gforce_y;
    float nunchuk_gforce_z;
} rust_wiiuse_state_t;

/*
 * Copies the latest state for devices[index] into out.
 * Returns 1 when the device exists and 0 for invalid arguments/device.
 */
int rust_wiiuse_get_state(void *devices, int index, int count,
                          rust_wiiuse_state_t *out);


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
