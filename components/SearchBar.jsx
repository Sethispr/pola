window.SearchBar = function SearchBar({ value, onChange, onToggleFilters, showFilters }) {
  const searchInputRef = React.useRef(null);

  const handleClear = (e) => {
    e.preventDefault();
    onChange('');
    if (searchInputRef.current) {
      searchInputRef.current.focus();
    }
  };

  React.useEffect(() => {
    const handleKeyDown = (e) => {
      if ((e.ctrlKey || e.metaKey) && e.key === 'k') {
        e.preventDefault();
        searchInputRef.current?.focus();
      }
    };

    document.addEventListener('keydown', handleKeyDown);
    return () => document.removeEventListener('keydown', handleKeyDown);
  }, []);

  const isMac = navigator.platform.toUpperCase().indexOf('MAC') >= 0;
  const modifierKey = isMac ? '⌘' : 'Ctrl';

  return (
    <div className="max-w-2xl mx-auto relative group">
      <div className="relative">
        <div className="absolute left-6 top-1/2 -translate-y-1/2 text-gray-400">
          <svg xmlns="http://www.w3.org/2000/svg" className="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
            <path fillRule="evenodd" d="M8 4a4 4 0 100 8 4 4 0 000-8zM2 8a6 6 0 1110.89 3.476l4.817 4.817a1 1 0 01-1.414 1.414l-4.816-4.816A6 6 0 012 8z" clipRule="evenodd" />
          </svg>
        </div>
        
        <input
          ref={searchInputRef}
          type="text"
          placeholder="Search skins..."
          value={value}
          onChange={(e) => onChange(e.target.value)}
          className="w-full pl-14 pr-28 py-4 text-lg rounded-xl border-2 bg-gray-800 border-gray-700 text-white placeholder-gray-400 focus:ring-2 focus:ring-primary focus:border-transparent focus:outline-none transition-colors duration-200"
        />

        <div className="absolute right-4 top-1/2 -translate-y-1/2 flex items-center gap-2">
          {value && (
            <button
              onClick={handleClear}
              className="p-1.5 rounded-lg text-gray-400 hover:text-white hover:bg-gray-700 transition-colors"
              type="button"
            >
              <svg xmlns="http://www.w3.org/2000/svg" className="h-6 w-6" viewBox="0 0 20 20" fill="currentColor">
                <path fillRule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z" clipRule="evenodd" />
              </svg>
            </button>
          )}
          
          <div className="h-8 w-px bg-gray-700"></div>
          
          <button
            onClick={(e) => {
              e.stopPropagation();
              onToggleFilters();
            }}
            className={`p-1.5 rounded-lg transition-all duration-200 ${
              showFilters ? 'bg-primary text-white' : 'text-gray-400 hover:text-white hover:bg-gray-700'
            }`}
            title="Toggle Filters"
          >
            <svg xmlns="http://www.w3.org/2000/svg" className="h-6 w-6" viewBox="0 0 20 20" fill="currentColor">
              <path fillRule="evenodd" d="M3 3a1 1 0 011-1h12a1 1 0 011 1v3a1 1 0 01-.293.707L12 11.414V15a1 1 0 01-.293.707l-2 2A1 1 0 018 17v-5.586L3.293 6.707A1 1 0 013 6V3z" clipRule="evenodd" />
            </svg>
          </button>
        </div>
      </div>
      
      <div className="absolute -bottom-6 left-6 text-xs text-gray-500 flex items-center gap-4">
        <div>
          Press <kbd className="px-2 py-0.5 text-xs rounded bg-gray-700 text-gray-300">{modifierKey}</kbd> + <kbd className="px-2 py-0.5 text-xs rounded bg-gray-700 text-gray-300">K</kbd> to search
        </div>
      </div>
    </div>
  );
}
