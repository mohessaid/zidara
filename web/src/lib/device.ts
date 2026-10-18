/** What a browser may say about the device (Chromium reports both; others report neither). */
export interface DeviceHints {
  connection?: { saveData?: boolean };
  deviceMemory?: number;
}

/**
 * Low-end phones and data saver start in the light experience: the table, with no game engine
 * loaded or running until the learner asks for the map. Economy phones (3–4 GB) get the map.
 */
export function startsLight(device: DeviceHints): boolean {
  return device.connection?.saveData === true || (device.deviceMemory ?? Infinity) <= 2;
}
