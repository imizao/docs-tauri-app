<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { save } from "@tauri-apps/plugin-dialog";
import {
  NSpace,
  NUpload,
  NButton,
  NDataTable,
  NCard,
  NInput,
  NFormItem,
  NForm,
  NSpin,
  NEmpty,
  NImage,
  NGrid,
  NGi,
  useMessage,
} from "naive-ui";
import * as XLSX from "xlsx";

const message = useMessage();

const ollamaUrl = ref("http://127.0.0.1:11434");
const ollamaModel = ref("llava");
const fileList = ref([]);
const previewUrl = ref("");
const currentFile = ref(null);
const tableData = ref([]);
const tableColumns = ref([]);
const loading = ref(false);

function fileToBase64(file) {
  return new Promise((resolve, reject) => {
    const reader = new FileReader();
    reader.onload = () => {
      const dataUrl = reader.result;
      const base64 = dataUrl.split(",")[1] || "";
      resolve(base64);
    };
    reader.onerror = reject;
    reader.readAsDataURL(file);
  });
}

function extractJsonArray(text) {
  if (!text || typeof text !== "string") return null;
  const trimmed = text.trim();
  const codeBlock = trimmed.match(/```(?:json)?\s*([\s\S]*?)```/);
  const str = codeBlock ? codeBlock[1].trim() : trimmed;
  try {
    const parsed = JSON.parse(str);
    return Array.isArray(parsed) ? parsed : [parsed];
  } catch {
    const start = str.indexOf("[");
    const end = str.lastIndexOf("]");
    if (start !== -1 && end !== -1 && end > start) {
      try {
        return JSON.parse(str.slice(start, end + 1));
      } catch {
        return null;
      }
    }
  }
  return null;
}

function rowsToColumns(rows) {
  if (!rows.length) return [];
  const keys = Object.keys(rows[0]);
  return keys.map((key) => ({
    title: key,
    key,
    ellipsis: { tooltip: true },
  }));
}

async function handleRecognize() {
  if (!currentFile.value) {
    message.warning("请先上传一张图片");
    return;
  }
  loading.value = true;
  tableData.value = [];
  tableColumns.value = [];

  try {
    const base64 = await fileToBase64(currentFile.value);
    const url = `${ollamaUrl.value.replace(/\/$/, "")}/api/chat`;
    const prompt = `请识别这张图片中的文字或表格内容。将识别结果以 JSON 数组格式返回，每个元素为一行数据，对象的键为列名、值为单元格内容。例如：[{"姓名":"张三","年龄":"25"},{"姓名":"李四","年龄":"30"}]。只输出 JSON 数组，不要其他说明或 markdown 代码块标记。`;

    const res = await fetch(url, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({
        model: ollamaModel.value,
        messages: [{ role: "user", content: prompt, images: [base64] }],
        stream: false,
      }),
    });

    if (!res.ok) {
      const errText = await res.text();
      throw new Error(
        res.status === 404
          ? "请确认已安装并运行 Ollama，且已拉取视觉模型（如 llava）"
          : errText || `请求失败 ${res.status}`
      );
    }

    const data = await res.json();
    const content = data.message?.content || "";
    const rows = extractJsonArray(content);

    if (!rows || !rows.length) {
      message.warning(
        "未能解析出表格数据，请确认图片包含可识别的文字或表格。原始回复已显示在下方。"
      );
      tableData.value = [{ 原始回复: content }];
      tableColumns.value = [
        { title: "原始回复", key: "原始回复", ellipsis: { tooltip: true } },
      ];
      return;
    }

    tableColumns.value = rowsToColumns(rows);
    tableData.value = rows;
    message.success("识别完成");
  } catch (e) {
    message.error(e?.message || "识别失败");
  } finally {
    loading.value = false;
  }
}

