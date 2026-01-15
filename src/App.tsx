import "./App.css";
import LeftPanel from "./components/LeftPanel";
import RightPanel from "./components/RightPanel";


export const App = () => {
  return (
    <main className="flex flex-row min-h-screen flex-col items-center justify-center bg-black">
      <LeftPanel />
      <RightPanel />
    </main>
  );
}

export default App;
