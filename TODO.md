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
- allow tags to be excluded by using -- ? or -
- combining rarity tags such as pink and red can still show pink and red skins or summer and pattern case can show both
- fix number of suggestions features (fixed)
- fix non tags being tagged
- keyboard shortcut changer
- tui -> gui or standalone terminal
- use 256 bit colors from ratatui (slightly added, only fg, border color and bg of binds etc not the whole bg yet)
- add mod-only skins (added)
- add description to detailed view
- add placeholder images to all skins and start adding skins to assets folder
- improve auto suggestions (ex: if you type valentine then type sth it should auto suggest a skin in a valentine case)
- fix auto suggestions where if u input sth like "omg" and accept the suggestion omega, omg will still be in search bar which looks not so clean. or when cycling suggestions it happens too
- add highlight feature like the website where if you type in anything it also shows what you input highlighted in the suggested skins
- ten results per table page? then cycle through next pages using keybind or arrow button, can also customize amount of results shown per page
- logging and advanced error handling
- performance monitoring view and binds for it
- add scrollbar
- make ui more modern and more flexible with other terminals
- separate codes into different files not just 1 whole main.rs file
- adding a caret in the search bar
- keybind to,toggle detail view on/off default is on
- ctrl + r to select random skin
- contextual suggestion should get refined since case and event overlaps e.g. valentine case and bundle or christmas case and event
- putting 2 in the search doesnt suggest 2022 etc year tags
- skins with popular tag should be suggested first for any prompt trying for a name
- when typing red and then s it should be showing skins with names that start with s like salmon or sanctum
- skin translation in diff lang spanish russian, config for lang
- favorite or star system OR add/edit tags like add favorite / for trade tags
- persistent bg color so can make light mode theme and hc
- support importing skin data from csv or json
- select multiple skins using shift up down, can export or add to favs or compare
- search results stats and overall stats of skins in sa
- export list of skin to csv/json/txt format
- x icon in search bar at the right to clear search
- introduce operators like and or not similar to include exclude etc skins
- year range search e.g. 2022-2025
- show current shop, needs live backend
- search history clear history
- fix block styling when theres 3 tags particularly the valentine/birthday exquisite case skins

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

pola-cli to-do's:
- animated ASCII intro?
- featured skin of the day
- random skin feature
- export results
- favorite skins
- introduce operators AND OR NOT similar to include/exclude etc skins
- ANSI colors?
- history and clear history
- numbered results and quick selection
- compact view
- value converter pink = 4 reds = ... 256 blues

github to-do's:
- neaten out tables in skin.md
- put all github related md files into a .github folder

tui bugs to fix:
- suggestion isnt too smart (not contextual), also doesnt clear input after accpeting suggestion (fixed)
- after turning off detailed view rarity descending sort feature activates the event descending sort instead (found reason: the state doesnt actually update where the mouse is suppposed to click so theres an offset)
- ctrl+y redo feature not working
- click feature doesnt work after 12 results, it offsets
- number of suggestions feature doesnt actually show accurate number (fixed)
- caret being shown near the block styled tags in detailed view when scrolling fast
