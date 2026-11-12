#include "wiiuse_shim.h"
#include <string.h>
#include <wiiuse.h>

int rust_wiiuse_get_state(void *devices, int index, int count,
                          rust_wiiuse_state_t *out) {
  struct wiimote_t **wiimotes = (struct wiimote_t **)devices;
  struct wiimote_t *wm;
  const struct nunchuk_t *nunchuk = 0;

  if (wiimotes == 0 || out == 0 || index < 0 || count <= index) {
    return 0;
  }

  wm = wiimotes[index];
  if (wm == 0) {
    return 0;
  }

  /* Always initialize the entire value, including absent Nunchuk fields. */
  memset(out, 0, sizeof(*out));

  out->wiimote_buttons_down = wm->btns & WIIMOTE_BUTTON_ALL;
  out->wiimote_buttons_held = wm->btns_held & WIIMOTE_BUTTON_ALL;
  out->wiimote_buttons_released = wm->btns_released & WIIMOTE_BUTTON_ALL;
  out->wiimote_accel_x = wm->accel.x;
  out->wiimote_accel_y = wm->accel.y;
  out->wiimote_accel_z = wm->accel.z;
  out->wiimote_gforce_x = wm->gforce.x;
  out->wiimote_gforce_y = wm->gforce.y;
  out->wiimote_gforce_z = wm->gforce.z;

  /* A Nunchuk may be attached directly or through MotionPlus passthrough. */
  switch (wm->exp.type) {
  case EXP_NUNCHUK:
    nunchuk = &wm->exp.nunchuk;
    break;
  case EXP_MOTION_PLUS_NUNCHUK:
    nunchuk = wm->exp.mp.nc;
    break;
  default:
    break;
  }

  if (nunchuk == 0) {
    return 1;
  }

  out->has_nunchuk = 1;
  out->nunchuk_buttons_down = (uint8_t)(nunchuk->btns & NUNCHUK_BUTTON_ALL);
  out->nunchuk_buttons_held =
      (uint8_t)(nunchuk->btns_held & NUNCHUK_BUTTON_ALL);
  out->nunchuk_buttons_released =
      (uint8_t)(nunchuk->btns_released & NUNCHUK_BUTTON_ALL);
  out->nunchuk_accel_x = nunchuk->accel.x;
  out->nunchuk_accel_y = nunchuk->accel.y;
  out->nunchuk_accel_z = nunchuk->accel.z;
  out->nunchuk_gforce_x = nunchuk->gforce.x;
  out->nunchuk_gforce_y = nunchuk->gforce.y;
  out->nunchuk_gforce_z = nunchuk->gforce.z;
  out->nunchuk_joystick_x = nunchuk->js.x;
  out->nunchuk_joystick_y = nunchuk->js.y;

  return 1;
}
