# nacl_win
A flutter plugin for Windows for generating Ed25519 keys and signing messages. It uses a Rust code and a generated `.dll` file. The `flutter_rust_bridge` plugin connects the Rust code to the Dart one.

### Available methods:
- `generateKey()` - generates a `EdKeyPair`. It is a struct that contains base64 strings of public and private Ed25519 keys.
- `signMessage(String message, String key)` - Signs provided message with provided Ed25519 key.


### Exceptions:
- If the sodium library fails to initialize, the plugin throws a `InitializationFailedException`. Most likely the fault of this exception would be on the Rust crate side and it may not be related to plugin.
- If the key provided to `signMessage` method is incorrect (too short, not base64, etc), the plugin will throw `KeyDecodingFailedException` as the Rust code fails to decode the key from base64 to vector.
- If somehow the key decoding succeeds, but the Rust fails to transform the key from vector to sodium's SecretKey, the plugin will throw `KeyTransformingFailedException`.


### Usage:
```dart=
var key = await NaclWin.generateKey();
var sig = await NaclWin.signMessage('message', key.privKey);
```
For complete example see `example/lib/main.dart`
