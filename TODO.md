optimizing to-do's:

- reduce allocations
- more cache-friendly data structures
- make tags capitalized at first letter?
- more efficient index-based operations
- reuse fuzzy matcher instance
- change fuzzy matching to using nucleo (x8 times faster)
- use indices instead of cloning entire skin objects
- compressing term metadata with bitflags in TermInfo reducing memory usage
- improve tags especially long ones such as valentines case (exquisite) and just separate tags using exquisite and valentines as tags

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
- fix auto suggestions where if u input sth like "omg" and accept the suggestion omega, omg will still be in search bar which looks not so clean. or when cycling suggestions it happens too

website to-do's:
- make only the top left border rounded specifically since sa skins won't look good in a fully rounded square
- optimize suggestions speed
- add all images for the little amount of skins we currently have in website
- add all skins, images into assets
- fix tags and help modal and add rarity, value, owner and year tags
- get owner list thrugh trade-sa or ask people or promote site
- remove name and rarity from detailed view since its redudant if we have a skin image already showing it
- add max-width for images
- make tags capitalized at first letter

