#include "wiiuse_shim.h"

#include <string.h>
#include <wiiuse.h>

int rust_wiiuse_get_state(void *devices, int index, int count,
                          rust_wiiuse_state_t *out) {
  struct wiimote_t **wiimotes = (struct wiimote_t **)devices;
  struct wiimote_t *wm;
  const struct nunchuk_t *nunchuk = NULL;

  if (wiimotes == NULL || out == NULL || index < 0 || count <= index) {
    return 0;
  }

  wm = wiimotes[index];
  if (wm == NULL) {
    return 0;
  }

  /* Clear optional Nunchuk fields when no Nunchuk is attached. */
  memset(out, 0, sizeof(*out));

  /*
   * In wiiuse, btns is the current button mask. btns_held is the subset
   * that was also down in the previous report; btns_released is the
   * transition mask for buttons released by the current report.
   */
  out->wiimote_buttons_down = wm->btns & WIIMOTE_BUTTON_ALL;
  out->wiimote_buttons_held = wm->btns_held & WIIMOTE_BUTTON_ALL;
  out->wiimote_buttons_released = wm->btns_released & WIIMOTE_BUTTON_ALL;

  out->wiimote_accel_x = wm->accel.x;
  out->wiimote_accel_y = wm->accel.y;
  out->wiimote_accel_z = wm->accel.z;

  out->wiimote_gforce_x = wm->gforce.x;
  out->wiimote_gforce_y = wm->gforce.y;
  out->wiimote_gforce_z = wm->gforce.z;

  /*
   * A regular Nunchuk occupies exp.nunchuk. With MotionPlus passthrough,
   * wiiuse keeps the attached Nunchuk behind exp.mp.nc instead.
   */
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

  if (nunchuk == NULL) {
    return 1;
  }

  out->has_nunchuk = 1;
  out->nunchuk_buttons_down =
      (uint8_t)(nunchuk->btns & NUNCHUK_BUTTON_ALL);
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

  return 1;
}
