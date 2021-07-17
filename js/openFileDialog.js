// This file is included in `bindings.rs`

/**
 * @param {Object} rust callback; this callback
 * will include a Vec<u8> containing the binary file for the pk8
 */
export function openPK8File(callback) {
  let tauri = window.__TAURI__;
  tauri.dialog.open({ filter: "pk8" })
    .then((path) => tauri.fs.readBinaryFile(path))
    .then((buf) => callback(buf));
}

