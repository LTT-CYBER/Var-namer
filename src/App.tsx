import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

// 命名类型定义
type NamingType = "variable" | "function" | "project";

// Rust 返回的数据结构（和 Rust 端保持一致）
interface StyleResult {
  style: string;
  value: string;
}

interface NamingResult {
  naming_type: string;
  results: StyleResult[];
}

function App() {
  const [input, setInput] = useState("");
  const [selectedTypes, setSelectedTypes] = useState<NamingType[]>(["variable"]);
  const [results, setResults] = useState<NamingResult[]>([]);
  const [isLoading, setIsLoading] = useState(false);

  // 切换类型勾选
  const toggleType = (type: NamingType) => {
    setSelectedTypes((prev) =>
      prev.includes(type) ? prev.filter((t) => t !== type) : [...prev, type]
    );
  };

  // 调用 Rust 后端生成命名
  const handleGenerate = async () => {
    if (!input.trim()) return;

    setIsLoading(true);
    try {
      const data = await invoke<NamingResult[]>("name_it", {
        request: {
          meaning: input.trim(),
          naming_types: selectedTypes,
        },
      });
      setResults(data);
    } catch (error) {
      console.error("生成失败:", error);
    } finally {
      setIsLoading(false);
    }
  };

  // 处理回车键
  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === "Enter") {
      handleGenerate();
    }
  };

  // 复制到剪贴板
  const copyToClipboard = async (text: string) => {
    try {
      await navigator.clipboard.writeText(text);
    } catch {
      // 降级方案
      const textarea = document.createElement("textarea");
      textarea.value = text;
      document.body.appendChild(textarea);
      textarea.select();
      document.execCommand("copy");
      document.body.removeChild(textarea);
    }
  };

  // 类型中文映射
  const typeLabel: Record<string, string> = {
    variable: "变量命名",
    function: "函数命名",
    project: "项目/文件命名",
  };

  return (
    <div className="container">
      <h1>🏷️ 变量命名工具</h1>

      {/* 输入区域 */}
      <div className="input-section">
        <label>输入含义（中英文均可）</label>
        <div className="input-row">
          <input
            type="text"
            value={input}
            onChange={(e) => setInput(e.target.value)}
            onKeyDown={handleKeyDown}
            placeholder="例如：用户最大登录尝试次数"
          />
          <button onClick={handleGenerate} disabled={isLoading}>
            {isLoading ? "生成中..." : "生成"}
          </button>
        </div>
      </div>

      {/* 命名类型选择 */}
      <div className="type-section">
        <span>命名类型：</span>
        <label>
          <input
            type="checkbox"
            checked={selectedTypes.includes("variable")}
            onChange={() => toggleType("variable")}
          />
          变量命名
        </label>
        <label>
          <input
            type="checkbox"
            checked={selectedTypes.includes("function")}
            onChange={() => toggleType("function")}
          />
          函数命名
        </label>
        <label>
          <input
            type="checkbox"
            checked={selectedTypes.includes("project")}
            onChange={() => toggleType("project")}
          />
          项目/文件命名
        </label>
      </div>

      {/* 结果展示区 */}
      <div className="results-container">
        {results.length === 0 ? (
          <div className="result-placeholder">
            输入含义后点击"生成"或按回车查看结果
          </div>
        ) : (
          results.map((nr) => (
            <div key={nr.naming_type} className="naming-group">
              <h3>{typeLabel[nr.naming_type] || nr.naming_type}</h3>
              {nr.results.map((r) => (
                <div key={r.style} className="result-row">
                  <span className="style-label">{r.style}</span>
                  <code className="result-value">{r.value}</code>
                  <button
                    className="copy-btn"
                    onClick={() => copyToClipboard(r.value)}
                    title="复制"
                  >
                    📋
                  </button>
                </div>
              ))}
            </div>
          ))
        )}
      </div>
    </div>
  );
}

export default App;