/**
 * Apple Human Interface Guidelines (macOS HIG) for GPUI
 * Interactive Documentation & Component Showcase
 */

document.addEventListener('DOMContentLoaded', () => {
  initTheme();
  initAccentPicker();
  initSidebar();
  initPlaygroundTabs();
  initCliBadges();
  initInteractivePlaygrounds();
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

      navSections.forEach(sec => {
        const visibleItems = sec.querySelectorAll('.nav-item[style="display: flex;"], .nav-item:not([style*="display: none"])');
        sec.style.display = visibleItems.length === 0 ? 'none' : 'block';
      });
    });
  }

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
   4. Playground Tabs (Preview vs Code vs Anatomy)
   ========================================================================== */

function initPlaygroundTabs() {
  document.querySelectorAll('.playground-tabs').forEach(tabGroup => {
    const buttons = tabGroup.querySelectorAll('.playground-tab-btn');
    buttons.forEach(btn => {
      btn.addEventListener('click', () => {
        buttons.forEach(b => b.classList.remove('active'));
        btn.classList.add('active');

        const wrapper = btn.closest('.playground-wrapper');
        const mode = btn.getAttribute('data-tab');
        if (wrapper && mode) {
          const stage = wrapper.querySelector('.playground-stage');
          const sidebar = wrapper.querySelector('.playground-sidebar');
          const codeView = wrapper.querySelector('.playground-code-view');

          if (mode === 'code') {
            if (stage) stage.style.display = 'none';
            if (sidebar) sidebar.style.display = 'none';
            if (codeView) codeView.style.display = 'block';
          } else {
            if (stage) stage.style.display = 'flex';
            if (sidebar) sidebar.style.display = 'flex';
            if (codeView) codeView.style.display = 'none';
          }
        }
      });
    });
  });
}

function initCliBadges() {
  document.querySelectorAll('.playground-cli-badge').forEach(badge => {
    badge.addEventListener('click', async () => {
      const cmd = badge.textContent.trim().replace(/^>\s*/, '');
      try {
        await navigator.clipboard.writeText(cmd);
        showToast(`Copied: ${cmd}`);
      } catch (err) {
        showToast('Failed to copy command');
      }
    });
  });
}

/* ==========================================================================
   5. Interactive Component Playgrounds with Live Prop Knobs
   ========================================================================== */

