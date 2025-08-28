import React from 'react';

interface InfoPageProps {
  /** Whether the info page is open */
  open: boolean;
  /** Callback to close the modal */
  onClose: () => void;
}

// Array of rule descriptions used in the About section. Each entry lists
// the rule number, name and a short summary of what it checks for.
const RULES: Array<{ no: number; name: string; text: string }> = [
  { no: 1,  name: 'Modularize PLC Code', text: 'Use FC/FB/OB separation; avoid monolithic logic.' },
  { no: 2,  name: 'Track operating modes', text: 'Gate risky actions on RUN/STOP/STARTUP states.' },
  { no: 3,  name: 'Validate and alert for paired I/O', text: 'Never drive conflicting outputs simultaneously.' },
  { no: 4,  name: 'Use PLC flags as integrity checks', text: 'Guard divisions with SW.OV/SW.OS and divisor<>0.' },
  { no: 5,  name: 'Use checksum integrity checks', text: 'Compute and verify checksums where data integrity matters.' },
  { no: 6,  name: 'Validate timers and counters', text: 'Range-check externally set presets and parameters.' },
  { no: 7,  name: 'Validate paired inputs/outputs', text: 'Mutual exclusion on forward/reverse or open/close pairs.' },
  { no: 8,  name: 'Validate HMI input variables', text: 'Sanitize HMI/DB values with plausibility checks.' },
  { no: 9,  name: 'Validate indirections', text: 'Bounds-check array and indirect memory access.' },
  { no: 10, name: 'Assign designated register blocks', text: 'Writes only to policy-allowed memory regions.' },
  { no: 11, name: 'Plausibility Checks', text: 'Document plausibility with inline comments before use.' },
  { no: 12, name: 'Document critical assumptions', text: 'Attach context/assumptions before critical operations.' },
  { no: 13, name: 'Alarm escalation path', text: 'Escalate persistent alarms instead of silencing.' },
  { no: 14, name: 'Fail-safe defaults', text: 'Default outputs to safe states during uncertainty.' },
  { no: 15, name: 'Define a safe restart state', text: 'Use OB100 to reset to a secure, deterministic baseline.' },
  { no: 16, name: 'Summarize PLC cycle times', text: 'Record scan/cycle time to detect overloads/regressions.' },
  { no: 17, name: 'Log PLC uptime', text: 'Record uptime for diagnostics and duty-cycle analysis.' },
  { no: 18, name: 'Log PLC hard stops', text: 'Use OB121/OB82/OB86 to capture faults and program errors.' },
  { no: 19, name: 'Monitor PLC memory usage', text: 'Track memory usage trends to avoid overflows.' },
  { no: 20, name: 'Trap false alerts', text: 'Debounce/validate alarms to reduce noise and flapping.' },
];

/**
 * InfoPage displays a modal overlay with a comprehensive explanation of
 * the tool, including supported languages, the list of security checks
 * performed, and examples for setting custom policies. It appears when
 * the user clicks the About button in the header and disappears when
 * the user clicks Close.
 */
const InfoPage: React.FC<InfoPageProps> = ({ open, onClose }) => {
  if (!open) return null;
  return (
    <div className="fixed inset-0 z-50 flex items-start justify-center bg-black/70">
      <div className="mt-10 w-full max-w-3xl rounded-xl border border-gray-700 bg-gray-900 p-6 shadow-2xl overflow-y-auto max-h-[90vh]">
        <div className="flex items-center justify-between">
          <h2 className="text-2xl font-bold">About – PLC Secure Coding Practices Checker</h2>
          <button
            className="rounded bg-gray-800 px-3 py-1 text-sm hover:bg-gray-700"
            onClick={onClose}
          >
            Close
          </button>
        </div>

        <section className="mt-4 space-y-2">
          <h3 className="text-lg font-semibold">What is this tool?</h3>
          <p className="text-gray-300">
            This tool analyzes Siemens PLC source (SCL, IL, PLCOpen XML) for secure coding practices.
            It runs entirely in your browser via WebAssembly — your code never leaves your machine.
          </p>
        </section>

        <section className="mt-6">
          <h3 className="text-lg font-semibold mb-2">Security Checks Performed</h3>
          <ul className="grid grid-cols-1 gap-2 md:grid-cols-2">
            {RULES.map((r) => (
              <li key={r.no} className="rounded border border-gray-700 bg-gray-800 p-3">
                <div className="font-medium">{r.no}. {r.name}</div>
                <div className="text-sm text-gray-400">{r.text}</div>
              </li>
            ))}
          </ul>
        </section>

        <section className="mt-6">
          <h3 className="text-lg font-semibold mb-2">Supported Languages</h3>
          <ul className="list-disc list-inside text-gray-300">
            <li>Structured Text (SCL / ST)</li>
            <li>PLCOpen XML</li>
            <li>Instruction List (IL / AWL)</li>
          </ul>
        </section>

        <section className="mt-6">
          <h3 className="text-lg font-semibold mb-2">How to Set a Custom Policy</h3>
          <p className="text-gray-300">
            Paste JSON in the Policy box to configure paired I/O and memory permissions. Example:
          </p>
          <pre className="mt-2 max-h-48 overflow-auto rounded bg-gray-950 p-3 text-xs text-gray-300">
{`{
  "pairs": [["Motor_Fwd","Motor_Rev"], ["Valve_Open","Valve_Close"]],
  "memory_areas": [
    { "address": "%MW100-%MW200", "access": "ReadOnly" },
    { "address": "%M50-%M80",     "access": "ReadWrite" }
  ]
}`}
          </pre>
        </section>
      </div>
    </div>
  );
};

export default InfoPage;