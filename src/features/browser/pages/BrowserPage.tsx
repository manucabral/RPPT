import { CdpBrowserInfo } from "../types/browser";
import { useBrowser } from "../hooks/useBrowser";
import BrowserProfile from "../components/BrowserProfile";
import BrowserList from "../components/BrowserList";
import BrowserConnected from "../components/BrowserConnected";
import Spinner from "../../../shared/ui/components/Spinner";


export default function BrowserPage() {
  const { installedBrowsers, currentBrowser, launchBrowserByName, closeCurrentBrowser, isLoading } = useBrowser();

  const handleLaunchBrowser = async (browser: CdpBrowserInfo) => {
    try {
      console.info("Launching browser:", browser);
      await launchBrowserByName(browser.name);
    } catch (error) {
      console.error("Error launching browser:", error);
    }
  }
  const handleOpenLocation = async (browser: CdpBrowserInfo) => { }
  const handleForceClose = async () => {
    await closeCurrentBrowser();
  }
  const handleRefresh = async () => { }
  const handleRefreshCurrentBrowser = async () => { }

  if (isLoading) {
    return (
      <div className="flex items-center justify-center h-full">
        <Spinner />
      </div>
    );
  }

  return (
    <div className="bg-[#0A0A0A] flex flex-col overflow-hidden max-h-[80vh] overflow-y-auto custom-scrollbar">
      <BrowserConnected
        currentBrowser={currentBrowser}
        onRefreshCurrentBrowser={handleRefreshCurrentBrowser}
        onForceClose={handleForceClose} />
      <BrowserProfile />
      <BrowserList
        browsers={installedBrowsers}
        onLaunch={handleLaunchBrowser}
        onOpenLocation={handleOpenLocation}
        onForceClose={handleForceClose}
        onRefreshCurrentBrowser={handleRefreshCurrentBrowser}
        onRefresh={handleRefresh}
      />
    </div>
  );
}




