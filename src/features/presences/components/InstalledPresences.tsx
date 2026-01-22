import { Presence } from "../types/presences";
import PresenceItem from "./PresenceItem"



const initialPresences: Presence[] = [
  { id: "1", name: "Phasmophobia", enabled: true, icon: "G" },
  { id: "2", name: "YouTube", enabled: false, icon: "Y" },
  { id: "3", name: "YouTube Music", enabled: true, icon: "M" },
  { id: "4", name: "Spotify", enabled: false, icon: "S" },
  { id: "5", name: "Visual Studio Code", enabled: true, icon: "V" },

];


export default function InstalledPresences() {
  return (
    <div className="flex-1 p-5 flex flex-col h-full">
      <h2 className="text-[#2FA572] font-medium text-sm uppercase tracking-wider mb-4">
        Installed Presences
      </h2>
      <div className="flex-1 min-h-0 space-y-2 overflow-y-auto scrollbar-thin pr-2 max-h-[60vh] custom-scrollbar">
        {initialPresences.map((presence) => (
          <PresenceItem
            key={presence.id}
            presence={presence}
            onToggle={(id) => {
              // Handle toggle logic here
              console.log(`Toggled presence with id: ${id}`);
            }}
          />
        ))}
      </div>
    </div>
  );
}
