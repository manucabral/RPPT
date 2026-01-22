import type { CdpBrowserInfo } from "../types/browser";

interface BrowserConnectedProps {
    currentBrowser?: CdpBrowserInfo | null;
    onRefreshCurrentBrowser: () => void;
    onForceClose: () => void;
}

export default function BrowserConnected({
    currentBrowser,
    onRefreshCurrentBrowser,
    onForceClose,
}: BrowserConnectedProps) {
    return (
        <div className="flex items-center justify-between p-5 border-b border-gray-800">
            <div className="flex-col justify-between w-full">
                <h2 className="text-[#2FA572] font-medium text-sm uppercase tracking-wider">
                    Browser Status
                </h2>
                <div className="flex items-center justify-start gap-4 mt-2 w-full ">
                    {currentBrowser ? (
                        <h2 className="text-green-500 text-xs font-medium">
                            Connected: {currentBrowser.name}
                        </h2>
                    ) : (
                        <div className="flex items-center justify-center gap-4">
                            <h2 className="text-red-500 font-medium">
                                No browser connected
                            </h2>
                            <button
                                onClick={onRefreshCurrentBrowser}
                                className="bg-[#1a1a1a] hover:bg-[#2FA572] text-white font-medium py-2 px-4 rounded-lg transition-colors cursor-pointer text-xs"
                            >
                                Refresh browser status
                            </button>
                        </div>
                    )}
                </div>
            </div>
        </div>
    );
}