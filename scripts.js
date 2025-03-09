document.addEventListener('DOMContentLoaded', function() {
  // Initialize Feather icons
  feather.replace();
  
  // Theme Toggle
  const themeToggle = document.getElementById('theme-toggle');
  const themeToggleIcon = themeToggle.querySelector('i');
  
  themeToggle.addEventListener('click', () => {
    document.body.classList.toggle('light-mode');
    
    if (document.body.classList.contains('light-mode')) {
      themeToggleIcon.setAttribute('data-feather', 'sun');
    } else {
      themeToggleIcon.setAttribute('data-feather', 'moon');
    }
    
    feather.replace();
    
    // Save user preference
    const theme = document.body.classList.contains('light-mode') ? 'light' : 'dark';
    localStorage.setItem('theme', theme);
  });
  
  // Check for saved user preference
  const savedTheme = localStorage.getItem('theme');
  if (savedTheme === 'light') {
    document.body.classList.add('light-mode');
    themeToggleIcon.setAttribute('data-feather', 'sun');
    feather.replace();
  }
  
  // Copyright year - fixing the syntax error by adding the missing parenthesis
  const currentYear = new Date().getFullYear();
  document.querySelector('.copyright p').textContent = `© ${currentYear} POLA. All rights reserved.`;
  
  // Mobile menu toggle
  const menuToggle = document.getElementById('menu-toggle');
  const mobileNav = document.querySelector('.mobile-nav');
  
  menuToggle.addEventListener('click', () => {
    mobileNav.classList.toggle('active');
  });
  
  // Clone the TOC for mobile
  const toc = document.querySelector('.toc');
  const mobileToc = document.querySelector('.mobile-toc');
  
  if (toc && mobileToc) {
    mobileToc.innerHTML = toc.innerHTML;
    
    // Close mobile nav when a link is clicked
    mobileToc.querySelectorAll('a').forEach(link => {
      link.addEventListener('click', () => {
        mobileNav.classList.remove('active');
      });
    });
  }
  
  // Back to top functionality
  document.querySelectorAll('.back-to-top').forEach(button => {
    button.addEventListener('click', (e) => {
      e.preventDefault();
      window.scrollTo({ top: 0, behavior: 'smooth' });
    });
  });
  
  // Add global back to top button
  const backToTopGlobal = document.createElement('div');
  backToTopGlobal.className = 'back-to-top-global';
  backToTopGlobal.innerHTML = '<i data-feather="chevron-up"></i>';
  document.body.appendChild(backToTopGlobal);
  
  // Initialize the icon
  feather.replace();
  
  // Handle global back to top button
  backToTopGlobal.addEventListener('click', () => {
    window.scrollTo({ top: 0, behavior: 'smooth' });
  });
  
  // Show/hide global back to top button based on scroll position
  window.addEventListener('scroll', () => {
    if (window.scrollY > 300) {
      backToTopGlobal.classList.add('visible');
    } else {
      backToTopGlobal.classList.remove('visible');
    }
  });
  
  // Expandable sections
  document.querySelectorAll('.expandable-toggle').forEach(toggle => {
    toggle.addEventListener('click', () => {
      toggle.classList.toggle('active');
      const content = toggle.nextElementSibling;
      content.classList.toggle('active');
      
      // Check if the span element exists before trying to change its text content
      const spanElement = toggle.querySelector('span');
      if (spanElement) {
        if (content.classList.contains('active')) {
          spanElement.textContent = 'Hide Merged Table';
        } else {
          spanElement.textContent = 'Show Merged Table';
        }
      }
    });
  });
  
  // Search functionality
  const searchInput = document.getElementById('search-input');
  const searchContainer = document.querySelector('.search-container');
  
  if (searchInput) {
    // Focus search with keyboard shortcut
    document.addEventListener('keydown', function(e) {
      if (e.key === '/' && document.activeElement !== searchInput) {
        e.preventDefault();
        searchInput.focus();
      }
    });
    
    searchInput.addEventListener('input', function() {
      const searchTerm = this.value.toLowerCase();
      
      // If the search term is empty, show all content
      if (searchTerm.trim() === '') {
        document.querySelectorAll('table tbody tr').forEach(row => {
          row.style.display = '';
          // Remove any highlighting
          row.innerHTML = row.innerHTML.replace(/<mark class="search-highlight">(.*?)<\/mark>/g, '$1');
        });
        document.querySelectorAll('.section-card, .subsection, .subsubsection, .expandable-content').forEach(section => {
          section.style.display = '';
          if (section.classList.contains('expandable-content')) {
            section.classList.remove('active');
            const toggle = section.previousElementSibling;
            if (toggle && toggle.classList.contains('expandable-toggle')) {
              toggle.classList.remove('active');
            }
          }
        });
        
        // Hide "no results" message when search is empty
        const noResultsMsg = document.querySelector('.no-results-message');
        if (noResultsMsg) {
          noResultsMsg.style.display = 'none';
        }
        
        return;
      }
      
      // Special handling for rarity terms
      const isRaritySearch = ['pink', 'red', 'teal'].includes(searchTerm);
      
      // Keep track of which sections have matches
      const sectionsWithMatches = new Set();
      const subsectionsWithMatches = new Set();
      const subsubsectionsWithMatches = new Set();
      const tableCategories = new Set(); // To track which table categories (pink/red) have matches
      
      // Hide all expandable content sections first
      document.querySelectorAll('.expandable-content').forEach(section => {
        section.style.display = 'none';
        section.classList.remove('active');
        const toggle = section.previousElementSibling;
        if (toggle && toggle.classList.contains('expandable-toggle')) {
          toggle.classList.remove('active');
        }
      });
      
      // Search within tables (but not in expandable content tables)
      document.querySelectorAll('table tbody tr').forEach(row => {
        // Skip rows in expandable content
        if (row.closest('.expandable-content')) return;
        
        const textContent = row.textContent.toLowerCase();
        const cells = row.querySelectorAll('td');
        let hasMatch = false;
        
        // First, revert any previous highlighting
        cells.forEach(cell => {
          // Save the original HTML before any search highlighting was applied
          if (!cell.hasAttribute('data-original-html')) {
            cell.setAttribute('data-original-html', cell.innerHTML);
          } else {
            // Restore original HTML before applying new highlights
            cell.innerHTML = cell.getAttribute('data-original-html');
          }
        });
        
        // Special handling for rarity search
        if (isRaritySearch) {
          // Check if this row contains rarity information
          const hasRarity = row.innerHTML.toLowerCase().includes(`class="rarity ${searchTerm}"`) || 
                           row.innerHTML.toLowerCase().includes(`class="${searchTerm}"`) ||
                           row.innerHTML.toLowerCase().includes(`kbd class="${searchTerm}"`);
          
          // For rarity search, we want to show the row even if it doesn't contain the term directly
          if (hasRarity || textContent.includes(searchTerm)) {
            hasMatch = true;
          }
        } else {
          // Standard search
          hasMatch = textContent.includes(searchTerm);
        }
        
        if (hasMatch) {
          row.style.display = '';
          
          // Highlight the matching text
          cells.forEach(cell => {
            if (cell.textContent.toLowerCase().includes(searchTerm)) {
              // Clone the cell to work with it safely
              const tempDiv = document.createElement('div');
              tempDiv.innerHTML = cell.getAttribute('data-original-html');
              
              // Function to highlight text while preserving HTML structure
              const highlightHTML = (element) => {
                if (element.nodeType === Node.TEXT_NODE) {
                  const text = element.textContent;
                  const lowerText = text.toLowerCase();
                  const index = lowerText.indexOf(searchTerm);
                  
                  if (index >= 0) {
                    // Create text nodes and highlight element
                    const before = document.createTextNode(text.substring(0, index));
                    const match = document.createElement('mark');
                    match.className = 'search-highlight';
                    match.textContent = text.substring(index, index + searchTerm.length);
                    const after = document.createTextNode(text.substring(index + searchTerm.length));
                    
                    // Replace the original text node
                    const parent = element.parentNode;
                    parent.insertBefore(before, element);
                    parent.insertBefore(match, element);
                    parent.insertBefore(after, element);
                    parent.removeChild(element);
                    
                    // Continue searching in the remaining text
                    if (after.textContent.toLowerCase().includes(searchTerm)) {
                      highlightHTML(after);
                    }
                  }
                } else if (element.nodeType === Node.ELEMENT_NODE) {
                  // Skip highlighting in existing mark elements
                  if (element.nodeName !== 'MARK') {
                    // Process child nodes
                    const childNodes = Array.from(element.childNodes);
                    childNodes.forEach(highlightHTML);
                  }
                }
              };
              
              // Process all child nodes
              Array.from(tempDiv.childNodes).forEach(highlightHTML);
              
              // Update cell with highlighted content
              cell.innerHTML = tempDiv.innerHTML;
            }
          });
          
          // Track containing sections
          let parent = row.closest('.section-card');
          if (parent) {
            sectionsWithMatches.add(parent.id);
          }
          
          parent = row.closest('.subsection');
          if (parent) {
            subsectionsWithMatches.add(parent.id);
            
            // Track which category (pink/red) the match belongs to
            const categoryH3 = parent.querySelector('h3');
            if (categoryH3) {
              if (categoryH3.textContent.toLowerCase().includes('pink')) {
                tableCategories.add('pink');
              } else if (categoryH3.textContent.toLowerCase().includes('red')) {
                tableCategories.add('red');
              } else if (categoryH3.textContent.toLowerCase().includes('teal')) {
                tableCategories.add('teal');
              }
            }
          }
          
          parent = row.closest('.subsubsection');
          if (parent) {
            subsubsectionsWithMatches.add(parent.id);
            
            // Track which category (pink/red) the match belongs to
            const categoryH4 = parent.querySelector('h4');
            if (categoryH4) {
              if (categoryH4.textContent.toLowerCase().includes('pink')) {
                tableCategories.add('pink');
              } else if (categoryH4.textContent.toLowerCase().includes('red')) {
                tableCategories.add('red');
              } else if (categoryH4.textContent.toLowerCase().includes('teal')) {
                tableCategories.add('teal');
              }
            }
            
            // Also track its parent subsection
            const subsection = parent.closest('.subsection');
            if (subsection) {
              subsectionsWithMatches.add(subsection.id);
            }
          }
        } else {
          row.style.display = 'none';
        }
      });
      
      // After processing all rows, count total matches
      let matchCount = 0;
      document.querySelectorAll('table tbody tr').forEach(row => {
        if (row.style.display !== 'none' && !row.closest('.expandable-content')) {
          matchCount++;
        }
      });
      
      // Check if we have any matches
      const hasVisibleResults = matchCount > 0;
      
      // Show "no results" message if needed
      let noResultsMsg = document.querySelector('.no-results-message');
      if (!hasVisibleResults && searchTerm.trim() !== '') {
        if (!noResultsMsg) {
          noResultsMsg = document.createElement('div');
          noResultsMsg.className = 'no-results-message';
          noResultsMsg.innerHTML = '<i data-feather="search"></i><p>No results found for "' + searchTerm + '"</p>';
          document.querySelector('.content').prepend(noResultsMsg);
          feather.replace();
        } else {
          noResultsMsg.innerHTML = '<i data-feather="search"></i><p>No results found for "' + searchTerm + '"</p>';
          feather.replace();
          noResultsMsg.style.display = '';
        }
      } else if (noResultsMsg) {
        noResultsMsg.style.display = 'none';
      }
      
      // Hide sections that don't have matches
      document.querySelectorAll('.section-card').forEach(section => {
        if (sectionsWithMatches.has(section.id)) {
          section.style.display = '';
        } else {
          section.style.display = 'none';
        }
      });
      
      // Handle rarity filtering for subsections
      document.querySelectorAll('.subsection').forEach(subsection => {
        if (subsectionsWithMatches.has(subsection.id)) {
          subsection.style.display = '';
          
          // For rarity searches, make sure we show all matching categories
          const subsectionH3 = subsection.querySelector('h3');
          if (subsectionH3 && isRaritySearch) {
            const text = subsectionH3.textContent.toLowerCase();
            if (text.includes(searchTerm)) {
              subsection.style.display = '';
            }
          } else {
            // Hide category-specific subsections that don't match the found categories
            // (e.g., hide Pinks section if only Reds have matches)
            if (subsectionH3) {
              const text = subsectionH3.textContent.toLowerCase();
              if ((text.includes('pink') && !tableCategories.has('pink')) ||
                  (text.includes('red') && !tableCategories.has('red')) ||
                  (text.includes('teal') && !tableCategories.has('teal'))) {
                subsection.style.display = 'none';
              }
            }
          }
          
          // Hide sibling subsections that don't have matches
          const parentSection = subsection.closest('.section-card');
          if (parentSection) {
            parentSection.querySelectorAll('.subsection').forEach(siblingSubsection => {
              if (siblingSubsection !== subsection && !subsectionsWithMatches.has(siblingSubsection.id)) {
                siblingSubsection.style.display = 'none';
              }
            });
          }
        } else {
          subsection.style.display = 'none';
        }
      });
      
      // Hide subsubsections that don't have matches
      document.querySelectorAll('.subsubsection').forEach(subsubsection => {
        if (subsubsectionsWithMatches.has(subsubsection.id)) {
          subsubsection.style.display = '';
          
          // For rarity searches, make sure we show all matching categories
          const subsubsectionH4 = subsubsection.querySelector('h4');
          if (subsubsectionH4 && isRaritySearch) {
            const text = subsubsectionH4.textContent.toLowerCase();
            if (text.includes(searchTerm)) {
              subsubsection.style.display = '';
            }
          } else {
            // Hide category-specific subsubsections that don't match the found categories
            if (subsubsectionH4) {
              const text = subsubsectionH4.textContent.toLowerCase();
              if ((text.includes('pink') && !tableCategories.has('pink')) ||
                  (text.includes('red') && !tableCategories.has('red')) ||
                  (text.includes('teal') && !tableCategories.has('teal'))) {
                subsubsection.style.display = 'none';
              }
            }
          }
          
          // Hide sibling subsubsections that don't have matches
          const parentSubsection = subsubsection.closest('.subsection');
          if (parentSubsection) {
            parentSubsection.querySelectorAll('.subsubsection').forEach(siblingSubsubsection => {
              if (siblingSubsubsection !== subsubsection && !subsubsectionsWithMatches.has(siblingSubsubsection.id)) {
                siblingSubsubsection.style.display = 'none';
              }
            });
          }
        } else {
          subsubsection.style.display = 'none';
        }
      });
    });
  }
  
  // Highlight active section in sidebar based on scroll position
  const sections = document.querySelectorAll('section[id]');
  const navLinks = document.querySelectorAll('.toc a');
  
  // Only set up scroll highlighting if sections exist
  if (sections.length > 0) {
    function highlightNavigation() {
      const scrollPosition = window.scrollY;
      
      sections.forEach(section => {
        const sectionTop = section.offsetTop - 100;
        const sectionHeight = section.offsetHeight;
        const sectionId = section.getAttribute('id');
        
        if (scrollPosition >= sectionTop && scrollPosition < sectionTop + sectionHeight) {
          document.querySelectorAll(`.toc a[href="#${sectionId}"]`).forEach(link => {
            link.classList.add('active');
          });
        } else {
          document.querySelectorAll(`.toc a[href="#${sectionId}"]`).forEach(link => {
            link.classList.remove('active');
          });
        }
      });
    }
    
    window.addEventListener('scroll', highlightNavigation);
  }
});
