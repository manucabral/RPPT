import { invoke } from '@tauri-apps/api/core';
import { useState } from 'react';
import Button from './Button';


export default function RightPanel() {
    return (
        <div className="flex flex-col gap-4 w-1/2 h-screen bg-gray-600 p-4">
            <h2 className="text-2xl font-bold mb-4">Configuration</h2>
            <Button>Browser</Button>
            <Button>Catalog</Button>
            <Button>Settings</Button>
        </div>
    );
}