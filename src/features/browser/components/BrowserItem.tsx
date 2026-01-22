import { CdpBrowserInfo } from "../types/browser";

interface BrowserItemProps {
  browser: CdpBrowserInfo;
  onLaunch: (browser: CdpBrowserInfo) => void;
  onOpenLocation: (browser: CdpBrowserInfo) => void;
}

export default function BrowserItem({ browser, onLaunch, onOpenLocation }: BrowserItemProps) {
  return (
    <div className="flex items-center justify-between p-3 rounded-xl bg-[#1a1a1a] border border-transparent hover:border-[#2a2a2a] transition-all duration-200 group">
      <div className="flex items-center gap-3 min-w-0 flex-1">
        ICON
        <div className="min-w-0 flex-1">
          <span className="text-sm font-medium text-white block">{browser.name}</span>
          {browser.path && (
            <span className="text-xs text-gray-500 truncate block" title={browser.path}>
              {browser.path}
            </span>
          )}
        </div>
      </div>
      
      <div className="flex items-center gap-2 opacity-0 group-hover:opacity-100 transition-opacity duration-200">
        <button
          onClick={() => onLaunch(browser)}
          className="flex items-center gap-1.5 px-3 py-1.5 rounded-lg bg-[#2FA572] hover:bg-[#259660] text-white text-xs font-medium transition-colors"
          title="Launch browser"
        >
          Launch
        </button>
        
        <button
          onClick={() => onOpenLocation(browser)}
          disabled={!browser.path}
          className="flex items-center gap-1.5 px-3 py-1.5 rounded-lg bg-[#252525] hover:bg-[#333] text-gray-300 hover:text-white text-xs font-medium transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
          title="Open file location"
        >
          Location
        </button>
      </div>
    </div>
  );
}
