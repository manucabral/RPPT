
import type { Presence } from "../types/presences";

interface PresenceItemProps {
  presence: Presence;
  onToggle: (id: string) => void;
}

export default function PresenceItem({ presence, onToggle }: PresenceItemProps) {
  return (
    <div
      className={`flex items-center justify-between p-3 rounded-xl transition-all duration-200 ${
        presence.enabled
          ? "bg-[#2FA572]/10 border border-[#2FA572]/20"
          : "bg-[#1a1a1a] border border-transparent hover:border-[#2a2a2a]"
      }`}
    >
      <div className="flex items-center gap-3">
        <span className="text-lg">{presence.icon}</span>
        <span
          className={`text-sm font-medium ${
            presence.enabled ? "text-white" : "text-gray-400"
          }`}
        >
          {presence.name}
        </span>
      </div>
      <button
        onClick={() => onToggle(presence.id)}
        className={`relative w-11 h-6 rounded-full transition-all duration-300 ${
          presence.enabled ? "bg-[#2FA572]" : "bg-[#333]"
        }`}
        aria-label={`Toggle ${presence.name}`}
      >
        <span
          className={`absolute top-1 w-4 h-4 bg-white rounded-full shadow-md transition-all duration-300 ${
            presence.enabled ? "left-6" : "left-1"
          }`}
        />
      </button>
    </div>
  );
}
