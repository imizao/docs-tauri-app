# 图片识别转表格（Tauri + Vue 3）

上传图片 → 本地 Ollama 视觉模型识别 → 导出 Excel 表格。

## Linux 构建依赖

在 Ubuntu/Debian 上编译前需安装 Tauri 2 所需的系统库（WebKit2GTK 4.1、libsoup3、JavaScriptCore 等）：

```bash
sudo apt update
sudo apt install -y libwebkit2gtk-4.1-dev libjavascriptcoregtk-4.1-dev libsoup-3.0-dev \
  libgtk-3-dev build-essential curl wget libappindicator3-dev librsvg2-dev patchelf
```

安装完成后执行 `pnpm tauri build` 或 `pnpm tauri dev`。

---

This template should help get you started developing with Tauri + Vue 3 in Vite. The template uses Vue 3 `<script setup>` SFCs, check out the [script setup docs](https://v3.vuejs.org/api/sfc-script-setup.html#sfc-script-setup) to learn more.

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Vue - Official](https://marketplace.visualstudio.com/items?itemName=Vue.volar) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
