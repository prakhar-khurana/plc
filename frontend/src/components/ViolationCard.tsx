import React from 'react';

/**
 * Interface describing the shape of a single analysis result.
 */
export interface Violation {
  status: string;
  rule_no: number;
  rule_name: string;
  line?: number;
  reason?: string;
  suggestion?: string;
}

/**
 * ViolationCard is a presentational component that displays details about
 * a violated rule in a styled card. It highlights the rule number and
 * name, and lists the line, reason and suggestion if provided.
 */
const ViolationCard: React.FC<{ violation: Violation }> = ({ violation }) => {
  const { rule_no, rule_name, line, reason, suggestion } = violation;
  return (
    <div className="border border-red-500 rounded-lg p-4 bg-gray-800 shadow-md">
      <h3 className="text-lg font-semibold text-red-400 mb-2">
        {rule_no}. {rule_name}
      </h3>
      {line !== undefined && (
        <p className="text-sm">
          <span className="font-medium">Line:</span> {line}
        </p>
      )}
      {reason && (
        <p className="text-sm mt-1">
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