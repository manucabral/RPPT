import React from 'react';

interface ButtonProps {
    children: React.ReactNode;
    type?: 'button' | 'submit' | 'reset';
    onClick?: () => void;
    variant?: 'primary' | 'secondary' | 'outline';
    disabled?: boolean;
    className?: string;
}

export default function Button({
    children,
    onClick,
    type = 'button',
    variant = 'primary',
    disabled = false,
    className = '',
}: ButtonProps) {
    const baseStyles = 'px-6 py-2.5 rounded-lg font-medium transition-all duration-200 focus:outline-none focus:ring-2 focus:ring-offset-2 disabled:opacity-50 disabled:cursor-not-allowed transform hover:scale-105 active:scale-95 shadow-md hover:shadow-lg active:shadow-sm';
    
    const variants = {
        primary: 'bg-green-600 text-white hover:bg-green-700 active:bg-green-800 focus:ring-green-500',
        secondary: 'bg-black text-white hover:bg-gray-800 active:bg-gray-900 focus:ring-gray-500',
        outline: 'bg-transparent text-green-600 border-2 border-green-600 hover:bg-green-50 active:bg-green-100 focus:ring-green-500',
    };

    return (
        <button
            type={type}
            onClick={onClick}
            disabled={disabled}
            className={`${baseStyles} ${variants[variant]} ${className}`}
        >
            {children}
        </button>
    );
};
