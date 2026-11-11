#ifndef WIIUSE_RUST_SHIM_H
#define WIIUSE_RUST_SHIM_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

/**
 * @file wiiuse_shim.h
 * @brief C interface between the Rust wiiuse-rs library and the native wiiuse
 * library.
 *
 * This shim provides a stable, fixed-layout interface for Rust to communicate
 * with the native wiiuse library.
 *
 * Native wiiuse structures and pointers are kept behind the C interface.
 * Controller state is copied into rust_wiiuse_state_t before being exposed
 * to Rust.
 *
 * All functions operate on the opaque device array returned by
 * rust_wiiuse_init().
 *
 * @note The device pointer must remain valid for the entire lifetime of the
 *       controller session.
 *
 * @note The caller must eventually release the device array using
 *       rust_wiiuse_cleanup().
 */

/**
 * @brief Fixed-layout snapshot of the latest Wii Remote, IR camera, and
 *        Nunchuk state.
 *
 * This structure contains a copy of the controller state, allowing Rust
 * to access controller data without depending on native wiiuse structures
 * or exposing native pointers.
 *
 * Button values are bitmasks using the corresponding wiiuse button flags.
 *
 * Accelerometer values are provided in two representations:
 * - Raw sensor readings as unsigned 16-bit integers.
 * - Acceleration expressed as G-force values using 32-bit floats.
 *
 * IR fields contain the camera status, estimated cursor position, resolution,
 * and information about up to four detected IR dots.
 *
 * Nunchuk fields are meaningful only when has_nunchuk is nonzero.
 *
 * @note This structure is shared across the C/Rust FFI boundary.
 *       Its field order and types must remain synchronized with the
 *       corresponding Rust representation.
 */
