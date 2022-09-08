import 'package:flutter/material.dart';
import 'dart:async';

import 'package:nacl_win/nacl_win.dart';

void main() {
  runApp(const MyApp());
}

class MyApp extends StatefulWidget {
  const MyApp({Key? key}) : super(key: key);

  @override
  State<MyApp> createState() => _MyAppState();
}

class _MyAppState extends State<MyApp> {
  String signature = 'Unknown';

  @override
  void initState() {
    super.initState();
    initPlatformState();
  }

  // Platform messages are asynchronous, so we initialize in an async method.
  Future<void> initPlatformState() async {
    String sig;
    var key = await NaclWin.generateKey();
    sig = await NaclWin.signMessage('message', key.privKey);

    setState(() {
      signature = sig;
    });
  }

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: Scaffold(
        appBar: AppBar(
          title: const Text('Plugin example app'),
        ),
        body: Center(
          child: Text('Signature: $signature\n'),
        ),
      ),
    );
  }
}
