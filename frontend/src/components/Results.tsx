import React from 'react';
import ViolationCard, { Violation } from './ViolationCard';

// Extend Violation for analysis results; status may be "OK", "NOT FOLLOWED" or "ERROR"
export interface AnalysisResult extends Violation {}

interface ResultsProps {
  results: AnalysisResult[];
  /**
   * Source code string used to map line numbers back to exact code lines.
   */
  source?: string;
}

/**
 * Results separates followed rules from violations and displays them using
 * lists and violation cards. It also surfaces any error results (e.g.
 * policy parsing errors) as a separate section. If a source string is
 * provided, the offending line of code is displayed on each card.
 */
const Results: React.FC<ResultsProps> = ({ results, source }) => {
  if (!results || results.length === 0) return null;
  // Pre-split source into lines; note that line numbers are 1-based
  const lines = (source ?? '').split(/\r?\n/);
  const codeFor = (n?: number) => {
    return n && n > 0 && n <= lines.length ? lines[n - 1] : undefined;
  };

  const errors = results.filter((r) => r.status === 'ERROR');
  const followed = results.filter((r) => r.status === 'OK');
  const notFollowed = results.filter((r) => r.status === 'NOT FOLLOWED');
  // Sort violations by line number then rule number for consistency
  const sortedViolations = [...notFollowed].sort((a, b) => {
    const la = a.line ?? Number.MAX_SAFE_INTEGER;
    const lb = b.line ?? Number.MAX_SAFE_INTEGER;
    if (la !== lb) return la - lb;
    return (a.rule_no ?? 0) - (b.rule_no ?? 0);
  });
  return (
    <div className="mt-4">
      {errors.length > 0 && (
        <div className="mb-4 rounded border border-yellow-600 bg-yellow-900/30 p-3">
          <div className="font-semibold">Policy Parsing Errors</div>
          <ul className="mt-1 list-disc list-inside text-sm">
            {errors.map((e, idx) => (
              <li key={idx}>{e.reason ?? 'Invalid policy JSON'}{e.line ? ` (line ${e.line})` : ''}</li>
            ))}
          </ul>
        </div>
      )}
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
                key={`${r.rule_no}-${r.line ?? 'noline'}-${idx}`}
                violation={r}
                code={codeFor(r.line)}
              />
            ))}
          </div>
        </div>
      )}
    </div>
  );
};

export default Results;