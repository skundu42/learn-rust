"use client";

import { useState, useCallback, useRef } from "react";
import CodeMirror from "@uiw/react-codemirror";
import { rust } from "@codemirror/lang-rust";
import { oneDark } from "@codemirror/theme-one-dark";
import {
  Play,
  RotateCcw,
  Copy,
  Check,
  Loader2,
  Terminal,
  AlertCircle,
  ChevronDown,
  ChevronUp,
} from "lucide-react";
import { cn } from "@/lib/utils";

interface RunResult {
  stdout: string;
  stderr: string;
  success: boolean;
}

interface RustIDEProps {
  code: string;
  initialCode: string;
  onCodeChange: (code: string) => void;
}

export default function RustIDE({ code, initialCode, onCodeChange }: RustIDEProps) {
  const [result, setResult] = useState<RunResult | null>(null);
  const [running, setRunning] = useState(false);
  const [copied, setCopied] = useState(false);
  const [outputOpen, setOutputOpen] = useState(true);
  const abortRef = useRef<AbortController | null>(null);

  const runCode = useCallback(async () => {
    if (running) {
      abortRef.current?.abort();
      setRunning(false);
      return;
    }

    setRunning(true);
    setResult(null);
    setOutputOpen(true);

    const abort = new AbortController();
    abortRef.current = abort;

    try {
      // Use the Rust Playground API
      const response = await fetch(
        "https://play.rust-lang.org/execute",
        {
          method: "POST",
          headers: { "Content-Type": "application/json" },
          body: JSON.stringify({
            channel: "stable",
            mode: "debug",
            edition: "2021",
            crateType: "bin",
            tests: false,
            code,
            backtrace: false,
          }),
          signal: abort.signal,
        }
      );

      if (!response.ok) {
        throw new Error(`HTTP ${response.status}: ${response.statusText}`);
      }

      const data = await response.json();
      setResult({
        stdout: data.stdout || "",
        stderr: data.stderr || "",
        success: data.success,
      });
    } catch (err: unknown) {
      if (err instanceof Error && err.name === "AbortError") {
        setResult({ stdout: "", stderr: "Execution cancelled.", success: false });
      } else {
        setResult({
          stdout: "",
          stderr: err instanceof Error ? err.message : "Failed to connect to Rust Playground. Check your internet connection.",
          success: false,
        });
      }
    } finally {
      setRunning(false);
    }
  }, [code, running]);

  const resetCode = () => {
    onCodeChange(initialCode);
    setResult(null);
  };

  const copyCode = async () => {
    await navigator.clipboard.writeText(code);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  const hasOutput = result && (result.stdout || result.stderr);

  return (
    <div className="flex flex-col h-full min-h-0 bg-[#0d1117]">
      {/* IDE Toolbar */}
      <div className="flex items-center justify-between px-3 py-2 border-b border-[#21262d] bg-[#161b22] shrink-0">
        <div className="flex items-center gap-2">
          <span className="text-xs text-[#484f58] font-mono">main.rs</span>
          <span className="text-xs text-[#484f58]">·</span>
          <span className="text-xs text-[#484f58] font-mono">Rust 2021</span>
        </div>
        <div className="flex items-center gap-1">
          <button
            onClick={copyCode}
            className="flex items-center gap-1.5 px-2.5 py-1 rounded text-xs text-[#8b949e] hover:text-[#c9d1d9] hover:bg-[#21262d] transition-colors"
            title="Copy code"
          >
            {copied ? (
              <Check size={12} className="text-success" />
            ) : (
              <Copy size={12} />
            )}
            <span className="hidden sm:inline">{copied ? "Copied" : "Copy"}</span>
          </button>
          <button
            onClick={resetCode}
            className="flex items-center gap-1.5 px-2.5 py-1 rounded text-xs text-[#8b949e] hover:text-[#c9d1d9] hover:bg-[#21262d] transition-colors"
            title="Reset to starter code"
          >
            <RotateCcw size={12} />
            <span className="hidden sm:inline">Reset</span>
          </button>
          <button
            onClick={runCode}
            disabled={false}
            className={cn(
              "flex items-center gap-1.5 px-3 py-1 rounded text-xs font-medium transition-colors",
              running
                ? "bg-red-600/20 text-red-400 hover:bg-red-600/30"
                : "bg-accent text-white hover:bg-accent/90"
            )}
          >
            {running ? (
              <>
                <Loader2 size={12} className="animate-spin" />
                <span>Stop</span>
              </>
            ) : (
              <>
                <Play size={12} />
                <span>Run</span>
              </>
            )}
          </button>
        </div>
      </div>

      {/* Editor */}
      <div className="flex-1 overflow-auto min-h-0">
        <CodeMirror
          value={code}
          onChange={onCodeChange}
          extensions={[rust()]}
          theme={oneDark}
          height="100%"
          style={{ height: "100%" }}
          basicSetup={{
            lineNumbers: true,
            highlightActiveLineGutter: true,
            highlightSpecialChars: true,
            foldGutter: true,
            drawSelection: true,
            dropCursor: true,
            allowMultipleSelections: true,
            indentOnInput: true,
            bracketMatching: true,
            closeBrackets: true,
            autocompletion: true,
            rectangularSelection: true,
            crosshairCursor: true,
            highlightActiveLine: true,
            highlightSelectionMatches: true,
            closeBracketsKeymap: true,
            searchKeymap: true,
          }}
        />
      </div>

      {/* Output Panel */}
      <div
        className={cn(
          "border-t border-[#21262d] bg-[#0d1117] shrink-0 transition-all duration-200",
          outputOpen ? "max-h-56" : "max-h-8"
        )}
      >
        {/* Output header */}
        <div
          className="flex items-center gap-2 px-3 py-1.5 cursor-pointer hover:bg-[#161b22] transition-colors select-none"
          onClick={() => setOutputOpen(!outputOpen)}
        >
          <Terminal size={12} className="text-[#484f58]" />
          <span className="text-xs text-[#8b949e] font-mono flex-1">Output</span>
          {result && (
            <span
              className={cn(
                "text-xs font-mono px-1.5 py-0.5 rounded",
                result.success
                  ? "text-success bg-success/10"
                  : "text-danger bg-danger/10"
              )}
            >
              {result.success ? "OK" : "Error"}
            </span>
          )}
          {running && (
            <span className="text-xs text-warning font-mono flex items-center gap-1">
              <Loader2 size={10} className="animate-spin" />
              Running...
            </span>
          )}
          {outputOpen ? (
            <ChevronDown size={12} className="text-[#484f58]" />
          ) : (
            <ChevronUp size={12} className="text-[#484f58]" />
          )}
        </div>

        {/* Output content */}
        {outputOpen && (
          <div className="overflow-auto max-h-44 px-3 pb-3">
            {!result && !running && (
              <p className="text-xs text-[#484f58] font-mono py-2">
                Press Run to execute your code via the Rust Playground API.
              </p>
            )}
            {running && (
              <div className="flex items-center gap-2 py-2">
                <Loader2 size={12} className="animate-spin text-accent" />
                <span className="text-xs text-[#8b949e] font-mono">
                  Compiling and running...
                </span>
              </div>
            )}
            {hasOutput && (
              <div className="space-y-2 pt-1">
                {result.stdout && (
                  <pre className="text-xs text-[#c9d1d9] font-mono whitespace-pre-wrap leading-relaxed">
                    {result.stdout}
                  </pre>
                )}
                {result.stderr && (
                  <div className="flex items-start gap-2">
                    <AlertCircle
                      size={12}
                      className={cn(
                        "mt-0.5 shrink-0",
                        result.success ? "text-[#8b949e]" : "text-danger"
                      )}
                    />
                    <pre
                      className={cn(
                        "text-xs font-mono whitespace-pre-wrap leading-relaxed",
                        result.success ? "text-[#8b949e]" : "text-danger/90"
                      )}
                    >
                      {result.stderr}
                    </pre>
                  </div>
                )}
              </div>
            )}
          </div>
        )}
      </div>
    </div>
  );
}
