import React from 'react';
import ViolationCard, { Violation } from './ViolationCard';

/**
 * Extends the Violation interface to represent the shape of analysis results.
 */
export interface AnalysisResult extends Violation {}

interface ResultsProps {
  results: AnalysisResult[];
}

/**
 * Results component separates followed rules from violations and displays them
 * using simple lists and reusable ViolationCard components.
 */
const Results: React.FC<ResultsProps> = ({ results }) => {
  if (!results || results.length === 0) {
    return null;
  }
  const followed = results.filter((r) => r.status === 'OK');
  const notFollowed = results.filter((r) => r.status !== 'OK');
  return (
    <div className="mt-6">
      {followed.length > 0 && (
        <div className="mb-4">
          <h2 className="text-xl font-semibold mb-2">✅ Practices Followed</h2>
          <ul className="list-disc list-inside space-y-1">
            {followed.map((r) => (
              <li key={r.rule_no}>
                {r.rule_no}. {r.rule_name}
              </li>
            ))}
          </ul>
        </div>
      )}
      {notFollowed.length > 0 && (
        <div>
          <h2 className="text-xl font-semibold mb-2">❌ Practices Not Followed</h2>
          <div className="space-y-4">
            {notFollowed.map((r) => (
              <ViolationCard key={r.rule_no} violation={r} />
            ))}
          </div>
        </div>
      )}
    </div>
  );
};

export default Results;