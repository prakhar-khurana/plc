import React from 'react';
// THE FIX: Import the new types
import ViolationCard, { AnalysisResult } from './ViolationCard';

interface ResultsProps {
  results: AnalysisResult[];
  source: string;
}

const Results: React.FC<ResultsProps> = ({ results, source }) => {
  if (!results || results.length === 0) return null;

  const followed = results.filter((r) => r.status === 'OK');
  const notFollowed = results.filter((r) => r.status !== 'OK');

  const lines = source.split(/\r?\n/);
  const getLineText = (lineNumber?: number) => {
    if (lineNumber && lineNumber > 0 && lineNumber <= lines.length) {
      return lines[lineNumber - 1];
    }
    return undefined;
  };

  const sortedViolations = [...notFollowed].sort((a, b) => {
    // THE FIX: Access the nested line number for sorting
    const la = a.violation?.line ?? Number.MAX_SAFE_INTEGER;
    const lb = b.violation?.line ?? Number.MAX_SAFE_INTEGER;
    if (la !== lb) return la - lb;
    return a.rule_no - b.rule_no;
  });

  return (
    <div className="mt-6">
      {followed.length > 0 && (
        <div className="mb-4">
          <h2 className="text-xl font-semibold mb-2">✅ Practices Followed</h2>
          <ul className="list-disc list-inside space-y-1">
            {followed.map((r, idx) => (
              <li key={`${r.rule_no}-${idx}`}>
                {r.rule_no}. {r.rule_name}
              </li>
            ))}
          </ul>
        </div>
      )}
      {sortedViolations.length > 0 && (
        <div>
          <h2 className="text-xl font-semibold mb-2">❌ Practices Not Followed</h2>
          <div className="space-y-4">
            {sortedViolations.map((r, idx) => (
              <ViolationCard
              key={`${r.rule_no}-${r.violation?.line ?? 'noline'}-${idx}`}
              violation={r}
              // THE FIX: Access the nested line number to get the code
              code={getLineText(r.violation?.line)}
            />
            ))}
          </div>
        </div>
      )}
    </div>
  );
};

export default Results;
