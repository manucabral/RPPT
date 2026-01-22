export default function BrowserProfile() {
    return (
        <div className="flex flex-col p-5 h-full w-full">
            <div className="flex items-center justify-between mb-4">
                <h2 className="text-[#2FA572] font-medium text-sm uppercase tracking-wider">
                    Browser Profile
                </h2>
            </div>
            <div className="flex flex-col gap-4">
                <p className="text-gray-500 text-sm">
                    When you launch a browser, a new profile will be created with isolated settings and data.
                </p>
                <input 
                    type="text"
                    placeholder="Profile Name"
                    className="w-full p-3 outline-none rounded-lg bg-[#1a1a1a] border border-transparent hover:border-[#2a2a2a] text-white placeholder-gray-500 transition-all duration-200"
                />
            </div>
        </div>
    );
}