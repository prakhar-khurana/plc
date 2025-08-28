import React from 'react';

interface HeaderProps {
  /** Called when the user clicks the About button */
  onAbout: () => void;
}

/**
 * Header component displaying the application title, subtitle and an About button.
 */
const Header: React.FC<HeaderProps> = ({ onAbout }) => (
  <header className="mb-6 flex items-center justify-between">
    <div>
      <h1 className="text-3xl font-bold mb-2">PLC Secure Coding Practices Checker</h1>
      <p className="text-gray-400">
        Upload your PLC source, set an optional policy, and analyze against secure practices.
      </p>
    </div>
    <button
      onClick={onAbout}
      className="rounded bg-gray-800 px-3 py-1 text-sm hover:bg-gray-700 border border-gray-700"
    >
      About
    </button>
  </header>
);

export default Header;