import SwiftRs
import Tauri
import UIKit
import WebKit
import os

let logger = Logger(subsystem: "com.plugin.safe.area.insets.css", category: "InsetPlugin")



class InsetPlugin: Plugin {

  // MARK: - Chargement du plugin
  override func load(webview: WKWebView) {

    // Observer les notifications du clavier (équivalent de setOnApplyWindowInsetsListener sur Android)
    NotificationCenter.default.addObserver(
      self,
      selector: #selector(keyboardWillShow),
      name: UIResponder.keyboardWillShowNotification,
      object: nil
    )

    NotificationCenter.default.addObserver(
      self,
      selector: #selector(keyboardWillHide),
      name: UIResponder.keyboardWillHideNotification,
      object: nil
    )
  }

  // MARK: - Gestion clavier
  @objc func keyboardWillShow(notification: Notification) {
    trigger("keyboard_shown", data: [:])
  }

  @objc func keyboardWillHide(notification: Notification) {
    trigger("keyboard_hidden", data: [:])
  }

  // MARK: - Commande: obtenir le top inset (status bar / notch)
  @objc public func getTopInset(_ invoke: Invoke) throws {
    let window = UIApplication.shared.windows.first
    let topInset = window?.safeAreaInsets.top ?? 0
    let topInsetDIP = toDIPFromPixel(topInset)
    logger.info("Top inset: \(topInsetDIP)")

    invoke.resolve(["inset": topInsetDIP])
  }

  // MARK: - Commande: obtenir le bottom inset (home indicator / nav bar)
  @objc public func getBottomInset(_ invoke: Invoke) throws {
    let window = UIApplication.shared.windows.first
    let bottomInset = window?.safeAreaInsets.bottom ?? 0
    let bottomInsetDIP = toDIPFromPixel(bottomInset)
    logger.info("Bottom inset: \(bottomInsetDIP)")
    invoke.resolve(["inset": bottomInsetDIP])
  }

  // MARK: - Conversion PX -> DIP
  private func toDIPFromPixel(_ pixels: CGFloat) -> Double {
    let scale = UIScreen.main.scale
    return Double(pixels / scale)
  }
}

// MARK: - Initialisation plugin Tauri
@_cdecl("init_plugin_safe_area_insets_css")
func initPlugin() -> Plugin {
  return InsetPlugin()
}
