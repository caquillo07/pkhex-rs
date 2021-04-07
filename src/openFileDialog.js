// This file is included in `bindings.rs`

export function getPayload() {
  return new Date().toString();
}

export function getPayloadLater(callback) {
  let tauri = window.__TAURI__;
  tauri.dialog.open({ filter: "pk8" })
    .then((path) => tauri.fs.readBinaryFile(path))
    .then((buf) => callback(buf));
}

