/**
 * Apple Human Interface Guidelines (macOS HIG) for GPUI
 * Interactive Documentation & Component Showcase
 */

document.addEventListener('DOMContentLoaded', () => {
  initTheme();
  initAccentPicker();
  initSidebar();
  initInteractiveComponents();
  initCopyCodeButtons();
  initKeyboardShortcuts();
});

/* ==========================================================================
   1. Theme Management (Light / Dark Appearance)
   ========================================================================== */

function initTheme() {
  const themeToggleBtn = document.getElementById('theme-toggle-btn');
  const themeIcon = document.getElementById('theme-icon');
  const themeLabel = document.getElementById('theme-label');

  // Check saved preference or system preference
  const savedTheme = localStorage.getItem('apple_hig_theme');
  const prefersDark = window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches;
  const initialTheme = savedTheme || (prefersDark ? 'dark' : 'light');

  applyTheme(initialTheme);

  if (themeToggleBtn) {
    themeToggleBtn.addEventListener('click', () => {
      const currentTheme = document.documentElement.getAttribute('data-theme') || 'light';
      const newTheme = currentTheme === 'dark' ? 'light' : 'dark';
      applyTheme(newTheme);
      localStorage.setItem('apple_hig_theme', newTheme);
    });
  }

  // Listen to system changes if no preference saved
  if (window.matchMedia) {
    window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', (e) => {
      if (!localStorage.getItem('apple_hig_theme')) {
        applyTheme(e.matches ? 'dark' : 'light');
      }
    });
  }

  function applyTheme(theme) {
    document.documentElement.setAttribute('data-theme', theme);
    if (themeIcon && themeLabel) {
      if (theme === 'dark') {
        themeIcon.innerHTML = `
          <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"></path>
          </svg>
        `;
        themeLabel.textContent = 'Dark';
      } else {
        themeIcon.innerHTML = `
          <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <circle cx="12" cy="12" r="5"></circle>
            <line x1="12" y1="1" x2="12" y2="3"></line>
            <line x1="12" y1="21" x2="12" y2="23"></line>
            <line x1="4.22" y1="4.22" x2="5.64" y2="5.64"></line>
            <line x1="18.36" y1="18.36" x2="19.78" y2="19.78"></line>
            <line x1="1" y1="12" x2="3" y2="12"></line>
            <line x1="21" y1="12" x2="23" y2="12"></line>
            <line x1="4.22" y1="19.78" x2="5.64" y2="18.36"></line>
            <line x1="18.36" y1="5.64" x2="19.78" y2="4.22"></line>
          </svg>
        `;
        themeLabel.textContent = 'Light';
      }
    }
  }
}

/* ==========================================================================
   2. System Accent Color Switcher
   ========================================================================== */

const ACCENT_COLORS = {
  blue: {
    light: '#007AFF',
    lightHover: '#0062CC',
    lightActive: '#004FA3',
    lightTranslucent: 'rgba(0, 122, 255, 0.12)',
    dark: '#0A84FF',
    darkHover: '#3395FF',
    darkActive: '#0070E0',
    darkTranslucent: 'rgba(10, 132, 255, 0.2)',
  },
  purple: {
    light: '#AF52DE',
    lightHover: '#983AC7',
    lightActive: '#7C21AA',
    lightTranslucent: 'rgba(175, 82, 222, 0.12)',
    dark: '#BF5AF2',
    darkHover: '#CB78F5',
    darkActive: '#A93EEF',
    darkTranslucent: 'rgba(191, 90, 242, 0.2)',
  },
  pink: {
    light: '#FF2D55',
    lightHover: '#E0173E',
    lightActive: '#BF0B2F',
    lightTranslucent: 'rgba(255, 45, 85, 0.12)',
    dark: '#FF375F',
    darkHover: '#FF597B',
    darkActive: '#F51A46',
    darkTranslucent: 'rgba(255, 55, 95, 0.2)',
  },
  red: {
    light: '#FF3B30',
    lightHover: '#E02318',
    lightActive: '#BD150B',
    lightTranslucent: 'rgba(255, 59, 48, 0.12)',
    dark: '#FF453A',
    darkHover: '#FF645C',
    darkActive: '#E52B20',
    darkTranslucent: 'rgba(255, 69, 58, 0.2)',
  },
  orange: {
    light: '#FF9500',
    lightHover: '#D97E00',
    lightActive: '#B36800',
    lightTranslucent: 'rgba(255, 149, 0, 0.12)',
    dark: '#FF9F0A',
    darkHover: '#FFAE33',
    darkActive: '#E58A00',
    darkTranslucent: 'rgba(255, 159, 10, 0.2)',
  },
  yellow: {
    light: '#FFCC00',
    lightHover: '#D9AD00',
    lightActive: '#B38F00',
    lightTranslucent: 'rgba(255, 204, 0, 0.15)',
    dark: '#FFD60A',
    darkHover: '#FFDE38',
    darkActive: '#E5BF00',
    darkTranslucent: 'rgba(255, 214, 10, 0.2)',
  },
  green: {
    light: '#34C759',
    lightHover: '#2AA849',
    lightActive: '#228B3C',
    lightTranslucent: 'rgba(52, 199, 89, 0.12)',
    dark: '#30D158',
    darkHover: '#52DB74',
    darkActive: '#25B749',
    darkTranslucent: 'rgba(48, 209, 88, 0.2)',
  },
  graphite: {
    light: '#8E8E93',
    lightHover: '#737378',
    lightActive: '#59595D',
    lightTranslucent: 'rgba(142, 142, 147, 0.15)',
    dark: '#98989D',
    darkHover: '#AEAEB2',
    darkActive: '#7C7C80',
    darkTranslucent: 'rgba(152, 152, 157, 0.2)',
  }
};

