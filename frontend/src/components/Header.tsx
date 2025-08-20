import React from 'react';

/**
 * Header component displaying the application title and a short description.
 */
const Header: React.FC = () => (
  <header className="mb-6">
    <h1 className="text-3xl font-bold mb-2">PLC Secure Coding Practices Checker</h1>
    <p className="text-gray-400">
      Upload your PLC source code to ensure it adheres to secure coding guidelines.
    </p>
  </header>
);

export default Header;