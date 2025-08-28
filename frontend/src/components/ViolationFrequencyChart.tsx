import React, { useMemo } from 'react';
import {
  BarChart,
  Bar,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  Legend,
  ResponsiveContainer,
} from 'recharts';
import { AnalysisResult } from './ViolationCard';

interface ViolationFrequencyChartProps {
  results: AnalysisResult[];
}

const ViolationFrequencyChart: React.FC<ViolationFrequencyChartProps> = ({ results }) => {
  const frequencyData = useMemo(() => {
    const violations = results.filter((r) => r.status === 'NOT FOLLOWED');
    if (violations.length === 0) return [];

    const frequencyMap = new Map<number, { name: string; count: number }>();

    for (const v of violations) {
      if (frequencyMap.has(v.rule_no)) {
        frequencyMap.get(v.rule_no)!.count++;
      } else {
        frequencyMap.set(v.rule_no, { name: `Rule ${v.rule_no}`, count: 1 });
      }
    }
    return Array.from(frequencyMap.values()).sort((a, b) => b.count - a.count);
  }, [results]);

  if (frequencyData.length === 0) return null;

  return (
    <div className="my-6 rounded border border-gray-700 bg-gray-800 p-4">
      <div className="mb-4 text-sm text-gray-400">Violation Frequency</div>
      <div className="h-72">
        <ResponsiveContainer>
          <BarChart data={frequencyData} margin={{ top: 5, right: 20, left: -10, bottom: 5 }}>
            <CartesianGrid strokeDasharray="3 3" stroke="#374151" />
            <XAxis dataKey="name" stroke="#9ca3af" />
            <YAxis allowDecimals={false} stroke="#9ca3af" />
            <Tooltip
              contentStyle={{ background: '#0b1220', border: '1px solid #374151' }}
              cursor={{ fill: 'rgba(156, 163, 175, 0.1)' }}
            />
            <Legend />
            <Bar dataKey="count" name="Violations" fill="#dc2626" />
          </BarChart>
        </ResponsiveContainer>
      </div>
    </div>
  );
};

export default ViolationFrequencyChart;