function initAccentPicker() {
  const accentDots = document.querySelectorAll('.accent-dot');
  const savedAccent = localStorage.getItem('apple_hig_accent') || 'blue';

  applyAccent(savedAccent);

  accentDots.forEach(dot => {
    dot.addEventListener('click', () => {
      const color = dot.getAttribute('data-accent');
      if (color && ACCENT_COLORS[color]) {
        applyAccent(color);
        localStorage.setItem('apple_hig_accent', color);
      }
    });
  });

  function applyAccent(colorName) {
    const pal = ACCENT_COLORS[colorName] || ACCENT_COLORS.blue;
    const isDark = document.documentElement.getAttribute('data-theme') === 'dark';

    const mainColor = isDark ? pal.dark : pal.light;
    const hoverColor = isDark ? pal.darkHover : pal.lightHover;
    const activeColor = isDark ? pal.darkActive : pal.lightActive;
    const translucentColor = isDark ? pal.darkTranslucent : pal.lightTranslucent;

    document.documentElement.style.setProperty('--apple-accent', mainColor);
    document.documentElement.style.setProperty('--apple-accent-hover', hoverColor);
    document.documentElement.style.setProperty('--apple-accent-active', activeColor);
    document.documentElement.style.setProperty('--apple-accent-translucent', translucentColor);
    document.documentElement.style.setProperty('--apple-focus-ring', `${mainColor}80`);

    accentDots.forEach(d => {
      if (d.getAttribute('data-accent') === colorName) {
        d.classList.add('active');
      } else {
        d.classList.remove('active');
      }
    });
  }

  // Update on theme toggle as well
  const observer = new MutationObserver(() => {
    const currentAccent = localStorage.getItem('apple_hig_accent') || 'blue';
    applyAccent(currentAccent);
  });
  observer.observe(document.documentElement, { attributes: true, attributeFilter: ['data-theme'] });
}

/* ==========================================================================
   3. Sidebar Navigation, Filtering & ScrollSpy
   ========================================================================== */

function initSidebar() {
  const searchInput = document.getElementById('sidebar-search-input');
  const navItems = document.querySelectorAll('.nav-item');
  const navSections = document.querySelectorAll('.nav-section');

  // Search filter
  if (searchInput) {
    searchInput.addEventListener('input', (e) => {
      const q = e.target.value.toLowerCase().trim();
      
      navItems.forEach(item => {
        const text = item.textContent.toLowerCase();
        if (!q || text.includes(q)) {
          item.style.display = 'flex';
        } else {
          item.style.display = 'none';
        }
      });

      // Hide section title if all items hidden
      navSections.forEach(sec => {
        const visibleItems = sec.querySelectorAll('.nav-item[style="display: flex;"], .nav-item:not([style*="display: none"])');
        sec.style.display = visibleItems.length === 0 ? 'none' : 'block';
      });
    });
  }

  // Smooth scroll and active state
  navItems.forEach(link => {
    link.addEventListener('click', (e) => {
      const href = link.getAttribute('href');
      if (href && href.startsWith('#')) {
        e.preventDefault();
        const target = document.querySelector(href);
        if (target) {
          target.scrollIntoView({ behavior: 'smooth', block: 'start' });
          navItems.forEach(i => i.classList.remove('active'));
          link.classList.add('active');
          history.pushState(null, null, href);
        }
      }
    });
  });

  // ScrollSpy with IntersectionObserver
  const sections = document.querySelectorAll('section[id]');
  if ('IntersectionObserver' in window && sections.length > 0) {
    const observer = new IntersectionObserver((entries) => {
      entries.forEach(entry => {
        if (entry.isIntersecting) {
          const id = entry.target.getAttribute('id');
          navItems.forEach(item => {
            if (item.getAttribute('href') === `#${id}`) {
              item.classList.add('active');
            } else {
              item.classList.remove('active');
            }
          });
        }
      });
    }, { rootMargin: '-20% 0px -70% 0px' });

    sections.forEach(s => observer.observe(s));
  }
}

