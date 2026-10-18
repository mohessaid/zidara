/** What a browser may say about the device (Chromium reports both; others report neither). */
export interface DeviceHints {
  connection?: { saveData?: boolean };
  deviceMemory?: number;
}

/**
 * Low-end phones and data saver start in the light experience: the table, with no game engine
 * loaded or running until the learner asks for the map. Browsers round memory to a power of
 * two, so some 3 GB phones report 2 and start light too; the map is one tap away. 4 GB and up
 * get the map.
 */
export function startsLight(device: DeviceHints): boolean {
  return device.connection?.saveData === true || (device.deviceMemory ?? Infinity) <= 2;
}
