import type { CdpBrowserInfo } from "../types/browser";

export default function StatusBar({ browser, loading }: { browser: CdpBrowserInfo | null, loading: boolean }) {
  const isConnected = browser !== null;

  return (
    <div className="flex items-center justify-between px-5 py-3 bg-[#0d0d0d] border-t border-[#1a1a1a]">
      <div className="flex items-center gap-2">
        <span
          className={`w-2 h-2 rounded-full ${
            isConnected ? "bg-[#2FA572] animate-pulse" : "bg-red-500"
          }`}
        />
        {loading ? (
          <span className="text-gray-400 text-sm">Loading...</span>
        ) : (
        <span className="text-gray-400 text-sm">
          {isConnected ? (
            <>
              <span className="text-white font-medium">{browser?.name || "Unknown"}</span>{" "}
              <span className="text-gray-400">{browser?.version || ""}</span>{" "}  
            </>
          ) : (
            "Not browser connected. Please check control panel."
          )}
        </span>
        )}
      </div>
    </div>
  );
}
