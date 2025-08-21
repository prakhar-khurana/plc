import React from 'react';

// This is the inner violation object
export interface ViolationDetails {
  rule_no: number;
  rule_name: string;
  line: number;
  reason: string;
  suggestion: string;
}

// This is the top-level result object from Wasm
export interface AnalysisResult {
  status: 'OK' | 'NOT FOLLOWED';
  rule_no: number;
  rule_name: string;
  violation?: ViolationDetails; // Violation details are nested and optional
}

// Update the props for the card
const ViolationCard: React.FC<{ violation: AnalysisResult; code?: string }> = ({ violation, code }) => {
  // THE FIX: Destructure from the nested 'violation' object
  const { rule_no, rule_name } = violation;
  const { line, reason, suggestion } = violation.violation || {};

  return (
    <div className="border border-red-500 rounded-lg p-4 bg-gray-800 shadow-md">
      <div className="flex items-start justify-between">
        <h3 className="text-lg font-semibold text-red-400 mb-2">
          {rule_no}. {rule_name}
        </h3>
        {typeof line === 'number' && (
          <span className="ml-4 inline-flex items-center text-xs px-2 py-1 rounded bg-gray-900 border border-gray-700">
            Line {line}
          </span>
        )}
      </div>

      {code && (
        <pre className="mt-2 text-sm bg-gray-900 border border-gray-700 rounded p-3 overflow-x-auto font-mono">
          <code>{code.trim()}</code>
        </pre>
      )}

      {reason && (
        <p className="text-sm mt-2">
          <span className="font-medium">Reason:</span> {reason}
        </p>
      )}
      {suggestion && (
        <p className="text-sm mt-1">
          <span className="font-medium">Suggestion:</span> {suggestion}
        </p>
      )}
    </div>
  );
};

export default ViolationCard;