typedef struct rust_wiiuse_state_t {

  /**
   * @brief Bitmask of Wii Remote buttons currently reported as down.
   *
   * Each bit represents a Wii Remote button.
   * Interpret the bits using the corresponding wiiuse button constants.
   */
  uint16_t wiimote_buttons_down;

  /**
   * @brief Bitmask of Wii Remote buttons reported as held.
   *
   * Each bit represents a button that wiiuse reports as being held.
   */
  uint16_t wiimote_buttons_held;

  /**
   * @brief Bitmask of Wii Remote buttons released during the latest update.
   *
   * Each bit represents a button release reported by wiiuse.
   */
  uint16_t wiimote_buttons_released;

  /**
   * @brief Raw Wii Remote accelerometer X-axis reading.
   */
  uint16_t wiimote_accel_x;

  /**
   * @brief Raw Wii Remote accelerometer Y-axis reading.
   */
  uint16_t wiimote_accel_y;

  /**
   * @brief Raw Wii Remote accelerometer Z-axis reading.
   */
  uint16_t wiimote_accel_z;

  /**
   * @brief Wii Remote X-axis acceleration in G-force units.
   */
  float wiimote_gforce_x;

  /**
   * @brief Wii Remote Y-axis acceleration in G-force units.
   */
  float wiimote_gforce_y;

  /**
   * @brief Wii Remote Z-axis acceleration in G-force units.
   */
  float wiimote_gforce_z;

  /**
   * @brief Indicates whether the Wii Remote infrared camera is enabled.
   *
   * Nonzero if IR sensing is enabled.
   * Zero otherwise.
   */
  uint8_t ir_enabled;

  /**
   * @brief Number of currently visible infrared camera dots.
   *
   * The Wii Remote IR camera can detect up to four IR dots.
   */
  uint8_t ir_num_dots;

  /**
   * @brief Estimated infrared cursor X coordinate.
   *
   * Contains the estimated pointer position calculated by wiiuse.
   * Its coordinate range depends on the configured IR resolution.
   */
  int32_t ir_x;

  /**
   * @brief Estimated infrared cursor Y coordinate.
   *
   * Contains the estimated pointer position calculated by wiiuse.
   * Its coordinate range depends on the configured IR resolution.
   */
  int32_t ir_y;

  /**
   * @brief Horizontal infrared camera resolution.
   *
   * Represents the configured horizontal coordinate range.
   */
  uint32_t ir_vres_x;

  /**
   * @brief Vertical infrared camera resolution.
   *
   * Represents the configured vertical coordinate range.
   */
  uint32_t ir_vres_y;

  /**
   * @brief Visibility flags for the four infrared camera dots.
   *
   * Each element is nonzero if the corresponding dot is visible.
   * Zero indicates that the dot is not currently detected.
   */
  uint8_t ir_dot_visible[4];

  /**
   * @brief Reported size of each infrared camera dot.
   *
   * Each element corresponds to one of the four possible IR dots.
   */
  uint8_t ir_dot_size[4];

  /**
   * @brief Interpolated X coordinates for the four infrared camera dots.
   *
   * Each element corresponds to one of the four possible IR dots.
   */
  uint32_t ir_dot_x[4];

  /**
   * @brief Interpolated Y coordinates for the four infrared camera dots.
   *
   * Each element corresponds to one of the four possible IR dots.
   */
  uint32_t ir_dot_y[4];

  /**
   * @brief Raw X coordinates for the four infrared camera dots.
   *
   * Raw sensor coordinates normally fall within the 0..1023 range.
   * Each element corresponds to one of the four possible IR dots.
   */
  int16_t ir_dot_rx[4];

  /**
   * @brief Raw Y coordinates for the four infrared camera dots.
   *
   * Raw sensor coordinates normally fall within the 0..767 range.
   * Each element corresponds to one of the four possible IR dots.
   */
  int16_t ir_dot_ry[4];

  /**
   * @brief Indicates whether a Nunchuk extension is connected.
   *
   * Nonzero if a Nunchuk is connected and its state is available.
   * Zero otherwise.
   */
  uint8_t has_nunchuk;

  /**
   * @brief Bitmask of Nunchuk buttons currently reported as down.
   *
   * Button bits correspond to the Nunchuk button flags provided by wiiuse.
   */
  uint8_t nunchuk_buttons_down;

  /**
   * @brief Bitmask of Nunchuk buttons reported as held.
   */
  uint8_t nunchuk_buttons_held;

  /**
   * @brief Bitmask of Nunchuk buttons released during the latest update.
   */
  uint8_t nunchuk_buttons_released;

  /**
   * @brief Raw Nunchuk accelerometer X-axis reading.
   */
  uint16_t nunchuk_accel_x;

  /**
   * @brief Raw Nunchuk accelerometer Y-axis reading.
   */
  uint16_t nunchuk_accel_y;

  /**
   * @brief Raw Nunchuk accelerometer Z-axis reading.
   */
  uint16_t nunchuk_accel_z;

  /**
   * @brief Nunchuk X-axis acceleration in G-force units.
   */
  float nunchuk_gforce_x;

  /**
   * @brief Nunchuk Y-axis acceleration in G-force units.
   */
  float nunchuk_gforce_y;

  /**
   * @brief Nunchuk Z-axis acceleration in G-force units.
   */
  float nunchuk_gforce_z;

} rust_wiiuse_state_t;

/**
 * @brief Allocate and initialize a Wii Remote device array.
 *
 * Allocates the native wiiuse device array used for discovering, connecting
 * to, and communicating with Wii Remotes.
 *
 * The returned pointer is opaque to the Rust application and must only be
 * passed to the other functions in this shim.
 *
 * @param count Maximum number of Wii Remotes to manage.
 *              Must be greater than zero.
 *
 * @return Opaque pointer to the allocated device array, or NULL on failure.
 *
 * @note The returned pointer must eventually be passed to
 *       rust_wiiuse_cleanup().
 *
 * @note The allocated capacity determines the maximum number of devices
 *       that can be managed by this array.
 */
void *rust_wiiuse_init(int count);

/**
 * @brief Search for nearby Wii Remotes.
 *
 * Starts device discovery using the native wiiuse discovery mechanism.
 *
 * Discovered devices are stored in the supplied device array.
 *
 * @param devices Pointer returned by rust_wiiuse_init().
 * @param max_devices Maximum number of devices to discover.
 *                    Must not exceed the allocated capacity.
 * @param timeout Discovery timeout in seconds.
 *
 * @return Number of Wii Remotes discovered.
 *         Zero indicates that no devices were found.
 *
 * @note Wii Remotes should generally be placed into discovery mode by
 *       pressing 1 + 2 before starting discovery.
 */
int rust_wiiuse_find(void *devices, int max_devices, int timeout);

