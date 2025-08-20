import React from 'react';

interface PolicyInputProps {
  /**
   * The current value of the policy JSON string.
   */
  policy: string;
  /**
   * Called whenever the policy textarea changes.
   */
  onPolicyChange: (value: string) => void;
}

/**
 * PolicyInput component exposes a textarea for optional JSON policy input.
 */
const PolicyInput: React.FC<PolicyInputProps> = ({ policy, onPolicyChange }) => {
  return (
    <div className="mb-4">
      <label className="block mb-1 font-medium">Policy JSON (optional)</label>
      <textarea
        className="w-full h-32 p-2 bg-gray-800 border border-gray-600 rounded-lg text-gray-100 placeholder-gray-500 focus:outline-none focus:ring focus:border-blue-500"
        placeholder='[{"rule_no":4,"rule_name":"Use PLC flags as integrity checks","severity":"error"}]'
        value={policy}
        onChange={(e) => onPolicyChange(e.target.value)}
      />
    </div>
  );
};

export default PolicyInput;