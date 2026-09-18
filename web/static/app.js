import init, { convert, formats, is_binary, version } from "./pkg/convfmt_web.js";

const el = {
  from: document.getElementById("from"),
  to: document.getElementById("to"),
  compact: document.getElementById("compact"),
  sortKeys: document.getElementById("sort-keys"),
  input: document.getElementById("input"),
  output: document.getElementById("output"),
  inputFile: document.getElementById("input-file"),
  outputFile: document.getElementById("output-file"),
  file: document.getElementById("file"),
  download: document.getElementById("download"),
  status: document.getElementById("status"),
  version: document.getElementById("version"),
};

const encoder = new TextEncoder();
const decoder = new TextDecoder();

/** Bytes of the uploaded file, used when the source format is binary. */
let uploadedBytes = null;
/** Object url of the last binary result, revoked before being replaced. */
let downloadUrl = null;

const DEFAULTS = { from: "json", to: "yaml" };
const SAMPLE = '{\n  "the_answer": 42,\n  "array": ["a", "b"],\n  "boolean": false,\n  "my_favorite_game": "Tricky Castle"\n}\n';

function fillFormats(select, selected) {
  for (const format of formats()) {
    const option = document.createElement("option");
    option.value = format;
    option.textContent = format;
    option.selected = format === selected;
    select.append(option);
  }
}

function setStatus(message) {
  el.status.textContent = message;
}

function syncBinaryPanels() {
  const binaryInput = is_binary(el.from.value);
  const binaryOutput = is_binary(el.to.value);

  el.input.hidden = binaryInput;
  el.inputFile.hidden = !binaryInput;
  el.output.hidden = binaryOutput;
  el.outputFile.hidden = !binaryOutput;

  if (!binaryInput) {
    uploadedBytes = null;
    el.file.value = "";
  }
  if (!binaryOutput) {
    clearDownload();
  }
}

function clearDownload() {
  if (downloadUrl !== null) {
    URL.revokeObjectURL(downloadUrl);
    downloadUrl = null;
  }
  el.download.disabled = true;
}

function offerDownload(bytes) {
  clearDownload();
  const blob = new Blob([bytes], { type: "application/octet-stream" });
  downloadUrl = URL.createObjectURL(blob);
  el.download.disabled = false;
}

function currentInput() {
  return is_binary(el.from.value) ? uploadedBytes : encoder.encode(el.input.value);
}

function run() {
  const input = currentInput();
  if (input === null) {
    setStatus("");
    return;
  }
  if (input.length === 0) {
    el.output.value = "";
    clearDownload();
    setStatus("");
    return;
  }

  try {
    const output = convert(
      input,
      el.from.value,
      el.to.value,
      el.compact.checked,
      el.sortKeys.checked,
    );
    setStatus("");
    if (is_binary(el.to.value)) {
      el.output.value = "";
      offerDownload(output);
    } else {
      clearDownload();
      el.output.value = decoder.decode(output);
    }
  } catch (error) {
    el.output.value = "";
    clearDownload();
    setStatus(String(error.message ?? error));
  }
}

function debounce(fn, delay) {
  let timer = null;
  return () => {
    clearTimeout(timer);
    timer = setTimeout(fn, delay);
  };
}

async function main() {
  await init();

  el.version.textContent = `v${version()}`;
  fillFormats(el.from, DEFAULTS.from);
  fillFormats(el.to, DEFAULTS.to);
  el.input.value = SAMPLE;
  syncBinaryPanels();

  const rerun = debounce(run, 150);
  el.input.addEventListener("input", rerun);
  for (const control of [el.compact, el.sortKeys]) {
    control.addEventListener("change", run);
  }
  for (const select of [el.from, el.to]) {
    select.addEventListener("change", () => {
      syncBinaryPanels();
      run();
    });
  }

  el.file.addEventListener("change", async () => {
    const file = el.file.files?.[0];
    uploadedBytes = file ? new Uint8Array(await file.arrayBuffer()) : null;
    run();
  });

  el.download.addEventListener("click", () => {
    if (downloadUrl === null) {
      return;
    }
    const link = document.createElement("a");
    link.href = downloadUrl;
    link.download = `output.${el.to.value}`;
    link.click();
  });

  run();
}

main().catch((error) => {
  setStatus(`failed to load convfmt: ${error}`);
});
