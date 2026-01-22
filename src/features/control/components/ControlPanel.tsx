import { Link } from "react-router";

export default function ControlPanel() {
    return (
        <div className="flex flex-col gap-2 text-white justify-start p-5 h-full">
            <h2 className="text-[#2FA572] font-medium text-sm uppercase tracking-wider mb-4">
                Control Panel
            </h2>
            <Link to="/" className="bg-[#1a1a1a] hover:bg-[#2FA572] text-white font-medium py-2 px-4 rounded-lg transition-colors hover:cursor-pointer">
                Home
            </Link>
            <Link to="/browser" className="bg-[#1a1a1a] hover:bg-[#2FA572] text-white font-medium py-2 px-4 rounded-lg transition-colors hover:cursor-pointer">
                Browser
            </Link>
        </div>
    );
}