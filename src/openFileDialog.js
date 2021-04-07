// This file is included in `bindings.rs`

export function getPayload() {
  window.__TAURI__.dialog.open();
  return new Date().toString();
}

export function getPayloadLater(callback) {
  setTimeout(() => {
    callback(getPayload());
  }, 1000);
}

