import { Outlet } from "react-router";
import Header from "./shared/ui/Header";
import ControlPanel from "./features/control/components/ControlPanel";
import StatusBar from "./features/browser/components/StatusBar";
import { useBrowser } from "./features/browser/hooks/useBrowser";


export default function Layout() {
  const { currentBrowser, isLoading } = useBrowser();

  return (
    <div className="min-h-screen flex flex-col font-sans overflow-hidden bg-[#0A0A0A] w-full">
      <Header />

      <main className="md:flex justify-center w-full h-full flex-1 overflow-hidden mx-auto">
        <div className="md:w-1/4 border-r border-[#1a1a1a] justify-center">
          <ControlPanel />
        </div>
        <div className="w-full md:w-3/4">
          <Outlet />
        </div>
      </main>
      
      <StatusBar browser={currentBrowser || null} loading={isLoading} />
    </div>
  );
}

