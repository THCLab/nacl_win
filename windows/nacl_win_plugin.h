#ifndef FLUTTER_PLUGIN_NACL_WIN_PLUGIN_H_
#define FLUTTER_PLUGIN_NACL_WIN_PLUGIN_H_

#include <flutter/method_channel.h>
#include <flutter/plugin_registrar_windows.h>

#include <memory>

namespace nacl_win {

class NaclWinPlugin : public flutter::Plugin {
 public:
  static void RegisterWithRegistrar(flutter::PluginRegistrarWindows *registrar);

  NaclWinPlugin();

  virtual ~NaclWinPlugin();

  // Disallow copy and assign.
  NaclWinPlugin(const NaclWinPlugin&) = delete;
  NaclWinPlugin& operator=(const NaclWinPlugin&) = delete;

 private:
  // Called when a method is called on this plugin's channel from Dart.
  void HandleMethodCall(
      const flutter::MethodCall<flutter::EncodableValue> &method_call,
      std::unique_ptr<flutter::MethodResult<flutter::EncodableValue>> result);
};

}  // namespace nacl_win

#endif  // FLUTTER_PLUGIN_NACL_WIN_PLUGIN_H_
