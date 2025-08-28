import React, { useMemo } from 'react';
import { PieChart, Pie, Cell, ResponsiveContainer, Legend, Tooltip } from 'recharts';
import { AnalysisResult } from './ViolationCard';

// Green for passed, red for failed
const COLORS = ['#16a34a', '#dc2626'];
// Total number of unique rules the checker tests for
const TOTAL_RULES = 20;

interface ResultsChartProps {
  results: AnalysisResult[];
}

const ResultsChart: React.FC<ResultsChartProps> = ({ results }) => {
  const chartData = useMemo(() => {
    if (!results || results.length === 0) {
      return { data: [], compliantPercent: 0 };
    }
    // Find all unique rule numbers that have at least one violation
    const failedRuleNumbers = new Set(
      results.filter((r) => r.status !== 'OK').map((r) => r.rule_no)
    );

    const failedCount = failedRuleNumbers.size;
    const passedCount = TOTAL_RULES - failedCount;

    const data = [
      { name: 'Passed', value: passedCount },
      { name: 'Failed', value: failedCount },
    ];
    
    const compliantPercent = TOTAL_RULES === 0 ? 0 : Math.round((passedCount / TOTAL_RULES) * 100);

    return { data, compliantPercent };
  }, [results]);
  
  const total = chartData.data.reduce((sum, entry) => sum + entry.value, 0);
  if (total === 0) return null;

  return (
    <div className="mb-6 rounded border border-gray-700 bg-gray-800 p-3">
      <div className="mb-2 text-sm text-gray-400">Overall Compliance</div>
      <div className="h-56">
        <ResponsiveContainer>
          <PieChart>
            <Pie
              data={chartData.data}
              dataKey="value"
              nameKey="name"
              innerRadius={60}
              outerRadius={80}
              paddingAngle={2}
            >
              {chartData.data.map((_, i) => (
                <Cell key={i} fill={COLORS[i % COLORS.length]} />
              ))}
            </Pie>
            <Tooltip
              contentStyle={{ background: '#0b1220', border: '1px solid #374151', color: '#e5e7eb' }}
            />
            <Legend formatter={(value, entry) => `${value}: ${entry.payload.value}`} />
          </PieChart>
        </ResponsiveContainer>
      </div>
      <div className="mt-2 text-center text-xl font-semibold">{chartData.compliantPercent}% Compliant</div>
    </div>
  );
};

export default ResultsChart;