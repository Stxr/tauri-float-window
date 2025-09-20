import  { useEffect, useState } from "react";
import { createRoot } from "react-dom/client";
import { invoke } from "@tauri-apps/api/core";

function App() {
  const [url, setUrl] = useState("");
  const [status, setStatus] = useState<string | null>(null);

  useEffect(() => {
    (async () => {
      try {
        const saved = (await invoke<string | null>("get_saved_url")) || "";
        setUrl(saved);
      } catch (e) {
        // ignore
      }
    })();
  }, []);

  async function save() {
    try {
      await invoke("save_url", { url });
      setStatus("已保存");
    } catch (e) {
      setStatus(String(e));
    }
  }

  async function saveAndLoad() {
    await save();
    try {
      await invoke("navigate_to", { url });
      setStatus("已保存并加载");
    } catch (e) {
      setStatus(String(e));
    }
  }

  function close() {
    window.close();
  }

  return (
    <div style={{ fontFamily: "-apple-system, BlinkMacSystemFont, Segoe UI, Roboto, Arial", padding: 16 }}>
      <h3 style={{ marginTop: 0 }}>设置加载 URL</h3>
      <input
        autoFocus
        value={url}
        onChange={(e) => setUrl(e.currentTarget.value)}
        placeholder="例如：http://127.0.0.1:8080"
        style={{ width: "100%", padding: 8, boxSizing: "border-box" }}
      />
      <div style={{ marginTop: 12, display: "flex", gap: 8 }}>
        <button onClick={saveAndLoad}>保存并加载</button>
        <button onClick={save}>仅保存</button>
        <button onClick={close} style={{ marginLeft: "auto" }}>关闭</button>
      </div>
      {status && <div style={{ marginTop: 8, color: "#666" }}>{status}</div>}
      <div style={{ marginTop: 8, color: "#999", fontSize: 12 }}>
        提示：开发模式下会覆盖窗口指向该 URL；打包后仍加载本地前端。
      </div>
    </div>
  );
}

const root = createRoot(document.getElementById("root")!);
root.render(<App />);