/* ==========================================================================
   4. Interactive Component Playground Simulations
   ========================================================================== */

function initInteractiveComponents() {
  // MacSwitch
  document.querySelectorAll('.mac-switch:not(.disabled)').forEach(el => {
    el.addEventListener('click', () => {
      el.classList.toggle('on');
      const statusLabel = el.closest('.preview-stage')?.querySelector('.switch-status-text');
      if (statusLabel) {
        statusLabel.textContent = el.classList.contains('on') ? 'State: On (Enabled)' : 'State: Off (Disabled)';
      }
    });
  });

  // MacCheckbox
  document.querySelectorAll('.mac-checkbox-interactive').forEach(el => {
    el.addEventListener('click', () => {
      const box = el.querySelector('.mac-checkbox-box');
      if (!box || el.classList.contains('disabled')) return;

      if (box.classList.contains('checked')) {
        box.classList.remove('checked');
        box.classList.add('mixed');
      } else if (box.classList.contains('mixed')) {
        box.classList.remove('mixed');
      } else {
        box.classList.add('checked');
      }
    });
  });

  // MacRadio
  document.querySelectorAll('.mac-radio-group').forEach(group => {
    const radios = group.querySelectorAll('.mac-radio-row:not(.disabled), .mac-radio:not(.disabled)');
    radios.forEach(radio => {
      radio.addEventListener('click', () => {
        radios.forEach(r => r.classList.remove('selected'));
        radio.classList.add('selected');
      });
    });
  });

  // MacSegmentedControl
  document.querySelectorAll('.mac-segmented').forEach(control => {
    const segments = control.querySelectorAll('.mac-segment:not(.disabled)');
    segments.forEach(seg => {
      seg.addEventListener('click', () => {
        segments.forEach(s => s.classList.remove('active'));
        seg.classList.add('active');
      });
    });
  });

  // MacSlider
  document.querySelectorAll('.mac-slider').forEach(slider => {
    const output = slider.closest('.preview-stage')?.querySelector('.slider-val-badge');
    slider.addEventListener('input', () => {
      if (output) output.textContent = `${slider.value}%`;
    });
  });

  // MacStepper
  document.querySelectorAll('.mac-stepper-interactive').forEach(stepper => {
    const display = stepper.closest('.preview-stage')?.querySelector('.stepper-val-badge');
    let val = 1;
    const upBtn = stepper.querySelector('.stepper-up');
    const downBtn = stepper.querySelector('.stepper-down');

    if (upBtn) {
      upBtn.addEventListener('click', () => {
        val++;
        if (display) display.textContent = val;
      });
    }
    if (downBtn) {
      downBtn.addEventListener('click', () => {
        if (val > 0) val--;
        if (display) display.textContent = val;
      });
    }
  });

  // MacColorWell
  document.querySelectorAll('.mac-color-well').forEach(well => {
    const input = well.querySelector('input[type="color"]');
    const swatch = well.querySelector('.mac-color-well-swatch');
    const badge = well.closest('.preview-stage')?.querySelector('.color-val-badge');

    if (input && swatch) {
      input.addEventListener('input', (e) => {
        swatch.style.backgroundColor = e.target.value;
        if (badge) badge.textContent = e.target.value.toUpperCase();
      });
    }
  });

  // MacDisclosure
  document.querySelectorAll('.mac-disclosure-header').forEach(header => {
    header.addEventListener('click', () => {
      const container = header.closest('.mac-disclosure-container');
      if (container) {
        container.classList.toggle('open');
      }
    });
  });

  // MacSidebar Row selection
  document.querySelectorAll('.mac-sidebar-preview').forEach(sidebar => {
    const rows = sidebar.querySelectorAll('.mac-sidebar-row');
    rows.forEach(row => {
      row.addEventListener('click', () => {
        rows.forEach(r => r.classList.remove('active'));
        row.classList.add('active');
      });
    });
  });

  // MacSearchField Clear Action
  document.querySelectorAll('.mac-search-field').forEach(field => {
    const input = field.querySelector('.mac-search-field-input');
    const clearBtn = field.querySelector('.mac-search-clear-btn');
    if (input && clearBtn) {
      input.addEventListener('input', () => {
        clearBtn.style.display = input.value.length > 0 ? 'flex' : 'none';
      });
      clearBtn.addEventListener('click', () => {
        input.value = '';
        clearBtn.style.display = 'none';
        input.focus();
      });
    }
  });

  // MacProgress Interactive Demo
  const startProgressBtn = document.getElementById('demo-progress-btn');
  const progressBar = document.getElementById('demo-progress-bar');
  const progressVal = document.getElementById('demo-progress-val');
  if (startProgressBtn && progressBar && progressVal) {
    let timer = null;
    startProgressBtn.addEventListener('click', () => {
      if (timer) clearInterval(timer);
      let p = 0;
      progressBar.style.width = '0%';
      progressVal.textContent = '0%';
      timer = setInterval(() => {
        p += 5;
        if (p > 100) {
          p = 100;
          clearInterval(timer);
        }
        progressBar.style.width = `${p}%`;
        progressVal.textContent = `${p}%`;
      }, 75);
    });
  }

  // MacDialog Interactive trigger
  const showDialogBtn = document.getElementById('show-dialog-btn');
  const dialogCard = document.getElementById('interactive-dialog-card');
  const dismissDialogBtn = document.getElementById('dismiss-dialog-btn');
  const okDialogBtn = document.getElementById('ok-dialog-btn');

  if (showDialogBtn && dialogCard) {
    showDialogBtn.addEventListener('click', () => {
      dialogCard.style.display = 'flex';
      showDialogBtn.style.display = 'none';
    });
  }
  if (dismissDialogBtn && dialogCard && showDialogBtn) {
    dismissDialogBtn.addEventListener('click', () => {
      dialogCard.style.display = 'none';
      showDialogBtn.style.display = 'inline-flex';
    });
  }
  if (okDialogBtn && dialogCard && showDialogBtn) {
    okDialogBtn.addEventListener('click', () => {
      showToast('Action confirmed in dialog');
      dialogCard.style.display = 'none';
      showDialogBtn.style.display = 'inline-flex';
    });
  }
}

