window.SkinCard = function SkinCard({ name, rarity, source }) {
  return (
    <div className="rounded-xl shadow-sm p-4 space-y-4 transition-colors bg-gray-800">
      <div className="w-32 h-32 mx-auto rounded-lg bg-gray-700"></div>
      <div className="space-y-2">
        <h3 className="font-bold text-lg">{name}</h3>
        <div className="flex gap-2">
          <span className={`px-2 py-1 rounded-md text-sm font-medium bg-${rarity}/10 text-${rarity}`}>
            {rarity}
          </span>
          <span className="px-2 py-1 rounded-md text-sm font-medium bg-blue-50 text-blue-600">
            {source}
          </span>
        </div>
      </div>
    </div>
  );
}
