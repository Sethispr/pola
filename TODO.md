optimizing to-do's:

- reduce allocations
- more cache-friendly data structures
- more efficient index-based operations
- reuse fuzzy matcher instance
- change fuzzy matching to using nucleo (x8 times faster)
- use indices instead of cloning entire skin objects
- compressing term metadata with bitflags in TermInfo reducing memory usage

features to-do's:
- add value and owner tags
- allow tags to be excluded
- combining rarity tags such as pink and red can still show pink and red skins
- fix number of suggestions features
- fix non tags being tagged
- keyboard shortcut gamepad changer
- tui -> gui or standalone terminal
- use 256 bit colors from ratatui
- add mod-only skins
- add description to detailed view
- add placeholder images to all skins and start adding skins to assets folder
- improve auto suggestions (ex: if you type valentine then type sth it should auto suggest a skin in a valentine case)
- 
