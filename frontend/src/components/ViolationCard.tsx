import React from 'react';

/**
 * Interface describing the shape of a single analysis result.
 */
export interface Violation {
  status: 'OK' | 'NOT FOLLOWED' | 'ERROR';
  rule_no: number;
  rule_name: string;
  line?: number;
  reason?: string;
  suggestion?: string;
}

export interface AnalysisResult extends Violation {}

/**
 * ViolationCard is a presentational component that displays details about
 * a violated rule in a styled card. It highlights the rule number and
 * name, and lists the line, reason and suggestion if provided.
 */
interface ViolationCardProps {
  violation: Violation;
  code?: string;
}

const ViolationCard: React.FC<ViolationCardProps> = (props: ViolationCardProps) => {
  const { violation, code } = props;
  const { rule_no, rule_name, line, reason, suggestion, status } = violation;

  // Choose border and title colours based on status
  const border =
    status === 'ERROR' ? 'border-yellow-500' : status === 'NOT FOLLOWED' ? 'border-red-500' : 'border-gray-600';
  const titleColor =
    status === 'ERROR' ? 'text-yellow-400' : status === 'NOT FOLLOWED' ? 'text-red-400' : 'text-gray-200';
  return (
    <div className={`rounded-lg p-4 bg-gray-800 shadow-md border ${border}`}>
      <div className="flex items-start justify-between">
        <h3 className={`text-lg font-semibold ${titleColor}`}>
          {rule_no}. {rule_name}
        </h3>
        {typeof line === 'number' && line > 0 && (
          <span className="ml-4 inline-flex items-center text-xs px-2 py-1 rounded bg-gray-900 border border-gray-700">
            Line {line}
          </span>
        )}
      </div>
      {code && (
        <pre className="mt-2 text-xs bg-gray-900 border border-gray-700 rounded p-2 overflow-x-auto">
{code}
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