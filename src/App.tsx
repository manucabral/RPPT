
import './App.css';
import InstalledPresences from "./features/presences/components/InstalledPresences";

export default function App() {
    return (
        <div className="flex flex-col text-white justify-center">
            <InstalledPresences />
        </div>
    );
}