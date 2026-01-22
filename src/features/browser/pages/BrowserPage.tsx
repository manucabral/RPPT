import { CdpBrowserInfo } from "../types/browser";
import { useBrowser } from "../hooks/useBrowser";
import BrowserProfile from "../components/BrowserProfile";
import BrowserList from "../components/BrowserList";
import BrowserConnected from "../components/BrowserConnected";


export default function BrowserPage() {
  const { installedBrowsers, currentBrowser, launchBrowserByName } = useBrowser();

  const handleLaunchBrowser = async (browser: CdpBrowserInfo) => {
    try {
      await launchBrowserByName(browser.name);
    } catch (error) {
      console.error("Error launching browser:", error);
    }
   }
  const handleOpenLocation = async (browser: CdpBrowserInfo) => { }
  const handleForceClose = async () => { }
  const handleRefresh = async () => { }
  const handleRefreshCurrentBrowser = async () => { }


  return (
    <div className="bg-[#0A0A0A] flex flex-col font-sans overflow-hidden">
      <BrowserConnected
        currentBrowser={currentBrowser}
        onRefreshCurrentBrowser={handleRefreshCurrentBrowser}
        onForceClose={handleForceClose} />
      <BrowserList
        browsers={installedBrowsers}
        onLaunch={handleLaunchBrowser}
        onOpenLocation={handleOpenLocation}
        onForceClose={handleForceClose}
        onRefreshCurrentBrowser={handleRefreshCurrentBrowser}
        onRefresh={handleRefresh}
      />
      <BrowserProfile />
    </div>
  );
}