/* ==========================================================================
   5. Copy Code to Clipboard & Toast
   ========================================================================== */

function initCopyCodeButtons() {
  document.querySelectorAll('.copy-btn').forEach(btn => {
    btn.addEventListener('click', async () => {
      const codeBox = btn.closest('.code-box');
      const codeEl = codeBox?.querySelector('code');
      if (codeEl) {
        try {
          await navigator.clipboard.writeText(codeEl.textContent.trim());
          const originalHTML = btn.innerHTML;
          btn.innerHTML = `
            <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="#34C759" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
              <polyline points="20 6 9 17 4 12"></polyline>
            </svg>
            <span style="color: #34C759;">Copied!</span>
          `;
          showToast('Code copied to clipboard');
          setTimeout(() => {
            btn.innerHTML = originalHTML;
          }, 2000);
        } catch (err) {
          console.error('Failed to copy: ', err);
          showToast('Failed to copy code');
        }
      }
    });
  });
}

function showToast(message) {
  let toast = document.querySelector('.toast');
  if (!toast) {
    toast = document.createElement('div');
    toast.className = 'toast';
    document.body.appendChild(toast);
  }
  toast.textContent = message;
  toast.classList.add('show');
  setTimeout(() => {
    toast.classList.remove('show');
  }, 2200);
}

/* ==========================================================================
   6. Keyboard Shortcuts
   ========================================================================== */

function initKeyboardShortcuts() {
  window.addEventListener('keydown', (e) => {
    // ⌘K or Ctrl+K to search
    if ((e.metaKey || e.ctrlKey) && e.key === 'k') {
      e.preventDefault();
      const search = document.getElementById('sidebar-search-input');
      if (search) {
        search.focus();
        search.select();
      }
    }
  });
}
