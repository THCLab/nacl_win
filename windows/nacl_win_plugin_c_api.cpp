#include "include/nacl_win/nacl_win_plugin_c_api.h"

#include <flutter/plugin_registrar_windows.h>

#include "nacl_win_plugin.h"

void NaclWinPluginCApiRegisterWithRegistrar(
    FlutterDesktopPluginRegistrarRef registrar) {
  nacl_win::NaclWinPlugin::RegisterWithRegistrar(
      flutter::PluginRegistrarManager::GetInstance()
          ->GetRegistrar<flutter::PluginRegistrarWindows>(registrar));
}