function initInteractivePlaygrounds() {
  initMaterialsPlayground();

  // ----------------------------------------------------
  // A. MacSwitch Playground
  // ----------------------------------------------------
  const switchTarget = document.getElementById('knob-target-switch');
  const switchLabelTarget = document.getElementById('knob-target-switch-label');
  const switchCode = document.getElementById('dynamic-code-switch');

  const knobSwitchSize = document.getElementById('knob-switch-size');
  const knobSwitchState = document.getElementById('knob-switch-state');
  const knobSwitchDisabled = document.getElementById('knob-switch-disabled');
  const knobSwitchText = document.getElementById('knob-switch-text');

  function updateSwitch() {
    if (!switchTarget) return;
    const isMini = knobSwitchSize?.value === 'mini';
    const isOn = knobSwitchState?.checked ?? true;
    const isDisabled = knobSwitchDisabled?.checked ?? false;
    const text = knobSwitchText?.value ?? 'Background Indexing';

    if (isMini) switchTarget.classList.add('mini'); else switchTarget.classList.remove('mini');
    if (isOn) switchTarget.classList.add('on'); else switchTarget.classList.remove('on');
    if (isDisabled) switchTarget.classList.add('disabled'); else switchTarget.classList.remove('disabled');

    if (switchLabelTarget) switchLabelTarget.textContent = text;

    if (switchCode) {
      let code = `MacSwitch::new("my-switch", ${isOn})\n`;
      if (isMini) code += `    .size(MacSwitchSize::Mini)\n`;
      if (isDisabled) code += `    .disabled(true)\n`;
      code += `    .on_click(cx.listener(|this, _event, cx| {\n        this.enabled = !this.enabled;\n        cx.notify();\n    }))`;
      switchCode.textContent = code;
    }
  }

  if (switchTarget) {
    switchTarget.addEventListener('click', () => {
      if (switchTarget.classList.contains('disabled')) return;
      const willBeOn = !switchTarget.classList.contains('on');
      if (knobSwitchState) knobSwitchState.checked = willBeOn;
      updateSwitch();
    });
    knobSwitchSize?.addEventListener('change', updateSwitch);
    knobSwitchState?.addEventListener('change', updateSwitch);
    knobSwitchDisabled?.addEventListener('change', updateSwitch);
    knobSwitchText?.addEventListener('input', updateSwitch);
  }

  // ----------------------------------------------------
  // B. MacButton Playground
  // ----------------------------------------------------
  const buttonTarget = document.getElementById('knob-target-button');
  const buttonCode = document.getElementById('dynamic-code-button');

  const knobBtnVariant = document.getElementById('knob-btn-variant');
  const knobBtnSize = document.getElementById('knob-btn-size');
  const knobBtnDisabled = document.getElementById('knob-btn-disabled');
  const knobBtnLoading = document.getElementById('knob-btn-loading');
  const knobBtnText = document.getElementById('knob-btn-text');

  let clickCount = 0;
  function updateButton() {
    if (!buttonTarget) return;
    const variant = knobBtnVariant?.value || 'prominent';
    const size = knobBtnSize?.value || 'regular';
    const isDisabled = knobBtnDisabled?.checked || false;
    const isLoading = knobBtnLoading?.checked || false;
    const text = knobBtnText?.value || 'Save Changes';

    buttonTarget.className = 'mac-button';
    if (variant !== 'push') buttonTarget.classList.add(variant);
    if (size !== 'regular') buttonTarget.classList.add(size);
    if (isDisabled) buttonTarget.classList.add('disabled');

    if (isLoading) {
      buttonTarget.innerHTML = `<div class="mac-spinner small" style="margin-right: 6px;"></div><span>${text}</span>`;
    } else {
      buttonTarget.textContent = text;
    }

    if (buttonCode) {
      let code = `MacButton::new("action-btn", "${text}")\n`;
      if (variant === 'prominent') code += `    .prominent()\n`;
      else if (variant === 'destructive') code += `    .destructive()\n`;
      else if (variant === 'borderless') code += `    .borderless()\n`;

      if (size === 'small') code += `    .size(MacButtonSize::Small)\n`;
      else if (size === 'large') code += `    .size(MacButtonSize::Large)\n`;

      if (isDisabled) code += `    .disabled(true)\n`;
      code += `    .on_click(cx.listener(|this, _event, cx| {\n        this.save_changes();\n    }))`;
      buttonCode.textContent = code;
    }
  }

  if (buttonTarget) {
    buttonTarget.addEventListener('click', () => {
      if (buttonTarget.classList.contains('disabled')) return;
      clickCount++;
      showToast(`Button clicked (${clickCount})`);
    });
    knobBtnVariant?.addEventListener('change', updateButton);
    knobBtnSize?.addEventListener('change', updateButton);
    knobBtnDisabled?.addEventListener('change', updateButton);
    knobBtnLoading?.addEventListener('change', updateButton);
    knobBtnText?.addEventListener('input', updateButton);
  }

  // ----------------------------------------------------
  // C. MacCheckbox Playground
  // ----------------------------------------------------
  const checkboxRowTarget = document.getElementById('knob-target-checkbox-row');
  const checkboxBoxTarget = document.getElementById('knob-target-checkbox-box');
  const checkboxLabelTarget = document.getElementById('knob-target-checkbox-label');
  const checkboxCode = document.getElementById('dynamic-code-checkbox');

  const knobCheckState = document.getElementById('knob-check-state');
  const knobCheckDisabled = document.getElementById('knob-check-disabled');
  const knobCheckText = document.getElementById('knob-check-text');

  function updateCheckbox() {
    if (!checkboxBoxTarget) return;
    const state = knobCheckState?.value || 'checked';
    const isDisabled = knobCheckDisabled?.checked || false;
    const text = knobCheckText?.value || 'Preserve GPS location data';

    checkboxBoxTarget.classList.remove('checked', 'mixed');
    if (state === 'checked') checkboxBoxTarget.classList.add('checked');
    else if (state === 'mixed') checkboxBoxTarget.classList.add('mixed');

    if (isDisabled) checkboxRowTarget?.classList.add('disabled');
    else checkboxRowTarget?.classList.remove('disabled');

    if (checkboxLabelTarget) checkboxLabelTarget.textContent = text;

    if (checkboxCode) {
      let stateStr = state === 'checked' ? 'MacCheckboxState::Checked' : (state === 'mixed' ? 'MacCheckboxState::Mixed' : 'MacCheckboxState::Unchecked');
      let code = `MacCheckbox::new("meta-check", ${stateStr})\n    .label("${text}")\n`;
      if (isDisabled) code += `    .disabled(true)\n`;
      code += `    .on_click(cx.listener(|this, state, cx| {\n        this.preserve_gps = state.is_checked();\n    }))`;
      checkboxCode.textContent = code;
    }
  }

  if (checkboxRowTarget) {
    checkboxRowTarget.addEventListener('click', () => {
      if (checkboxRowTarget.classList.contains('disabled')) return;
      const current = knobCheckState?.value || 'checked';
      let next = 'checked';
      if (current === 'checked') next = 'mixed';
      else if (current === 'mixed') next = 'unchecked';
      else next = 'checked';

      if (knobCheckState) knobCheckState.value = next;
      updateCheckbox();
    });
    knobCheckState?.addEventListener('change', updateCheckbox);
    knobCheckDisabled?.addEventListener('change', updateCheckbox);
    knobCheckText?.addEventListener('input', updateCheckbox);
  }

  // ----------------------------------------------------
  // D. MacRadio Group Playground
  // ----------------------------------------------------
  const radioGroup = document.getElementById('knob-target-radio-group');
  const radioCode = document.getElementById('dynamic-code-radio');
  if (radioGroup) {
    const radioRows = radioGroup.querySelectorAll('.mac-radio-row');
    radioRows.forEach((row, idx) => {
      row.addEventListener('click', () => {
        if (row.classList.contains('disabled')) return;
        radioRows.forEach(r => r.classList.remove('selected'));
        row.classList.add('selected');
        const val = row.querySelector('.mac-radio-text')?.textContent || '';
        showToast(`Selected: ${val}`);

        if (radioCode) {
          radioCode.textContent = `MacRadio::new("radio-opt-${idx}", true)\n    .label("${val}")\n    .on_click(cx.listener(|this, _event, cx| {\n        this.selected_idx = ${idx};\n    }))`;
        }
      });
    });
  }

  // ----------------------------------------------------
  // E. MacSegmentedControl Playground & Dynamic Views
  // ----------------------------------------------------
  const segmentedControl = document.getElementById('demo-segmented-control');
  const segmentedViewContent = document.getElementById('segmented-view-content');
  const segmentedCode = document.getElementById('dynamic-code-segmented');

  if (segmentedControl) {
    const segments = segmentedControl.querySelectorAll('.mac-segment');
    segments.forEach((seg, idx) => {
      seg.addEventListener('click', () => {
        segments.forEach(s => s.classList.remove('active'));
        seg.classList.add('active');

        const viewType = seg.getAttribute('data-view');
        if (segmentedViewContent) {
          if (viewType === 'grid') {
            segmentedViewContent.innerHTML = `
              <div style="display: grid; grid-template-columns: repeat(4, 1fr); gap: 10px; width: 100%; max-width: 480px;">
                <div style="height: 70px; background: rgba(0,122,255,0.1); border-radius: 6px; display: flex; align-items: center; justify-content: center; font-size: 20px;">🌄</div>
                <div style="height: 70px; background: rgba(52,199,89,0.1); border-radius: 6px; display: flex; align-items: center; justify-content: center; font-size: 20px;">🏖️</div>
                <div style="height: 70px; background: rgba(255,149,0,0.1); border-radius: 6px; display: flex; align-items: center; justify-content: center; font-size: 20px;">🗼</div>
                <div style="height: 70px; background: rgba(175,82,222,0.1); border-radius: 6px; display: flex; align-items: center; justify-content: center; font-size: 20px;">🌸</div>
              </div>
            `;
          } else if (viewType === 'list') {
            segmentedViewContent.innerHTML = `
              <div style="width: 100%; max-width: 480px; display: flex; flex-direction: column; gap: 6px; font-size: 12px;">
                <div style="display: flex; justify-content: space-between; padding: 6px 10px; background: var(--apple-surface-hover); border-radius: 4px;">
                  <span>IMG_2026_Fuji.heic</span><span style="color: var(--apple-secondary-label);">4.8 MB</span>
                </div>
                <div style="display: flex; justify-content: space-between; padding: 6px 10px; background: var(--apple-surface-hover); border-radius: 4px;">
                  <span>IMG_2026_Tokyo.jpg</span><span style="color: var(--apple-secondary-label);">3.2 MB</span>
                </div>
                <div style="display: flex; justify-content: space-between; padding: 6px 10px; background: var(--apple-surface-hover); border-radius: 4px;">
                  <span>IMG_2026_Kyoto.raw</span><span style="color: var(--apple-secondary-label);">38.4 MB</span>
                </div>
              </div>
            `;
          } else {
            segmentedViewContent.innerHTML = `
              <div style="width: 100%; max-width: 480px; height: 100px; background: var(--apple-surface-hover); border-radius: 8px; border: 1px dashed var(--apple-border); display: flex; flex-direction: column; align-items: center; justify-content: center; gap: 6px;">
                <span style="font-size: 24px;">📍</span>
                <span style="font-size: 12px; color: var(--apple-secondary-label);">34 Photos in Tokyo, Japan</span>
              </div>
            `;
          }
        }

        if (segmentedCode) {
          segmentedCode.textContent = `MacSegmentedControl::new("view-switcher", ${idx})\n    .segment(MacSegment::new("grid", "Grid View"))\n    .segment(MacSegment::new("list", "List View"))\n    .segment(MacSegment::new("map", "Map View"))\n    .on_change(cx.listener(|this, idx, cx| {\n        this.view_idx = idx;\n    }))`;
        }
      });
    });
  }

  // ----------------------------------------------------
  // F. MacSearchField Playground & Real-Time Filter
  // ----------------------------------------------------
  const searchInput = document.getElementById('demo-search-input');
  const searchClearBtn = document.getElementById('demo-search-clear');
  const searchResultsContainer = document.getElementById('demo-search-results');
  const searchItems = [
    { title: 'Tokyo Skyline Night', tag: 'Landscape · Japan' },
    { title: 'Kyoto Bamboo Forest', tag: 'Nature · Japan' },
    { title: 'Sunset at Golden Gate', tag: 'Golden Hour · SF' },
    { title: 'Mount Fuji Sunrise', tag: 'Winter · Japan' },
  ];

  function renderSearchResults(filter = '') {
    if (!searchResultsContainer) return;
    const q = filter.toLowerCase().trim();
    const filtered = searchItems.filter(item => !q || item.title.toLowerCase().includes(q) || item.tag.toLowerCase().includes(q));

    if (filtered.length === 0) {
      searchResultsContainer.innerHTML = `<div style="font-size: 12px; color: var(--apple-tertiary-label); padding: 8px;">No matching photos found</div>`;
      return;
    }

    searchResultsContainer.innerHTML = filtered.map(item => `
      <div style="display: flex; align-items: center; justify-content: space-between; padding: 6px 10px; border-radius: 6px; background: var(--apple-surface-hover); font-size: 12px;">
        <span style="font-weight: 500;">${item.title}</span>
        <span style="font-size: 11px; color: var(--apple-secondary-label);">${item.tag}</span>
      </div>
    `).join('');
  }

  if (searchInput) {
    searchInput.addEventListener('input', (e) => {
      const val = e.target.value;
      if (searchClearBtn) searchClearBtn.style.display = val.length > 0 ? 'flex' : 'none';
      renderSearchResults(val);
    });

    if (searchClearBtn) {
      searchClearBtn.addEventListener('click', () => {
        searchInput.value = '';
        searchClearBtn.style.display = 'none';
        renderSearchResults('');
        searchInput.focus();
      });
    }
  }

  // ----------------------------------------------------
  // G. MacSlider Playground & Photo Exposure Simulation
  // ----------------------------------------------------
  const liveSlider = document.getElementById('demo-interactive-slider');
  const liveSliderVal = document.getElementById('demo-slider-val');
  const liveSliderImg = document.getElementById('demo-slider-img-target');

  if (liveSlider) {
    liveSlider.addEventListener('input', () => {
      const val = liveSlider.value;
      if (liveSliderVal) liveSliderVal.textContent = `${val}%`;
      if (liveSliderImg) {
        const brightness = 0.5 + (val / 100);
        liveSliderImg.style.filter = `brightness(${brightness})`;
      }
    });
  }

  // ----------------------------------------------------
  // H. MacColorWell Playground
  // ----------------------------------------------------
  const colorWellInput = document.getElementById('demo-color-input');
  const colorWellSwatch = document.getElementById('demo-color-swatch');
  const colorWellBadge = document.getElementById('demo-color-badge');
  const colorWellStageCard = document.getElementById('demo-color-stage-card');

  if (colorWellInput && colorWellSwatch) {
    colorWellInput.addEventListener('input', (e) => {
      const hex = e.target.value.toUpperCase();
      colorWellSwatch.style.backgroundColor = hex;
      if (colorWellBadge) colorWellBadge.textContent = hex;
      if (colorWellStageCard) colorWellStageCard.style.backgroundColor = `${hex}18`;
    });
  }

  // ----------------------------------------------------
  // I. MacPopupButton Popover Simulation
  // ----------------------------------------------------
  const popupBtn = document.getElementById('demo-popup-button');
  const popupMenu = document.getElementById('demo-popup-menu');
  const popupLabel = document.getElementById('demo-popup-label');

  if (popupBtn && popupMenu) {
    popupBtn.addEventListener('click', (e) => {
      e.stopPropagation();
      popupMenu.classList.toggle('open');
    });

    popupMenu.querySelectorAll('.mac-popover-item').forEach(item => {
      item.addEventListener('click', () => {
        popupMenu.querySelectorAll('.mac-popover-item').forEach(i => i.classList.remove('selected'));
        item.classList.add('selected');
        const text = item.textContent.replace(/^✓\s*/, '').trim();
        if (popupLabel) popupLabel.textContent = text;
        popupMenu.classList.remove('open');
        showToast(`Ratio changed: ${text}`);
      });
    });

    document.addEventListener('click', (e) => {
      if (!popupBtn.contains(e.target) && !popupMenu.contains(e.target)) {
        popupMenu.classList.remove('open');
      }
    });
  }

  // ----------------------------------------------------
  // J. MacDialog Modal Sheet Simulation
  // ----------------------------------------------------
  const triggerDialogBtn = document.getElementById('trigger-modal-sheet-btn');
  const modalOverlay = document.getElementById('demo-modal-sheet-overlay');
  const modalCancelBtn = document.getElementById('modal-sheet-cancel');
  const modalConfirmBtn = document.getElementById('modal-sheet-confirm');

  if (triggerDialogBtn && modalOverlay) {
    triggerDialogBtn.addEventListener('click', () => {
      modalOverlay.classList.add('open');
    });

    modalCancelBtn?.addEventListener('click', () => {
      modalOverlay.classList.remove('open');
    });

    modalConfirmBtn?.addEventListener('click', () => {
      modalOverlay.classList.remove('open');
      showToast('Export confirmed');
    });

    modalOverlay.addEventListener('click', (e) => {
      if (e.target === modalOverlay) modalOverlay.classList.remove('open');
    });
  }

  // ----------------------------------------------------
  // K. MacSettingsGroup Interactive Dependencies
  // ----------------------------------------------------
  const syncMasterSwitch = document.getElementById('settings-sync-master');
  const syncSubRow1 = document.getElementById('settings-subrow-1');
  const syncSubRow2 = document.getElementById('settings-subrow-2');

  if (syncMasterSwitch) {
    syncMasterSwitch.addEventListener('click', () => {
      syncMasterSwitch.classList.toggle('on');
      const isMasterOn = syncMasterSwitch.classList.contains('on');

      if (syncSubRow1) {
        syncSubRow1.style.opacity = isMasterOn ? '1' : '0.4';
        syncSubRow1.style.pointerEvents = isMasterOn ? 'auto' : 'none';
      }
      if (syncSubRow2) {
        syncSubRow2.style.opacity = isMasterOn ? '1' : '0.4';
        syncSubRow2.style.pointerEvents = isMasterOn ? 'auto' : 'none';
      }
      showToast(isMasterOn ? 'Automatic sync enabled' : 'Automatic sync paused');
    });
  }

  // ----------------------------------------------------
  // L. MacSidebar Gallery Switching
  // ----------------------------------------------------
  const sidebarRows = document.querySelectorAll('.mac-sidebar-preview .mac-sidebar-row');
  const sidebarGalleryTitle = document.getElementById('sidebar-gallery-title');
  const sidebarGalleryCount = document.getElementById('sidebar-gallery-count');

  if (sidebarRows.length > 0) {
    sidebarRows.forEach(row => {
      row.addEventListener('click', () => {
        sidebarRows.forEach(r => r.classList.remove('active'));
        row.classList.add('active');
        const title = row.querySelector('.sidebar-row-title')?.textContent || 'Photos';
        const badge = row.querySelector('.mac-badge')?.textContent || '0';

        if (sidebarGalleryTitle) sidebarGalleryTitle.textContent = title;
        if (sidebarGalleryCount) sidebarGalleryCount.textContent = `${badge} photos`;
        showToast(`Viewing: ${title}`);
      });
    });
  }

  // ----------------------------------------------------
  // M. MacComboBox
  // ----------------------------------------------------
  const comboTrigger = document.getElementById('knob-combobox-trigger');
  const comboDropdown = document.getElementById('knob-combobox-dropdown');
  const comboValue = document.getElementById('knob-combobox-value');
  const comboSelect = document.getElementById('knob-combobox-select');
  const comboDisabled = document.getElementById('knob-combobox-disabled');
  const comboTarget = document.getElementById('knob-target-combobox');
  const comboCode = document.getElementById('dynamic-code-combobox');

  if (comboTrigger && comboDropdown) {
    comboTrigger.addEventListener('click', (e) => {
      e.stopPropagation();
      if (comboTarget?.classList.contains('disabled')) return;
      comboDropdown.classList.toggle('open');
    });

    document.addEventListener('click', (e) => {
      if (!comboTarget?.contains(e.target)) {
        comboDropdown.classList.remove('open');
      }
    });

    comboDropdown.querySelectorAll('.mac-combobox-option').forEach(opt => {
      opt.addEventListener('click', () => {
        const val = opt.getAttribute('data-value') || opt.textContent.trim();
        comboDropdown.querySelectorAll('.mac-combobox-option').forEach(o => o.classList.remove('selected'));
        opt.classList.add('selected');
        if (comboValue) comboValue.textContent = val;
        if (comboSelect) comboSelect.value = val;
        comboDropdown.classList.remove('open');
        showToast(`Selected: ${val}`);
        if (comboCode) {
          comboCode.textContent = `MacComboBox::new("raw-codec")\n    .placeholder("Select RAW Preset...")\n    .option("${val}")\n    .selected_index(Some(0))\n    .on_select(cx.listener(|this, idx, text, cx| {\n        this.active_profile = text.to_string();\n    }))`;
        }
      });
    });

    comboSelect?.addEventListener('change', () => {
      const val = comboSelect.value;
      if (comboValue) comboValue.textContent = val;
      comboDropdown.querySelectorAll('.mac-combobox-option').forEach(o => {
        if (o.getAttribute('data-value') === val) o.classList.add('selected');
        else o.classList.remove('selected');
      });
    });

    comboDisabled?.addEventListener('change', () => {
      if (comboDisabled.checked) {
        comboTarget?.classList.add('disabled');
        comboDropdown.classList.remove('open');
      } else {
        comboTarget?.classList.remove('disabled');
      }
    });
  }

  // ----------------------------------------------------
  // N. MacDigitEntry
  // ----------------------------------------------------
  const digitValInput = document.getElementById('knob-digit-val');
  const digitSecureCheck = document.getElementById('knob-digit-secure');
  const digitCountSelect = document.getElementById('knob-digit-count');
  const digitContainer = document.getElementById('knob-target-digit-entry');
  const digitCode = document.getElementById('dynamic-code-digit');

  function updateDigitEntry() {
    if (!digitContainer) return;
    const count = parseInt(digitCountSelect?.value || '6', 10);
    const val = digitValInput?.value || '';
    const isSecure = digitSecureCheck?.checked || false;

    let html = '';
    for (let i = 0; i < count; i++) {
      const char = val[i] || '';
      const displayChar = isSecure && char ? '●' : char;
      const isActive = i === val.length && i < count;
      html += `<div class="mac-digit-cell ${isActive ? 'active' : ''}">${isActive ? '|' : displayChar}</div>`;
    }
    digitContainer.innerHTML = html;

    if (digitCode) {
      digitCode.textContent = `MacDigitEntry::new("2fa-code", ${count})\n    .value("${val}")\n    .secure(${isSecure})\n    .on_complete(cx.listener(|this, code, cx| {\n        this.verify_token(code);\n    }))`;
    }
  }

  digitValInput?.addEventListener('input', updateDigitEntry);
  digitSecureCheck?.addEventListener('change', updateDigitEntry);
  digitCountSelect?.addEventListener('change', updateDigitEntry);

  // ----------------------------------------------------
  // O. MacImageWell
  // ----------------------------------------------------
  const wellLoadedCheck = document.getElementById('knob-well-loaded');
  const wellCaptionInput = document.getElementById('knob-well-caption');
  const wellBox = document.getElementById('knob-imagewell-box');
  const wellCap = document.getElementById('knob-imagewell-cap');
  const wellCode = document.getElementById('dynamic-code-well');

  function updateImageWell() {
    if (!wellBox) return;
    const isLoaded = wellLoadedCheck?.checked || false;
    const caption = wellCaptionInput?.value || 'DSC04921.ARW (34.2 MB)';

    if (isLoaded) {
      wellBox.innerHTML = `
        <svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="var(--apple-accent)" stroke-width="1.8">
          <rect x="3" y="3" width="18" height="18" rx="2" ry="2"></rect>
          <circle cx="8.5" cy="8.5" r="1.5"></circle>
          <polyline points="21 15 16 10 5 21"></polyline>
        </svg>
        <span style="font-size: 10px; color: var(--apple-secondary-label); margin-top: 6px;">RAW Preview Loaded</span>
      `;
    } else {
      wellBox.innerHTML = `
        <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="var(--apple-secondary-label)" stroke-width="2">
          <line x1="12" y1="5" x2="12" y2="19"></line>
          <line x1="5" y1="12" x2="19" y2="12"></line>
        </svg>
        <span style="font-size: 10px; color: var(--apple-secondary-label); margin-top: 6px;">Drop RAW Photo Here</span>
      `;
    }
    if (wellCap) wellCap.textContent = isLoaded ? caption : 'No file selected';

    if (wellCode) {
      wellCode.textContent = `MacImageWell::new("photo-well")\n    .size(140.0, 140.0)\n    .has_image(${isLoaded})\n    .caption("${caption}")\n    .on_click(cx.listener(|this, _event, cx| {\n        this.open_finder_dialog();\n    }))`;
    }
  }

  wellLoadedCheck?.addEventListener('change', updateImageWell);
  wellCaptionInput?.addEventListener('input', updateImageWell);
  wellBox?.addEventListener('click', () => {
    if (wellLoadedCheck) {
      wellLoadedCheck.checked = !wellLoadedCheck.checked;
      updateImageWell();
      showToast(wellLoadedCheck.checked ? 'Image loaded' : 'Image cleared');
    }
  });

  // ----------------------------------------------------
  // P. MacDatePicker
  // ----------------------------------------------------
  const dateInput = document.getElementById('knob-date-input');
  const dateShowTime = document.getElementById('knob-date-showtime');
  const dateDisp = document.getElementById('knob-date-val');
  const timeSeg = document.getElementById('knob-time-segment');
  const dateCode = document.getElementById('dynamic-code-date');

  function updateDatePicker() {
    const val = dateInput?.value || 'Oct 21, 2026';
    const showTime = dateShowTime?.checked || false;
    if (dateDisp) dateDisp.textContent = val;
    if (timeSeg) timeSeg.style.display = showTime ? 'flex' : 'none';

    if (dateCode) {
      dateCode.textContent = `MacDatePicker::new("date-input")\n    .date(2026, 10, 21)\n    .time(12, 30)\n    .show_time(${showTime})\n    .on_change(cx.listener(|this, y, m, d, cx| {\n        this.update_timestamp(y, m, d);\n    }))`;
    }
  }

  dateInput?.addEventListener('input', updateDatePicker);
  dateShowTime?.addEventListener('change', updateDatePicker);

  // ----------------------------------------------------
  // Q. MacContextMenu & Menu Bar
  // ----------------------------------------------------
  const menuBarTitleInput = document.getElementById('knob-menubar-title');
  const menuBarAppDisp = document.getElementById('knob-menubar-appname');
  const menuBarCode = document.getElementById('dynamic-code-menubar');

  menuBarTitleInput?.addEventListener('input', () => {
    const title = menuBarTitleInput.value || 'Loupe';
    if (menuBarAppDisp) menuBarAppDisp.textContent = title;
    if (menuBarCode) {
      menuBarCode.textContent = `MacMenuBar::new("sys-menubar", "${title}")\n    .menus(vec!["File", "Edit", "View", "Window", "Help"])\n    .active_menu(Some(0))`;
    }
  });

  document.querySelectorAll('.mac-menubar-item').forEach(item => {
    item.addEventListener('click', () => {
      document.querySelectorAll('.mac-menubar-item').forEach(i => i.classList.remove('active'));
      item.classList.add('active');
      showToast(`Menu: ${item.textContent.trim()}`);
    });
  });

  // ----------------------------------------------------
  // R. MacPathControl
  // ----------------------------------------------------
  const pathLeafInput = document.getElementById('knob-path-leaf');
  const pathCurrentCrumb = document.querySelector('.mac-path-crumb.current');
  const pathCode = document.getElementById('dynamic-code-path');

  pathLeafInput?.addEventListener('input', () => {
    const leaf = pathLeafInput.value || 'photo-engine';
    if (pathCurrentCrumb) pathCurrentCrumb.textContent = `📷 ${leaf}`;
    if (pathCode) {
      pathCode.textContent = `MacPathControl::new("finder-path")\n    .item("Macintosh HD")\n    .item("Users")\n    .item("mayank")\n    .item("${leaf}")`;
    }
  });

  document.querySelectorAll('.mac-path-crumb').forEach(crumb => {
    crumb.addEventListener('click', () => {
      showToast(`Navigated to: ${crumb.textContent.trim()}`);
    });
  });

  // ----------------------------------------------------
  // S. MacTabBar
  // ----------------------------------------------------
  const tabSelect = document.getElementById('knob-tab-active-select');
  const tabItems = document.querySelectorAll('.mac-tab-demo .mac-tab-item');
  const tabCode = document.getElementById('dynamic-code-tabs');

  tabItems.forEach(tab => {
    tab.addEventListener('click', () => {
      tabItems.forEach(t => t.classList.remove('active'));
      tab.classList.add('active');
      const tabId = tab.getAttribute('data-tab');
      if (tabSelect && tabId) tabSelect.value = tabId;
      showToast(`Switched tab: ${tab.querySelector('span')?.textContent || tabId}`);
    });
  });

  tabSelect?.addEventListener('change', () => {
    const val = tabSelect.value;
    tabItems.forEach(t => {
      if (t.getAttribute('data-tab') === val) t.classList.add('active');
      else t.classList.remove('active');
    });
  });

  // ----------------------------------------------------
  // T. MacTokenField
  // ----------------------------------------------------
  const tokenContainer = document.getElementById('knob-target-tokenfield');
  const tokenAddInput = document.getElementById('knob-token-add-input');
  const tokenCode = document.getElementById('dynamic-code-token');

  function bindTokenCloseButtons() {
    tokenContainer?.querySelectorAll('.mac-token-chip .close-btn').forEach(btn => {
      btn.onclick = (e) => {
        e.stopPropagation();
        const chip = btn.closest('.mac-token-chip');
        const chipName = chip?.querySelector('span')?.textContent || 'tag';
        chip?.remove();
        showToast(`Removed tag: ${chipName}`);
      };
    });
  }
  bindTokenCloseButtons();

  tokenAddInput?.addEventListener('keydown', (e) => {
    if (e.key === 'Enter') {
      const val = tokenAddInput.value.trim();
      if (val && tokenContainer) {
        const newChip = document.createElement('div');
        newChip.className = 'mac-token-chip';
        newChip.innerHTML = `<span>${val}</span><span class="close-btn">×</span>`;
        tokenContainer.insertBefore(newChip, tokenContainer.lastElementChild);
        tokenAddInput.value = '';
        bindTokenCloseButtons();
        showToast(`Added tag: ${val}`);
      }
    }
  });

  // ----------------------------------------------------
  // U. MacGauge
  // ----------------------------------------------------
  const gaugeSlider = document.getElementById('knob-gauge-slider');
  const gaugeValDisp = document.getElementById('knob-gauge-val-disp');
  const gaugeFill = document.getElementById('knob-target-gauge-fill');
  const gaugeCode = document.getElementById('dynamic-code-gauge');

  gaugeSlider?.addEventListener('input', () => {
    const val = parseInt(gaugeSlider.value, 10);
    if (gaugeValDisp) gaugeValDisp.textContent = `${val}%`;
    if (gaugeFill) {
      gaugeFill.style.width = `${val}%`;
      if (val >= 90) {
        gaugeFill.style.backgroundColor = 'var(--apple-system-red)';
      } else if (val >= 70) {
        gaugeFill.style.backgroundColor = 'var(--apple-system-orange)';
      } else {
        gaugeFill.style.backgroundColor = 'var(--apple-accent)';
      }
    }
    if (gaugeCode) {
      gaugeCode.textContent = `MacGauge::new("disk-gauge", ${(val / 100).toFixed(2)})\n    .label("Scratch Disk Cache")\n    .thresholds(0.70, 0.90)`;
    }
  });

  // ----------------------------------------------------
  // V. MacRatingIndicator
  // ----------------------------------------------------
  const ratingStars = document.querySelectorAll('.mac-rating-star');
  const ratingLabel = document.getElementById('knob-rating-label');
  const ratingSelect = document.getElementById('knob-rating-select');
  const ratingCode = document.getElementById('dynamic-code-rating');

  function setRating(score) {
    ratingStars.forEach((star, idx) => {
      if (idx < score) star.classList.add('filled');
      else star.classList.remove('filled');
    });
    if (ratingLabel) ratingLabel.textContent = `${score} of 5 Stars`;
    if (ratingSelect) ratingSelect.value = String(score);
    if (ratingCode) {
      ratingCode.textContent = `MacRatingIndicator::new("photo-rating", ${score})\n    .max_rating(5)\n    .on_rate(cx.listener(|this, stars, cx| {\n        this.photo_stars = stars;\n    }))`;
    }
  }

  ratingStars.forEach((star, idx) => {
    star.addEventListener('click', () => {
      const score = idx + 1;
      setRating(score);
      showToast(`Rated: ${score} Stars`);
    });
  });

  ratingSelect?.addEventListener('change', () => {
    setRating(parseInt(ratingSelect.value, 10));
  });

  // ----------------------------------------------------
  // W. MacSplitView
  // ----------------------------------------------------
  const splitWidthSlider = document.getElementById('knob-split-width');
  const splitFirstPane = document.querySelector('.mac-split-demo > div:first-child');
  const splitCode = document.getElementById('dynamic-code-split');

  splitWidthSlider?.addEventListener('input', () => {
    const w = splitWidthSlider.value;
    if (splitFirstPane) splitFirstPane.style.width = `${w}px`;
    if (splitCode) {
      splitCode.textContent = `MacSplitView::new("main-split")\n    .leading_width(${w}.0)\n    .leading(MacSidebar::new("sidebar"))\n    .trailing(photo_canvas)`;
    }
  });

  // ----------------------------------------------------
  // X. MacOutlineView
  // ----------------------------------------------------
  const outlineRows = document.querySelectorAll('.mac-outline-row');
  outlineRows.forEach(row => {
    row.addEventListener('click', () => {
      outlineRows.forEach(r => r.classList.remove('selected'));
      row.classList.add('selected');
      const name = row.querySelector('span:last-child')?.textContent || 'item';
      showToast(`Selected outline: ${name}`);
    });
  });

  // ----------------------------------------------------
  // Y. MacChart
  // ----------------------------------------------------
  const chartPeakSlider = document.getElementById('knob-chart-peak');
  const chartWedBar = document.querySelector('.mac-chart-bar-item:nth-child(3) .mac-chart-bar-rect');
  const chartCode = document.getElementById('dynamic-code-chart');

  chartPeakSlider?.addEventListener('input', () => {
    const h = chartPeakSlider.value;
    if (chartWedBar) chartWedBar.style.height = `${h}px`;
    if (chartCode) {
      chartCode.textContent = `MacChart::new("ingest-chart")\n    .title("Weekly Photo Ingest Volume")\n    .point("Mon", 0.40)\n    .point("Tue", 0.70)\n    .point_colored("Wed", ${(h / 100).toFixed(2)}, MacAccentColor::Green)\n    .point("Thu", 0.50)\n    .point("Fri", 0.85)`;
    }
  });

  // ----------------------------------------------------
  // Z. MacPageControl
  // ----------------------------------------------------
  const pageDots = document.querySelectorAll('.mac-page-dot');
  const pageStatus = document.getElementById('knob-page-status');
  const pageCurrInput = document.getElementById('knob-page-curr');
  const pageCode = document.getElementById('dynamic-code-pagecontrol');

  function setPage(page) {
    pageDots.forEach((dot, idx) => {
      if (idx === page) dot.classList.add('active');
      else dot.classList.remove('active');
    });
    if (pageStatus) pageStatus.textContent = `Page ${page + 1} of ${pageDots.length}`;
    if (pageCurrInput) pageCurrInput.value = page + 1;
    if (pageCode) {
      pageCode.textContent = `MacPageControl::new("gallery-pager", ${pageDots.length})\n    .current_page(${page})\n    .on_change(cx.listener(|this, page, cx| {\n        this.active_slide = page;\n    }))`;
    }
  }

  pageDots.forEach((dot, idx) => {
    dot.addEventListener('click', () => {
      setPage(idx);
      showToast(`Page ${idx + 1}`);
    });
  });

  pageCurrInput?.addEventListener('input', () => {
    const p = Math.max(1, Math.min(pageDots.length, parseInt(pageCurrInput.value || '1', 10))) - 1;
    setPage(p);
  });
}

