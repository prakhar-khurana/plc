import React, { useState } from 'react';
import Header from './components/Header';
import FileInput from './components/FileInput';
import PolicyInput from './components/PolicyInput';
import Results from './components/Results';
import type { AnalysisResult } from './components/Results';

const App: React.FC = () => {
  const [fileContent, setFileContent] = useState<string>('');
  const [fileName, setFileName] = useState<string>('');
  const [policy, setPolicy] = useState<string>('');
  const [results, setResults] = useState<AnalysisResult[]>([]);
  const [loading, setLoading] = useState<boolean>(false);
  const [error, setError] = useState<string>('');

  /**
   * Callback invoked when a file is successfully read by the FileInput component.
   * @param content The raw text content of the file
   * @param name The name of the file
   */
  const handleFileLoaded = (content: string, name: string) => {
    setFileContent(content);
    setFileName(name);
    // Clear previous results when new file is selected
    setResults([]);
  };

  /**
   * Callback invoked when the policy textarea content changes.
   * @param value The updated JSON policy string
   */
  const handlePolicyChange = (value: string) => {
    setPolicy(value);
  };

  /**
   * Trigger analysis of the uploaded PLC source code by invoking the Wasm module.
   */
  const handleAnalyze = async () => {
    setError('');
    if (!fileContent) {
      setError('Please upload a source code file before analyzing.');
      return;
    }
    setLoading(true);
    try {
      /**
       * Dynamically import the wasm-bindgen generated module. The @vite-ignore
       * comment instructs Vite not to attempt to bundle or pre-bundle this import,
       * allowing it to remain as a runtime dynamic import. You must ensure
       * that the compiled files (plc_checker.js and plc_checker_bg.wasm) are
       * available in the `wasm` directory at the project root as described in
       * the project README.
       */
      const module = await import(/* @vite-ignore */ '../wasm/plc_secure_checker.js');
      // If the module has a default export, call it to initialize the wasm module.
      if (typeof module.default === 'function') {
        await module.default();
      }
      // Retrieve the exported check_plc_code function from the wasm module.
      const checkPlcCode: (src: string, policy: string, fileName: string) => string =
        module.check_plc_code || module.checkPlcCode || module.check_plc_code;
      if (typeof checkPlcCode !== 'function') {
        throw new Error('WASM function check_plc_code was not found. Make sure the Rust code is compiled with wasm-bindgen and placed correctly.');
      }
      // Invoke the checker and parse the resulting JSON string.
      const rawResult = checkPlcCode(fileContent, policy || '', fileName);
      const parsed: AnalysisResult[] = JSON.parse(rawResult);
      console.log("Raw JSON from Wasm:", rawResult); 
  // --- PRINT HERE (2) ---
      console.log("Parsed JavaScript Object:", parsed);
      setResults(parsed);
    } catch (err: any) {
      console.error(err);
      setError(err?.message || 'An error occurred while analyzing the code.');
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="min-h-screen p-6 bg-gray-900 text-gray-100">
      <div className="max-w-4xl mx-auto">
        <Header />
        <FileInput onFileLoaded={handleFileLoaded} />
        <PolicyInput policy={policy} onPolicyChange={handlePolicyChange} />
        <button
          type="button"
          className="mt-2 px-4 py-2 bg-blue-600 hover:bg-blue-700 rounded disabled:bg-gray-600 transition-colors"
          onClick={handleAnalyze}
          disabled={loading || !fileContent}
        >
          {loading ? 'Analyzing…' : 'Analyze Code'}
        </button>
        {error && <p className="mt-2 text-red-400">{error}</p>}
        <Results results={results} source={fileContent} />
        </div>
    </div>
  );
};

export default App;