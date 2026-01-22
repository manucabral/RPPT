import { CdpBrowserInfo } from "../types/browser";
import BrowserItem from "./BrowserItem";

interface BrowserListProps {
    browsers: CdpBrowserInfo[];
    currentBrowser?: CdpBrowserInfo | null;
    onLaunch: (browser: CdpBrowserInfo) => void;
    onOpenLocation: (browser: CdpBrowserInfo) => void;
    onForceClose: () => void;
    onRefreshCurrentBrowser: () => void;
    onRefresh: () => void;
}

export default function BrowserList({
    browsers,
    currentBrowser,
    onLaunch,
    onOpenLocation,
    onForceClose,
    onRefreshCurrentBrowser,
    onRefresh,
}: BrowserListProps) {
    return (
        <div className="flex flex-col p-5 h-full w-full">
            <div className="flex items-center justify-between mb-4">
                <h2 className="text-[#2FA572] font-medium text-sm uppercase tracking-wider">
                    Installed Browsers
                </h2>
            </div>
            <div className="flex flex-col gap-3 overflow-y-auto h-full">
                {browsers.length === 0 ? (
                    <p className="text-gray-500 text-sm">No browsers detected</p>
                ) : (
                    browsers.map((browser, index) => (
                        <BrowserItem
                            key={index}
                            browser={browser}
                            onLaunch={onLaunch}
                            onOpenLocation={onOpenLocation}
                        />
                    ))
                )}
            </div>
        </div>
    );
}