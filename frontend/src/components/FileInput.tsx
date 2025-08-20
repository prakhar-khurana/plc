import React, { useRef, useState } from 'react';

interface FileInputProps {
  /**
   * Called with the file's text content and its filename once reading succeeds.
   */
  onFileLoaded: (content: string, fileName: string) => void;
}

/**
 * FileInput component provides a drag-and-drop area and file picker button
 * for uploading PLC source code files. It performs basic validation on
 * the file extension before reading.
 */
const FileInput: React.FC<FileInputProps> = ({ onFileLoaded }) => {
  const [fileName, setFileName] = useState<string | null>(null);
  const inputRef = useRef<HTMLInputElement>(null);

  /**
   * Supported PLC source code file extensions.
   */
  const supportedExtensions = ['scl', 'st', 'xml', 'il', 'awl'];

  /**
   * Reads the provided file and passes its content to the parent via callback.
   */
  const handleFileSelect = (file: File) => {
    const extension = file.name.split('.').pop()?.toLowerCase();
    if (!extension || !supportedExtensions.includes(extension)) {
      alert('Unsupported file type. Please upload a .scl, .st, .xml, .il, or .awl file.');
      return;
    }
    const reader = new FileReader();
    reader.onload = () => {
      const text = reader.result as string;
      onFileLoaded(text, file.name);
      setFileName(file.name);
    };
    reader.readAsText(file);
  };

  /**
   * Handles file selection via the hidden file input element.
   */
  const handleInputChange: React.ChangeEventHandler<HTMLInputElement> = (e) => {
    const file = e.target.files?.[0];
    if (file) {
      handleFileSelect(file);
    }
  };

  /**
   * Handles files dropped onto the drop zone.
   */
  const handleDrop: React.DragEventHandler<HTMLDivElement> = (e) => {
    e.preventDefault();
    if (e.dataTransfer.files.length > 0) {
      const file = e.dataTransfer.files[0];
      handleFileSelect(file);
    }
  };

  /**
   * Prevents default dragover behaviour to allow dropping.
   */
  const handleDragOver: React.DragEventHandler<HTMLDivElement> = (e) => {
    e.preventDefault();
  };

  /**
   * Focuses the hidden input when the drop area is clicked, opening the file picker.
   */
  const handleClick = () => {
    inputRef.current?.click();
  };

  return (
    <div className="mb-4">
      <label className="block mb-1 font-medium">PLC Source Code File</label>
      <div
        className="flex flex-col items-center justify-center w-full h-32 border-2 border-dashed border-gray-600 rounded-lg p-4 cursor-pointer hover:border-gray-400 transition-colors"
        onDrop={handleDrop}
        onDragOver={handleDragOver}
        onClick={handleClick}
      >
        <p className="text-sm text-gray-400">Drag &amp; drop your file here, or click to browse</p>
        {fileName && (
          <p className="mt-2 text-sm text-gray-300">Selected: {fileName}</p>
        )}
      </div>
      <input
        type="file"
        accept=".scl,.st,.xml,.il,.awl"
        className="hidden"
        ref={inputRef}
        onChange={handleInputChange}
      />
    </div>
  );
};

export default FileInput;