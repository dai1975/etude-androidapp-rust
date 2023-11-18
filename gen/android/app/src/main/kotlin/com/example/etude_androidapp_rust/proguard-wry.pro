# THIS FILE IS AUTO-GENERATED. DO NOT MODIFY!!

# Copyright 2020-2023 Tauri Programme within The Commons Conservancy
# SPDX-License-Identifier: Apache-2.0
# SPDX-License-Identifier: MIT

-keep class com.example.etude_androidapp_rust.* {
  native <methods>;
}

-keep class com.example.etude_androidapp_rust.WryActivity {
  public <init>(...);

  void setWebView(com.example.etude_androidapp_rust.RustWebView);
  java.lang.Class getAppClass(...);
  java.lang.String getVersion();
}

-keep class com.example.etude_androidapp_rust.Ipc {
  public <init>(...);

  @android.webkit.JavascriptInterface public <methods>;
}

-keep class com.example.etude_androidapp_rust.RustWebView {
  public <init>(...);

  void loadUrlMainThread(...);
  void loadHTMLMainThread(...);
  void setAutoPlay(...);
  void setUserAgent(...);
}

-keep class com.example.etude_androidapp_rust.RustWebChromeClient,com.example.etude_androidapp_rust.RustWebViewClient {
  public <init>(...);
}