# Tauri Plugin safe-area-insets-css

A Tauri plugin to expose safe area insets as CSS variables for your frontend. This is useful for mobile applications where you need to account for notches, rounded corners, or system bars.

#Rust Side

## Add the crate to your Cargo.toml:
```toml
tauri-plugin-safe-area-insets-css = "0.1"
```


## Initialize the plugin in your Tauri application:
```rust
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_safe_area_insets_css::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

## JavaScript Side

Install the JS API:
```bash
npm install @saurl/tauri-plugin-safe-area-insets-css-api
```

Import it in your JS/TS file:
```ts
import '@saurl/tauri-plugin-safe-area-insets-css-api';
```

After initialization, the following CSS variables are available for use:

`--safe-area-inset-top`
`--safe-area-inset-bottom`

You can now use these in your CSS:
```css
body {
  padding-top: var(--safe-area-inset-top);
  padding-bottom: var(--safe-area-inset-bottom);
}
```
# ⚠️ Important Note

Do not run this plugin outside of a Tauri environment. Doing so will create an infinite loop that can significantly slow down your site.

# Notes

The plugin is configured to automatically set --safe-area-inset-bottom to 0 when the keyboard is visible.

It is fully operational on both iOS and Android.