/* ==========================================================================
   6. Copy Code to Clipboard & Toast
   ========================================================================== */

function initCopyCodeButtons() {
  document.querySelectorAll('.copy-btn').forEach(btn => {
    btn.addEventListener('click', async () => {
      const codeBox = btn.closest('.code-box') || btn.closest('.playground-code-view');
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
   7. Keyboard Shortcuts
   ========================================================================== */

function initKeyboardShortcuts() {
  window.addEventListener('keydown', (e) => {
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

/* ==========================================================================
   8. Apple HIG Materials & Vibrancy Interactive Playground
   ========================================================================== */

function initMaterialsPlayground() {
  const matStageContainer = document.getElementById('mat-stage-container');
  const matCard = document.getElementById('knob-target-mat-card');
  const matCardTitle = document.getElementById('mat-card-title');
  const matHudText = document.getElementById('mat-hud-text');
  const matCode = document.getElementById('dynamic-code-materials');

  const knobMatThickness = document.getElementById('knob-mat-thickness');
  const knobMatRole = document.getElementById('knob-mat-role');
  const knobMatBlend = document.getElementById('knob-mat-blend');
  const knobMatState = document.getElementById('knob-mat-state');
  const knobMatReduceTransparency = document.getElementById('knob-mat-reduce-transparency');
  const knobMatRim = document.getElementById('knob-mat-rim');

  const matSwitch = document.getElementById('mat-sample-switch');
  const matSegmentedBtns = document.querySelectorAll('#mat-sample-segmented .mac-segment-btn');
  const matDensitySlider = document.getElementById('mat-density-slider');
  const matDensityLabel = document.getElementById('mat-density-label');
  const wallpaperThumbs = document.querySelectorAll('.wallpaper-thumb');

  const matViewGrid = document.getElementById('mat-view-grid');
  const matViewList = document.getElementById('mat-view-list');
  const matViewLayers = document.getElementById('mat-view-layers');
  const matActiveLayerName = document.getElementById('mat-active-layer-name');
  const matActiveLayerBlur = document.getElementById('mat-active-layer-blur');
  const matBtnDismiss = document.getElementById('mat-btn-dismiss');
  const matBtnApply = document.getElementById('mat-btn-apply');

  if (!matCard) return;

  let currentViewMode = 'grid'; // 'grid' | 'list' | 'layers'

  const THICKNESS_DATA = {
    'ultra-thin': { blur: 16, lightOpacity: 0.35, darkOpacity: 0.40, name: 'UltraThin' },
    'thin':       { blur: 24, lightOpacity: 0.50, darkOpacity: 0.55, name: 'Thin' },
    'regular':    { blur: 32, lightOpacity: 0.70, darkOpacity: 0.70, name: 'Regular' },
    'thick':      { blur: 44, lightOpacity: 0.85, darkOpacity: 0.82, name: 'Thick' },
    'ultra-thick':{ blur: 60, lightOpacity: 0.95, darkOpacity: 0.92, name: 'UltraThick' },
  };

  const ROLE_DATA = {
    'titlebar': { thickness: 'regular', blend: 'within-window', title: 'Titlebar Surface' },
    'selection': { thickness: 'regular', blend: 'within-window', title: 'Selection Highlight' },
    'menu': { thickness: 'regular', blend: 'within-window', title: 'Menu Surface' },
    'popover': { thickness: 'thick', blend: 'within-window', title: 'Popover Surface' },
    'sidebar': { thickness: 'thin', blend: 'behind-window', title: 'Sidebar Source List' },
    'header': { thickness: 'regular', blend: 'within-window', title: 'Header View' },
    'sheet': { thickness: 'thick', blend: 'within-window', title: 'Modal Sheet' },
    'window-bg': { thickness: 'ultra-thick', blend: 'behind-window', title: 'Window Background Chrome' },
    'hud': { thickness: 'thick', blend: 'within-window', title: 'Heads-Up Display (HUD)' },
    'fullscreen': { thickness: 'ultra-thick', blend: 'within-window', title: 'Full Screen Backdrop' },
    'tooltip': { thickness: 'regular', blend: 'within-window', title: 'Tool Tip Bubble' },
  };

  function updateViewMode(mode) {
    currentViewMode = mode;
    if (matViewGrid) matViewGrid.style.display = mode === 'grid' ? 'flex' : 'none';
    if (matViewList) matViewList.style.display = mode === 'list' ? 'flex' : 'none';
    if (matViewLayers) matViewLayers.style.display = mode === 'layers' ? 'flex' : 'none';
    updateMaterials();
  }

  function updateMaterials() {
    const isDark = document.documentElement.getAttribute('data-theme') === 'dark';
    const role = knobMatRole?.value || 'custom';
    const isRolePreset = role !== 'custom' && ROLE_DATA[role];

    if (isRolePreset) {
      if (knobMatThickness) knobMatThickness.value = ROLE_DATA[role].thickness;
      if (knobMatBlend) knobMatBlend.value = ROLE_DATA[role].blend;
      if (matCardTitle) {
        matCardTitle.innerHTML = `
          <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <polygon points="12 2 2 7 12 12 22 7 12 2"></polygon>
            <polyline points="2 17 12 22 22 17"></polyline>
            <polyline points="2 12 12 17 22 12"></polyline>
          </svg>
          <span>${ROLE_DATA[role].title}</span>
        `;
      }
    } else {
      if (matCardTitle) {
        matCardTitle.innerHTML = `
          <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <polygon points="12 2 2 7 12 12 22 7 12 2"></polygon>
            <polyline points="2 17 12 22 22 17"></polyline>
            <polyline points="2 12 12 17 22 12"></polyline>
          </svg>
          <span>Inspector & Materials Surface</span>
        `;
      }
    }

    const thicknessKey = knobMatThickness?.value || 'regular';
    const thickness = THICKNESS_DATA[thicknessKey] || THICKNESS_DATA.regular;
    const blendMode = knobMatBlend?.value || 'within-window';
    const isActive = knobMatState?.value !== 'inactive';
    const reduceTransparency = knobMatReduceTransparency?.checked || false;
    const rimHighlight = knobMatRim?.checked !== false;
    const isTinted = matSwitch?.classList.contains('on');
    const densityVal = parseInt(matDensitySlider?.value || '65', 10);

    let bg = '';
    let filter = '';
    let borderColor = '';
    let sat = isActive ? '180%' : '100%';

    // Scale blur with density slider
    const densityFactor = densityVal / 65.0;
    const blurRadius = Math.round(thickness.blur * densityFactor);

    if (reduceTransparency) {
      filter = 'none';
      bg = isDark ? '#242426' : '#F2F2F7';
      borderColor = isDark ? 'rgba(255, 255, 255, 0.15)' : 'rgba(0, 0, 0, 0.15)';
    } else if (role === 'hud') {
      filter = `blur(${Math.max(12, blurRadius)}px) saturate(${sat})`;
      bg = 'rgba(28, 28, 32, 0.88)';
      borderColor = rimHighlight ? 'rgba(255, 255, 255, 0.22)' : 'transparent';
    } else if (role === 'selection') {
      filter = `blur(${blurRadius}px) saturate(${sat})`;
      bg = isDark ? 'rgba(10, 132, 255, 0.25)' : 'rgba(0, 122, 255, 0.18)';
      borderColor = rimHighlight ? (isDark ? 'rgba(10, 132, 255, 0.45)' : 'rgba(0, 122, 255, 0.35)') : 'transparent';
    } else {
      filter = `blur(${blurRadius}px) saturate(${sat})`;
      const baseAlpha = isDark ? thickness.darkOpacity : thickness.lightOpacity;
      const alpha = isActive ? baseAlpha : Math.min(1.0, baseAlpha + 0.08);

      if (isTinted) {
        // macOS Desktop Tinting subtly blends accent hue into the frosted material
        bg = isDark 
          ? `color-mix(in srgb, rgba(34, 34, 38, ${alpha}) 88%, var(--apple-accent) 12%)`
          : `color-mix(in srgb, rgba(255, 255, 255, ${alpha}) 90%, var(--apple-accent) 10%)`;
      } else {
        bg = isDark ? `rgba(34, 34, 38, ${alpha})` : `rgba(255, 255, 255, ${alpha})`;
      }

      borderColor = rimHighlight ? (isDark ? 'rgba(255, 255, 255, 0.20)' : 'rgba(0, 0, 0, 0.10)') : 'transparent';
    }

    matCard.style.backdropFilter = filter;
    matCard.style.webkitBackdropFilter = filter;
    matCard.style.backgroundColor = bg;
    matCard.style.border = rimHighlight ? `1px solid ${borderColor}` : '1px solid transparent';

    // Update Layers inspector text
    if (matActiveLayerName) {
      matActiveLayerName.textContent = `MacMaterialSurface (${thickness.name} Glass)`;
    }
    if (matActiveLayerBlur) {
      matActiveLayerBlur.textContent = `${reduceTransparency ? '0' : blurRadius}px Blur`;
    }

    if (matHudText) {
      const curOpacity = Math.round((isDark ? thickness.darkOpacity : thickness.lightOpacity) * 100);
      matHudText.textContent = `Tier: ${thickness.name} | Blur: ${reduceTransparency ? 0 : blurRadius}px | Opacity: ${curOpacity}% | Mode: ${currentViewMode.toUpperCase()}`;
    }

    if (matCode) {
      let code = `// Apple Human Interface Guidelines - Materials & Vibrancy in GPUI\n`;
      if (role !== 'custom') {
        const rolePascal = role === 'window-bg' ? 'WindowBackground' : (role === 'hud' ? 'HudWindow' : (role === 'fullscreen' ? 'FullScreenUI' : (role === 'header' ? 'HeaderView' : (role === 'tooltip' ? 'ToolTip' : role.charAt(0).toUpperCase() + role.slice(1)))));
        code += `let material = MacMaterial::from_role(MacMaterialRole::${rolePascal})\n`;
      } else {
        code += `let material = MacMaterial::new(MacMaterialThickness::${thickness.name})\n`;
      }
      code += `    .is_dark(${isDark})\n`;
      code += `    .is_active(${isActive})\n`;
      if (reduceTransparency) {
        code += `    .reduce_transparency(true)\n`;
      }
      if (!rimHighlight) {
        code += `    .rim_highlight(false)\n`;
      }
      if (blendMode === 'behind-window') {
        code += `    .blending_mode(MacBlendingMode::BehindWindow)\n`;
      }
      code += `;\n\n`;

      if (currentViewMode === 'grid') {
        code += `// View Mode: Grid Layout inside Frosted Material\n`;
        code += `MacMaterialSurface::new("photo_grid_surface")\n`;
        code += `    .material(material)\n`;
        code += `    .corner_radius(12.0)\n`;
        code += `    .child(\n`;
        code += `        div()\n`;
        code += `            .p_4()\n`;
        code += `            .flex()\n`;
        code += `            .gap_2()\n`;
        code += `            .child(MacImageView::new("img1").size(100.0, 70.0).caption("Yosemite.raw"))\n`;
        code += `            .child(MacImageView::new("img2").size(100.0, 70.0).caption("BigSur.heic"))\n`;
        code += `            .child(MacImageView::new("img3").size(100.0, 70.0).caption("Sonoma.dng"))\n`;
        code += `    );`;
      } else if (currentViewMode === 'list') {
        code += `// View Mode: Table / List View inside Frosted Material\n`;
        code += `MacMaterialSurface::new("photo_list_surface")\n`;
        code += `    .material(material)\n`;
        code += `    .corner_radius(12.0)\n`;
        code += `    .child(\n`;
        code += `        MacTableView::new("materials_table")\n`;
        code += `            .column("File Name", 140.0)\n`;
        code += `            .column("Size", 70.0)\n`;
        code += `            .column("Format", 80.0)\n`;
        code += `            .row(vec!["IMG_0412.CR3", "32.8 MB", "Canon RAW"])\n`;
        code += `            .row(vec!["DSCF8921.RAF", "28.4 MB", "Fuji Film"])\n`;
        code += `            .selected_row(Some(0))\n`;
        code += `    );`;
      } else {
        code += `// View Mode: Z-Index Layer Stack Architecture\n`;
        code += `div()\n`;
        code += `    .relative()\n`;
        code += `    // Layer 1: BehindWindow Desktop Wallpaper\n`;
        code += `    .child(desktop_wallpaper_layer())\n`;
        code += `    // Layer 2: MacMaterialSurface translucent blur\n`;
        code += `    .child(MacMaterialSurface::new("frost_layer").material(material))\n`;
        code += `    // Layer 3: Vibrant foreground UI\n`;
        code += `    .child(vibrant_controls_layer());`;
      }

      matCode.textContent = code;
    }
  }

  knobMatThickness?.addEventListener('change', () => {
    if (knobMatRole) knobMatRole.value = 'custom';
    updateMaterials();
  });
  knobMatRole?.addEventListener('change', updateMaterials);
  knobMatBlend?.addEventListener('change', updateMaterials);
  knobMatState?.addEventListener('change', updateMaterials);
  knobMatReduceTransparency?.addEventListener('change', updateMaterials);
  knobMatRim?.addEventListener('change', updateMaterials);

  if (matSwitch) {
    matSwitch.addEventListener('click', () => {
      matSwitch.classList.toggle('on');
      updateMaterials();
    });
  }

  matSegmentedBtns.forEach(btn => {
    btn.addEventListener('click', () => {
      matSegmentedBtns.forEach(b => b.classList.remove('active'));
      btn.classList.add('active');
      const mode = btn.getAttribute('data-mat') || 'grid';
      updateViewMode(mode);
    });
  });

  // Interactive selection in list view
  const matListRows = document.querySelectorAll('.mat-list-row');
  matListRows.forEach(row => {
    row.addEventListener('click', () => {
      matListRows.forEach(r => r.classList.remove('selected'));
      row.classList.add('selected');
    });
  });

  if (matDensitySlider && matDensityLabel) {
    matDensitySlider.addEventListener('input', (e) => {
      matDensityLabel.textContent = `${e.target.value}%`;
      updateMaterials();
    });
  }

  if (matBtnDismiss) {
    matBtnDismiss.addEventListener('click', () => {
      if (knobMatThickness) knobMatThickness.value = 'regular';
      if (knobMatRole) knobMatRole.value = 'custom';
      if (knobMatBlend) knobMatBlend.value = 'within-window';
      if (knobMatState) knobMatState.value = 'active';
      if (knobMatReduceTransparency) knobMatReduceTransparency.checked = false;
      if (knobMatRim) knobMatRim.checked = true;
      if (matDensitySlider) {
        matDensitySlider.value = '65';
        if (matDensityLabel) matDensityLabel.textContent = '65%';
      }
      updateMaterials();
      showToast('Material reset to defaults');
    });
  }

  if (matBtnApply) {
    matBtnApply.addEventListener('click', () => {
      const thicknessKey = knobMatThickness?.value || 'regular';
      const thickness = THICKNESS_DATA[thicknessKey] || THICKNESS_DATA.regular;
      showToast(`Applied ${thickness.name} (${thickness.blur}px blur) to surface`);
    });
  }

  wallpaperThumbs.forEach(thumb => {
    thumb.addEventListener('click', () => {
      wallpaperThumbs.forEach(t => t.classList.remove('active'));
      thumb.classList.add('active');
      const wp = thumb.getAttribute('data-wp') || 'sonoma';
      if (matStageContainer) {
        matStageContainer.className = `material-stage-container wp-${wp}`;
      }
    });
  });

  const themeObserver = new MutationObserver(() => {
    updateMaterials();
  });
  themeObserver.observe(document.documentElement, { attributes: true, attributeFilter: ['data-theme'] });

  updateMaterials();
}