async function handleExportExcel() {
  if (!tableData.value.length) {
    message.warning("没有可导出的数据，请先进行 AI 识别");
    return;
  }

  try {
    const ws = XLSX.utils.json_to_sheet(tableData.value);
    const wb = XLSX.utils.book_new();
    XLSX.utils.book_append_sheet(wb, ws, "识别结果");
    const wbout = XLSX.write(wb, { bookType: "xlsx", type: "array" });
    const base64 = btoa(
      String.fromCharCode.apply(null, new Uint8Array(wbout))
    );

    const path = await save({
      defaultPath: "识别结果.xlsx",
      filters: [{ name: "Excel", extensions: ["xlsx"] }],
    });
    if (!path) return;

    await invoke("save_file", { path, base64Content: base64 });
    message.success("已保存到 " + path);
  } catch (e) {
    message.error(e?.message || "导出失败");
  }
}

function handleUploadChange(list) {
  fileList.value = list ?? [];
  const first = fileList.value[0];
  if (first) {
    currentFile.value = first.file;
    previewUrl.value =
      first.url || (first.file && URL.createObjectURL(first.file));
  } else {
    currentFile.value = null;
    previewUrl.value = "";
  }
}

function handleRemove() {
  fileList.value = [];
  currentFile.value = null;
  previewUrl.value = "";
}
</script>

<template>
  <div class="app">
    <header class="header">
      <h1>图片识别转表格</h1>
      <p class="subtitle">上传图片 → AI 识别 → 导出 Excel</p>
    </header>

    <main class="main">
      <NSpace vertical :size="20">
        <NCard title="AI 设置（Ollama 本地）" size="small">
          <NForm
            label-placement="left"
            label-width="90"
            :style="{ maxWidth: '480px' }"
          >
            <NGrid :cols="2" :x-gap="12">
              <NGi>
                <NFormItem label="Ollama 地址">
                  <NInput
                    v-model:value="ollamaUrl"
                    placeholder="http://127.0.0.1:11434"
                  />
                </NFormItem>
              </NGi>
              <NGi>
                <NFormItem label="视觉模型">
                  <NInput
                    v-model:value="ollamaModel"
                    placeholder="llava"
                  />
                </NFormItem>
              </NGi>
            </NGrid>
          </NForm>
        </NCard>

        <NCard title="上传图片" size="small">
          <NUpload
            :max="1"
            accept="image/*"
            :file-list="fileList"
            @update:file-list="handleUploadChange"
            list-type="image-card"
            @remove="handleRemove"
          >
            <div class="upload-tip">点击或拖拽图片到此处</div>
          </NUpload>
          <div v-if="previewUrl" class="preview-wrap">
            <NImage :src="previewUrl" width="200" object-fit="contain" />
          </div>
        </NCard>

        <NSpace>
          <NButton
            type="primary"
            :loading="loading"
            :disabled="!currentFile"
            @click="handleRecognize"
          >
            AI 识别
          </NButton>
          <NButton
            :disabled="!tableData.length"
            @click="handleExportExcel"
          >
            导出 Excel
          </NButton>
        </NSpace>

        <NCard title="识别结果" size="small">
          <NSpin :show="loading">
            <NEmpty
              v-if="!loading && !tableData.length"
              description="上传图片并点击「AI 识别」"
            />
            <NDataTable
              v-else
              :columns="tableColumns"
              :data="tableData"
              :bordered="true"
              :single-line="false"
              max-height="360"
              scroll-x="1200"
            />
          </NSpin>
        </NCard>
      </NSpace>
    </main>
  </div>
</template>

<style scoped>
.app {
  max-width: 900px;
  margin: 0 auto;
}
.header {
  text-align: center;
  margin-bottom: 24px;
}
.header h1 {
  font-size: 1.75rem;
  font-weight: 700;
  margin: 0 0 8px 0;
  color: #111827;
}
.subtitle {
  margin: 0;
  color: #6b7280;
  font-size: 0.95rem;
}
.main {
  background: #fff;
  border-radius: 12px;
  padding: 20px;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.08);
}
.upload-tip {
  padding: 24px;
  color: #9ca3af;
  font-size: 14px;
}
.preview-wrap {
  margin-top: 12px;
}
</style>
