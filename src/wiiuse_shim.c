#include "wiiuse_shim.h"
#include <wiiuse.h>

void *rust_wiiuse_init(int count) {
  if (count <= 0) {
    return 0;
  }
  return (void *)wiiuse_init(count);
}

int rust_wiiuse_find(void *devices, int max_devices, int timeout) {
  if (devices == 0 || max_devices <= 0 || timeout < 0) {
    return 0;
  }
  return wiiuse_find((struct wiimote_t **)devices, max_devices, timeout);
}

int rust_wiiuse_connect(void *devices, int count) {
  if (devices == 0 || count <= 0) {
    return 0;
  }
  return wiiuse_connect((struct wiimote_t **)devices, count);
}

int rust_wiiuse_poll(void *devices, int count) {
  if (devices == 0 || count <= 0) {
    return 0;
  }
  return wiiuse_poll((struct wiimote_t **)devices, count);
}

void rust_wiiuse_motion_sensing(void *devices, int count, int index, int enabled) {
  struct wiimote_t **wm = (struct wiimote_t **)devices;

  if (wm == 0 || count <= 0 || index < 0 || index >= count || wm[index] == 0) {
    return;
  }
  wiiuse_motion_sensing(wm[index], enabled != 0);
}

void rust_wiiuse_cleanup(void *devices, int count) {
  if (devices == 0 || count <= 0) {
    return;
  }
  wiiuse_cleanup((struct wiimote_t **)devices, count);
}
