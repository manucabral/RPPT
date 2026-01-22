import { invoke } from "@tauri-apps/api/core";
import type { CdpBrowserInfo } from "../types/browser";
import { useEffect, useState } from "react";


export function useBrowser() {
    const [isLoading, setIsLoading] = useState<boolean>(true);
    const [currentBrowser, setCurrentBrowser] = useState<CdpBrowserInfo | null>(null);
    const [installedBrowsers, setInstalledBrowsers] = useState<CdpBrowserInfo[]>([]);

    useEffect(() => {
        const fetchBrowser = async () => {
            const browser = await getCurrentBrowser();
            setCurrentBrowser(browser || null);
        };
        const fetchInstalledBrowsers = async () => {
            const browsers = await getInstalledBrowsers();
            setInstalledBrowsers(browsers || []);
        };
        fetchInstalledBrowsers();
        fetchBrowser();
    }, []);

    const refreshBrowser = async () => {
        try {
            setIsLoading(true);
            await invoke("refresh_current_cdp_browser");
        } catch (error) {
            console.error("Error refreshing browser:", error);
        } finally {
            setIsLoading(false);
        }
    }

    const getCurrentBrowser = async () => {
        try {
            await refreshBrowser();
            const browser = await invoke<CdpBrowserInfo | undefined>("get_current_cdp_browser");
            console.log("Current browser:", browser);
            return browser;
        } catch (error) {
            console.error("Error fetching current browser:", error);
            return undefined;
        } finally {
            setIsLoading(false);
        }
    }

    const refreshInstalledBrowsers = async () => {
        try {
            console.log("Refreshing installed browsers...");
            await invoke("refresh_installed_browsers");
        } catch (error) {
            console.error("Error refreshing browsers:", error);
        }
    };

    const getInstalledBrowsers = async () => {
        try {
            await refreshInstalledBrowsers();
            const result = await invoke<CdpBrowserInfo[]>("get_installed_browsers");
            console.log("Installed browsers:", result);
            return result;
        } catch (error) {
            console.error("Error fetching browsers:", error);
        }
    };

    const launchBrowserByName = async (browserName: string) => {
        try {
            await invoke("launch_browser_by_name", { 
                browserName: browserName,
                profileName: "Test",
                dryRun: false,
                remote_debugging_port: 4969,
                remote_debugging_address: "*"
            });
            console.log(`Launched browser: ${browserName}`);
        } catch (error) {
            console.error(`Error launching browser ${browserName}:`, error);
        }
    };

    return {
        currentBrowser,
        getCurrentBrowser,
        refreshBrowser,
        isLoading,
        installedBrowsers,
        getInstalledBrowsers,
        refreshInstalledBrowsers,
        launchBrowserByName,
    };
}