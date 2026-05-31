/// <reference types="vitest/globals" />

/* eslint-disable no-var */
declare global {
  var window: Window & typeof globalThis;
  var global: typeof globalThis;
  var navigator: Navigator;
  var localStorage: Storage;
  var FileReader: typeof FileReader;
  var performance: Performance;
  var confirm: (message?: string) => boolean;
}

export {};