/**
 * @brief Connect to previously discovered Wii Remotes.
 *
 * Attempts to establish connections with the devices discovered by
 * rust_wiiuse_find().
 *
 * @param devices Pointer returned by rust_wiiuse_init().
 * @param count Number of discovered devices to connect to.
 *
 * @return Number of successfully connected Wii Remotes.
 *         Zero indicates that no connections were established.
 *
 * @note The count should normally be the number returned by
 *       rust_wiiuse_find().
 */
int rust_wiiuse_connect(void *devices, int count);

/**
 * @brief Process incoming controller data.
 *
 * Polls the connected Wii Remotes for new input and updates their internal
 * state.
 *
 * This function should be called regularly while the controllers are
 * connected. Button transitions, accelerometer readings, IR camera data,
 * and extension state are updated through polling.
 *
 * @param devices Pointer returned by rust_wiiuse_init().
 * @param count Number of connected devices to poll.
 *
 * @return Result returned by the native wiiuse polling operation.
 *
 * @note Call this function before rust_wiiuse_get_state() to obtain the
 *       latest available controller state.
 */
int rust_wiiuse_poll(void *devices, int count);

/**
 * @brief Enable or disable Wii Remote accelerometer reporting.
 *
 * Controls motion sensing for a specific Wii Remote.
 *
 * @param devices Pointer returned by rust_wiiuse_init().
 * @param count Number of valid devices in the device array.
 * @param index Zero-based index of the Wii Remote to configure.
 *              Must be less than count.
 * @param enabled Nonzero to enable motion sensing, zero to disable it.
 *
 * @note This controls Wii Remote motion sensing. It does not require a
 *       Nunchuk to be connected.
 */
void rust_wiiuse_motion_sensing(void *devices, int count, int index,
                                int enabled);

/**
 * @brief Enable or disable Wii Remote infrared camera reporting.
 *
 * Controls the infrared camera for a specific Wii Remote.
 *
 * When enabled, the camera can report detected IR dots and an estimated
 * cursor position. An external infrared source, such as a powered sensor
 * bar, is required for the camera to detect IR points.
 *
 * @param devices Pointer returned by rust_wiiuse_init().
 * @param count Number of valid devices in the device array.
 * @param index Zero-based index of the Wii Remote to configure.
 *              Must be less than count.
 * @param enabled Nonzero to enable IR sensing, zero to disable it.
 *
 * @note Enabling IR sensing does not require a Wii or Wii U console.
 */
void rust_wiiuse_ir_sensing(void *devices, int count, int index, int enabled);

/**
 * @brief Copy the latest controller state into a Rust-compatible snapshot.
 *
 * Retrieves the most recently polled state for the specified Wii Remote
 * and copies it into the caller-provided output structure.
 *
 * The returned snapshot contains button states, accelerometer readings,
 * IR camera data, and, when connected, Nunchuk extension data.
 *
 * No pointers to native wiiuse structures are exposed to the caller.
 *
 * @param devices Pointer returned by rust_wiiuse_init().
 * @param index Zero-based index of the Wii Remote to read.
 *              Must be less than count.
 * @param count Number of valid devices in the device array.
 * @param out Pointer to a rust_wiiuse_state_t structure that receives
 *            the copied controller state.
 *
 * @return Nonzero if the state was successfully copied.
 *         Zero if the requested state could not be retrieved.
 *
 * @note The caller must provide valid storage for out.
 *
 * @note The snapshot represents the most recently processed controller
 *       state. Call rust_wiiuse_poll() regularly to keep it updated.
 *
 * @note When IR sensing is disabled or no IR dots are visible, the IR
 *       coordinates should not be treated as a valid pointer position.
 *
 * @note When no Nunchuk is connected, has_nunchuk is zero and the Nunchuk
 *       fields should not be treated as valid sensor data.
 */
int rust_wiiuse_get_state(void *devices, int index, int count,
                          rust_wiiuse_state_t *out);

/**
 * @brief Disconnect and release the Wii Remote device array.
 *
 * Releases the native resources associated with the device array,
 * disconnects connected Wii Remotes, and frees the allocated memory.
 *
 * @param devices Pointer returned by rust_wiiuse_init().
 * @param count Number of devices originally allocated.
 *
 * @note Call this function exactly once for each successfully initialized
 *       device array.
 *
 * @warning The device pointer becomes invalid after this function returns.
 *          Do not use it in subsequent shim calls.
 */
void rust_wiiuse_cleanup(void *devices, int count);

#ifdef __cplusplus
}
#endif

#endif /* WIIUSE_RUST_SHIM_H */
