import { invoke,addPluginListener, PluginListener } from '@tauri-apps/api/core'

interface GetInsetResponse {
  inset: number
}

export async function getTopInset(): Promise<GetInsetResponse | null> {
  return await invoke<GetInsetResponse>('plugin:safe-area-insets-css|get_top_inset', {
    payload: {},
  });
}

export async function getBottomInset(): Promise<GetInsetResponse | null> {
  return await invoke<GetInsetResponse>('plugin:safe-area-insets-css|get_bottom_inset', {
    payload: {},
  });
}

export async function onKeyboardShown(
  handler: () => void
): Promise<PluginListener> {
  return await addPluginListener(
    'safe-area-insets-css',
    'keyboard_shown',
    handler
  );
}

export async function onKeyboardHidden(
  handler: () => void
): Promise<PluginListener> {
  return await addPluginListener(
    'safe-area-insets-css',
    'keyboard_hidden',
    handler
  );
}

async function init() {
  const topInset = await getTopInset();
  const bottomInset = await getBottomInset();
  if (topInset) {
    document.documentElement.style.setProperty('--safe-area-inset-top', `${topInset?.inset}px`);
  }
  if (bottomInset) {
    document.documentElement.style.setProperty('--safe-area-inset-bottom', `${bottomInset?.inset}px`);
  }

  onKeyboardShown(() => {
    document.documentElement.style.setProperty('--safe-area-inset-bottom', `0px`);
  });

  onKeyboardHidden(() => {
    document.documentElement.style.setProperty('--safe-area-inset-bottom', `${bottomInset?.inset}px`);
  });
}

async function waitForTauritoLoad() {
  while (typeof (window as any).__TAURI_INTERNALS__ === "undefined") {
    await new Promise((resolve) => setTimeout(resolve, 50)); // check toutes les 50ms
  }

  console.log("Tauri ready, lancement de init()");
  init();
}
waitForTauritoLoad()