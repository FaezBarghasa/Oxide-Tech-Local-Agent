import React, { useState, useEffect } from 'react';
import {
  BookOpen,
  FileText,
  FileCode,
  FolderOpen,
  RefreshCw,
  Search,
  MessageSquare,
  ChevronLeft,
  ChevronRight,
  ZoomIn,
  ZoomOut,
  Copy,
  Check,
  Sparkles,
} from 'lucide-react';
import { documentList, documentReadText, documentReadPdfPages } from '../lib/desktop';
import { DocumentInfoDto, PdfPageDto } from '../types';
import { useUI } from '../store/uiStore';

export const LibraryTab: React.FC = () => {
  const { setTab } = useUI();
  const [documents, setDocuments] = useState<DocumentInfoDto[]>([]);
  const [selectedDoc, setSelectedDoc] = useState<DocumentInfoDto | null>(null);
  const [docContent, setDocContent] = useState<string>('');
  const [pdfPages, setPdfPages] = useState<PdfPageDto[]>([]);
  const [currentPage, setCurrentPage] = useState<number>(1);
  const [loading, setLoading] = useState<boolean>(false);
  const [searchQuery, setSearchQuery] = useState<string>('');
  const [copied, setCopied] = useState<boolean>(false);
  const [fontSize, setFontSize] = useState<number>(12);

  const fetchDocuments = async () => {
    setLoading(true);
    try {
      const list = await documentList();
      setDocuments(list);
      if (list.length > 0 && !selectedDoc) {
        handleSelectDoc(list[0]);
      }
    } catch (err) {
      console.error('Failed to load documents', err);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchDocuments();
  }, []);

  const handleSelectDoc = async (doc: DocumentInfoDto) => {
    setSelectedDoc(doc);
    setLoading(true);
    setCurrentPage(1);

    try {
      if (doc.is_pdf) {
        const pages = await documentReadPdfPages(doc.path);
        setPdfPages(pages);
        setDocContent(pages[0]?.text_content || '');
      } else {
        const text = await documentReadText(doc.path);
        setDocContent(text);
        setPdfPages([]);
      }
    } catch (err) {
      console.error('Failed to read document', err);
      setDocContent('Error loading document content.');
    } finally {
      setLoading(false);
    }
  };

  const handlePageChange = (newPage: number) => {
    if (newPage < 1 || newPage > pdfPages.length) return;
    setCurrentPage(newPage);
    setDocContent(pdfPages[newPage - 1]?.text_content || '');
  };

  const handleCopy = () => {
    navigator.clipboard.writeText(docContent);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  const handleChatWithDoc = () => {
    // Navigate to chat tab
    setTab('chat');
  };

  const filteredDocs = documents.filter(
    (d) =>
      d.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
      d.extension.toLowerCase().includes(searchQuery.toLowerCase())
  );

  return (
    <div className="flex flex-col gap-6 max-w-7xl mx-auto pb-12">
      {/* Header */}
      <div className="flex flex-col md:flex-row items-start md:items-center justify-between gap-4 bg-[#111113] p-5 rounded-xl border border-[#27272A]">
        <div className="flex items-center gap-3.5">
          <div className="w-10 h-10 rounded-lg bg-cyan-500/10 border border-cyan-500/30 flex items-center justify-center text-cyan-400">
            <BookOpen className="w-5 h-5" />
          </div>
          <div>
            <h1 className="text-lg font-semibold text-zinc-100 flex items-center gap-2">
              Document Library & In-App PDF Viewer
              <span className="text-[11px] px-2 py-0.5 rounded bg-cyan-500/20 text-cyan-300 font-mono border border-cyan-500/30">
                Native PDF/DOCX Parser
              </span>
            </h1>
            <p className="text-xs text-zinc-400">
              Browse workspace datasheets, design documents, and PDFs with token-aware page slicing and direct chat handoff.
            </p>
          </div>
        </div>

        <div className="flex items-center gap-2.5">
          <button
            onClick={fetchDocuments}
            className="px-3 py-1.5 rounded-lg bg-[#18181B] border border-[#27272A] hover:bg-[#27272A] text-xs font-medium text-zinc-300 flex items-center gap-1.5 transition-colors cursor-pointer"
          >
            <RefreshCw className={`w-3.5 h-3.5 ${loading ? 'animate-spin' : ''}`} />
            Scan Docs
          </button>
          {selectedDoc && (
            <button
              onClick={handleChatWithDoc}
              className="px-3.5 py-1.5 rounded-lg bg-cyan-600 hover:bg-cyan-500 text-xs font-semibold text-white flex items-center gap-1.5 transition-colors shadow-sm cursor-pointer"
            >
              <MessageSquare className="w-3.5 h-3.5" />
              Chat with this Doc
            </button>
          )}
        </div>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-12 gap-6">
        {/* Left Sidebar: Document List */}
        <div className="lg:col-span-4 flex flex-col gap-3">
          <div className="relative">
            <Search className="w-4 h-4 absolute left-3 top-2.5 text-zinc-500" />
            <input
              type="text"
              placeholder="Search library documents..."
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              className="w-full pl-9 pr-3 py-2 bg-[#111113] border border-[#27272A] rounded-lg text-xs text-zinc-200 placeholder-zinc-500 focus:outline-none focus:border-cyan-500/50"
            />
          </div>

          <div className="bg-[#111113] border border-[#27272A] rounded-xl overflow-hidden flex flex-col divide-y divide-[#1F1F23]">
            {filteredDocs.length === 0 ? (
              <div className="p-8 text-center text-xs text-zinc-500">
                <FolderOpen className="w-8 h-8 mx-auto mb-2 text-zinc-600" />
                No documents found in workspace
              </div>
            ) : (
              filteredDocs.map((doc) => {
                const isSelected = selectedDoc?.path === doc.path;
                return (
                  <button
                    key={doc.path}
                    onClick={() => handleSelectDoc(doc)}
                    className={`p-3 text-left transition-colors flex items-center justify-between gap-3 ${
                      isSelected ? 'bg-cyan-950/30 border-l-2 border-cyan-500' : 'hover:bg-[#18181B]'
                    }`}
                  >
                    <div className="flex items-center gap-2.5 min-w-0">
                      {doc.is_pdf ? (
                        <FileText className="w-4 h-4 text-red-400 shrink-0" />
                      ) : (
                        <FileCode className="w-4 h-4 text-cyan-400 shrink-0" />
                      )}
                      <div className="flex flex-col min-w-0">
                        <span className="text-xs font-semibold text-zinc-200 truncate">{doc.name}</span>
                        <span className="text-[10px] text-zinc-500 uppercase">{doc.extension} • {doc.size_formatted}</span>
                      </div>
                    </div>
                  </button>
                );
              })
            )}
          </div>
        </div>

        {/* Right Panel: Embedded Document & PDF Viewer */}
        <div className="lg:col-span-8 flex flex-col gap-4">
          {selectedDoc ? (
            <div className="bg-[#111113] border border-[#27272A] rounded-xl p-5 flex flex-col gap-4">
              {/* Document Toolbar */}
              <div className="flex flex-wrap items-center justify-between gap-3 border-b border-[#27272A] pb-3.5">
                <div className="flex items-center gap-2 min-w-0">
                  <span className="text-xs font-bold text-zinc-200 truncate">{selectedDoc.name}</span>
                  <span className="text-[10px] px-2 py-0.5 rounded bg-zinc-800 text-zinc-400 font-mono">
                    {selectedDoc.size_formatted}
                  </span>
                </div>

                <div className="flex items-center gap-2">
                  {pdfPages.length > 1 && (
                    <div className="flex items-center gap-1.5 bg-[#0A0A0A] border border-[#27272A] px-2 py-1 rounded text-xs">
                      <button
                        onClick={() => handlePageChange(currentPage - 1)}
                        disabled={currentPage <= 1}
                        className="p-0.5 hover:text-cyan-400 disabled:opacity-30"
                      >
                        <ChevronLeft className="w-3.5 h-3.5" />
                      </button>
                      <span className="text-[11px] text-zinc-400 font-mono">
                        {currentPage} / {pdfPages.length}
                      </span>
                      <button
                        onClick={() => handlePageChange(currentPage + 1)}
                        disabled={currentPage >= pdfPages.length}
                        className="p-0.5 hover:text-cyan-400 disabled:opacity-30"
                      >
                        <ChevronRight className="w-3.5 h-3.5" />
                      </button>
                    </div>
                  )}

                  <div className="flex items-center gap-1 bg-[#0A0A0A] border border-[#27272A] px-1.5 py-1 rounded">
                    <button
                      onClick={() => setFontSize((f) => Math.max(10, f - 1))}
                      className="p-0.5 hover:text-zinc-200 text-zinc-500"
                    >
                      <ZoomOut className="w-3.5 h-3.5" />
                    </button>
                    <span className="text-[10px] text-zinc-400 font-mono px-1">{fontSize}px</span>
                    <button
                      onClick={() => setFontSize((f) => Math.min(18, f + 1))}
                      className="p-0.5 hover:text-zinc-200 text-zinc-500"
                    >
                      <ZoomIn className="w-3.5 h-3.5" />
                    </button>
                  </div>

                  <button
                    onClick={handleCopy}
                    className="p-1.5 bg-[#18181B] hover:bg-[#27272A] border border-[#27272A] rounded text-zinc-300 text-xs flex items-center gap-1"
                  >
                    {copied ? <Check className="w-3.5 h-3.5 text-emerald-400" /> : <Copy className="w-3.5 h-3.5" />}
                  </button>
                </div>
              </div>

              {/* Document Text Rendering Pane */}
              <div
                style={{ fontSize: `${fontSize}px` }}
                className="min-h-[500px] max-h-[700px] overflow-y-auto p-4 bg-[#0A0A0A] border border-[#1F1F23] rounded-lg font-mono text-zinc-300 whitespace-pre-wrap leading-relaxed select-text scrollbar-thin"
              >
                {loading ? (
                  <div className="flex items-center justify-center h-64 text-zinc-500 gap-2">
                    <RefreshCw className="w-4 h-4 animate-spin text-cyan-400" />
                    Extracting document text...
                  </div>
                ) : (
                  docContent || <span className="text-zinc-600 italic">Empty document.</span>
                )}
              </div>
            </div>
          ) : (
            <div className="bg-[#111113] border border-[#27272A] rounded-xl p-12 text-center text-zinc-500 text-xs">
              Select a document from the left library to preview.
            </div>
          )}
        </div>
      </div>
    </div>
  );
};
