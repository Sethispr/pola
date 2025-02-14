window.TagFilter = function TagFilter({ title, tags, selectedTags, onTagClick }) {
  const getTagStyle = (tag) => {
    const state = selectedTags.get(tag);
    if (state === 'include') {
      return 'bg-primary text-white';
    } else if (state === 'exclude') {
      return 'bg-red-500 text-white border-red-600';
    }
    return 'bg-gray-800 text-white hover:bg-gray-700';
  };

  const getTagIcon = (tag) => {
    const state = selectedTags.get(tag);
    if (state === 'include') {
      return '+';
    } else if (state === 'exclude') {
      return '-';
    }
    return '';
  };

  return (
    <div className="space-y-2">
      <h3 className="text-sm font-semibold text-gray-400">{title}</h3>
      <div className="flex flex-wrap gap-2">
        {tags.map(tag => (
          <button
            key={tag}
            onClick={() => onTagClick(tag)}
            className={`px-4 py-2 rounded-lg border border-gray-700 font-medium transition-colors ${getTagStyle(tag)}`}
          >
            {getTagIcon(tag)} {tag}
          </button>
        ))}
      </div>
    </div>
  );
}
