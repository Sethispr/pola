const { useState } = window.React;
const { Logo, SearchBar, TagFilter, SkinCard } = window;

function App() {
  const [searchQuery, setSearchQuery] = useState('');
  const [selectedTags, setSelectedTags] = useState(new Map());
  const [showFilters, setShowFilters] = useState(false);
  const [sortMode, setSortMode] = useState('default');
  
  const rarityTags = ['teal', 'pink', 'red'];
  const sourceTags = ['Summer', 'Valentine', 'Birthday', 'Halloween', 'Christmas'];
  
  const skins = [
    {
      name: "Void Lord",
      rarity: "pink",
      source: "Summer"
    },
    {
      name: "All Hallows",
      rarity: "pink",
      source: "Halloween"
    },
    {
      name: "Iceblade",
      rarity: "teal",
      source: "Christmas"
    },
    {
      name: "Crimson Periastron",
      rarity: "red",
      source: "Valentine"
    }
  ];

  const sortOptions = [
    { id: 'default', label: 'Default' },
    { id: 'name-asc', label: 'Name (A-Z)' },
    { id: 'name-desc', label: 'Name (Z-A)' },
    { id: 'rarity', label: 'Rarity' },
    { id: 'source', label: 'Source' }
  ];
  
  const handleTagClick = (tag) => {
    const currentState = selectedTags.get(tag);
    const newTags = new Map(selectedTags);
    
    if (!currentState) {
      newTags.set(tag, 'include');
    } else if (currentState === 'include') {
      newTags.set(tag, 'exclude');
    } else {
      newTags.delete(tag);
    }
    
    setSelectedTags(newTags);
  };

  const filterSkins = () => {
    let filtered = skins.filter(skin => {
      if (searchQuery && !skin.name.toLowerCase().includes(searchQuery.toLowerCase())) {
        return false;
      }

      if (!searchQuery && selectedTags.size === 0) {
        return false;
      }

      const includeFilters = {
        rarity: 0,
        source: 0
      };
      const matchesInclude = {
        rarity: false,
        source: false
      };

      for (const [tag, state] of selectedTags.entries()) {
        const isRarityTag = rarityTags.includes(tag);
        const category = isRarityTag ? 'rarity' : 'source';
        const isTagMatch = isRarityTag ? 
          skin.rarity === tag.toLowerCase() : 
          skin.source === tag;

        if (state === 'include') {
          includeFilters[category]++;
          if (isTagMatch) {
            matchesInclude[category] = true;
          }
        } else if (state === 'exclude' && isTagMatch) {
          return false;
        }
      }

      for (const category in includeFilters) {
        if (includeFilters[category] > 0 && !matchesInclude[category]) {
          return false;
        }
      }

      return true;
    });

    switch (sortMode) {
      case 'name-asc':
        filtered.sort((a, b) => a.name.localeCompare(b.name));
        break;
      case 'name-desc':
        filtered.sort((a, b) => b.name.localeCompare(a.name));
        break;
      case 'rarity':
        filtered.sort((a, b) => a.rarity.localeCompare(b.rarity));
        break;
      case 'source':
        filtered.sort((a, b) => a.source.localeCompare(b.source));
        break;
      default:
        break;
    }

    return filtered;
  };

  return (
    <div className="min-h-screen p-8 bg-gray-900 text-white">
      <div className="max-w-6xl mx-auto space-y-8">
        <div className="max-w-2xl mx-auto space-y-8">
          <Logo />
          
          <div className="space-y-6">
            <SearchBar 
              value={searchQuery}
              onChange={setSearchQuery}
              onToggleFilters={() => setShowFilters(!showFilters)}
              showFilters={showFilters}
            />
            
            {showFilters && (
              <div className="space-y-6">
                <div className="space-y-4">
                  <TagFilter
                    title="Rarity"
                    tags={rarityTags}
                    selectedTags={selectedTags}
                    onTagClick={handleTagClick}
                  />
                  
                  <TagFilter
                    title="Source"
                    tags={sourceTags}
                    selectedTags={selectedTags}
                    onTagClick={handleTagClick}
                  />
                </div>

                <div className="space-y-2">
                  <h3 className="text-sm font-semibold text-gray-400">Sort By</h3>
                  <select 
                    value={sortMode}
                    onChange={(e) => setSortMode(e.target.value)}
                    className="px-4 py-2 rounded-lg border appearance-none cursor-pointer bg-gray-800 border-gray-700 text-white hover:bg-gray-700"
                    style={{
                      backgroundImage: `url("data:image/svg+xml,%3csvg xmlns='http://www.w3.org/2000/svg' fill='none' viewBox='0 0 20 20'%3e%3cpath stroke='%23ffffff' stroke-linecap='round' stroke-linejoin='round' stroke-width='1.5' d='M6 8l4 4 4-4'/%3e%3c/svg%3e")`,
                      backgroundPosition: 'right 0.5rem center',
                      backgroundRepeat: 'no-repeat',
                      backgroundSize: '1.5em 1.5em',
                      paddingRight: '2.5rem'
                    }}
                  >
                    {sortOptions.map(option => (
                      <option key={option.id} value={option.id}>
                        {option.label}
                      </option>
                    ))}
                  </select>
                </div>
              </div>
            )}
          </div>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          {filterSkins().map(skin => (
            <SkinCard
              key={skin.name}
              name={skin.name}
              rarity={skin.rarity}
              source={skin.source}
            />
          ))}
        </div>
      </div>
    </div>
  );
}

ReactDOM.createRoot(document.getElementById('root')).render(<App />);
