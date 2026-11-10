#include "wiiuse_shim.h"
#include <wiiuse.h>

void *rust_wiiuse_init(int count) { return (void *)wiiuse_init(count); }

int rust_wiiuse_find(void *devices, int max_devices, int timeout) {
  return wiiuse_find((struct wiimote_t **)devices, max_devices, timeout);
}

int rust_wiiuse_connect(void *devices, int count) {
  return wiiuse_connect((struct wiimote_t **)devices, count);
}

int rust_wiiuse_poll(void *devices, int count) {
  return wiiuse_poll((struct wiimote_t **)devices, count);
}

void rust_wiiuse_motion_sensing(void *devices, int index, int enabled) {
  struct wiimote_t **wm = (struct wiimote_t **)devices;

  if (wm && wm[index]) {
    wiiuse_motion_sensing(wm[index], enabled);
  }
}

void rust_wiiuse_get_gforce(void *devices, int index, float *x, float *y,
                            float *z) {
  struct wiimote_t **wm = (struct wiimote_t **)devices;

  if (!wm || !wm[index]) {
    return;
  }

  if (x)
    *x = wm[index]->gforce.x;
  if (y)
    *y = wm[index]->gforce.y;
  if (z)
    *z = wm[index]->gforce.z;
}

void rust_wiiuse_cleanup(void *devices, int count) {
  wiiuse_cleanup((struct wiimote_t **)devices, count);
}
