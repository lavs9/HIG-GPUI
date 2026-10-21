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
  const mainContent = document.querySelector('.main-content');

  let isClickScrolling = false;
  let clickScrollTimeout = null;

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
          isClickScrolling = true;
          navItems.forEach(i => i.classList.remove('active'));
          link.classList.add('active');
          link.scrollIntoView({ block: 'nearest', behavior: 'smooth' });
          target.scrollIntoView({ behavior: 'smooth', block: 'start' });
          history.pushState(null, null, href);

          clearTimeout(clickScrollTimeout);
          clickScrollTimeout = setTimeout(() => {
            isClickScrolling = false;
          }, 800);
        }
      }
    });
  });

  const sections = document.querySelectorAll('section[id]');
  if ('IntersectionObserver' in window && sections.length > 0 && mainContent) {
    const observer = new IntersectionObserver((entries) => {
      if (isClickScrolling) return;

      const visible = entries.filter(e => e.isIntersecting);
      if (visible.length === 0) return;

      visible.sort((a, b) => a.boundingClientRect.top - b.boundingClientRect.top);
      const topEntry = visible[0];
      const id = topEntry.target.getAttribute('id');

      navItems.forEach(item => {
        if (item.getAttribute('href') === `#${id}`) {
          item.classList.add('active');
          item.scrollIntoView({ block: 'nearest', behavior: 'smooth' });
        } else {
          item.classList.remove('active');
        }
      });
    }, {
      root: mainContent,
      rootMargin: '-10% 0px -65% 0px',
      threshold: [0, 0.15, 0.5]
    });

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
  // 1. MacSwitch Playground
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
      let c = `MacSwitch::new("my-switch", ${isOn})\n`;
      if (isMini) c += `    .size(MacSwitchSize::Mini)\n`;
      if (isDisabled) c += `    .disabled(true)\n`;
      c += `    .on_click(cx.listener(|this, _event, cx| {\n        this.enabled = !this.enabled;\n        cx.notify();\n    }))`;
      switchCode.textContent = c;
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
  // 2. MacButton Playground
  // ----------------------------------------------------
  const buttonTarget = document.getElementById('knob-target-button');
  const buttonCode = document.getElementById('dynamic-code-button');
  const knobBtnVariant = document.getElementById('knob-btn-variant');
  const knobBtnSize = document.getElementById('knob-btn-size');
  const knobBtnDisabled = document.getElementById('knob-btn-disabled');
  const knobBtnText = document.getElementById('knob-btn-text');
  let btnClickCount = 0;

  function updateButton() {
    if (!buttonTarget) return;
    const variant = knobBtnVariant?.value || 'prominent';
    const size = knobBtnSize?.value || 'regular';
    const isDisabled = knobBtnDisabled?.checked || false;
    const text = knobBtnText?.value || 'Save Changes';

    buttonTarget.className = 'mac-button';
    if (variant !== 'push') buttonTarget.classList.add(variant);
    if (size !== 'regular') buttonTarget.classList.add(size);
    if (isDisabled) buttonTarget.classList.add('disabled');
    buttonTarget.textContent = text;

    if (buttonCode) {
      let c = `MacButton::new("action-btn", "${text}")\n`;
      if (variant === 'prominent') c += `    .prominent()\n`;
      else if (variant === 'destructive') c += `    .destructive()\n`;
      else if (variant === 'borderless') c += `    .borderless()\n`;
      if (size === 'small') c += `    .size(MacButtonSize::Small)\n`;
      else if (size === 'large') c += `    .size(MacButtonSize::Large)\n`;
      if (isDisabled) c += `    .disabled(true)\n`;
      c += `    .on_click(cx.listener(|this, _event, cx| {\n        this.save_changes();\n    }))`;
      buttonCode.textContent = c;
    }
  }

  if (buttonTarget) {
    buttonTarget.addEventListener('click', () => {
      if (buttonTarget.classList.contains('disabled')) return;
      btnClickCount++;
      showToast(`Button clicked (${btnClickCount})`);
    });
    knobBtnVariant?.addEventListener('change', updateButton);
    knobBtnSize?.addEventListener('change', updateButton);
    knobBtnDisabled?.addEventListener('change', updateButton);
    knobBtnText?.addEventListener('input', updateButton);
  }

  // ----------------------------------------------------
  // 3. MacCheckbox Playground
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
    const text = knobCheckText?.value || 'Generate Smart Previews';

    checkboxBoxTarget.classList.remove('checked', 'mixed');
    if (state === 'checked') checkboxBoxTarget.classList.add('checked');
    else if (state === 'mixed') checkboxBoxTarget.classList.add('mixed');

    if (isDisabled) checkboxRowTarget?.classList.add('disabled');
    else checkboxRowTarget?.classList.remove('disabled');

    if (checkboxLabelTarget) checkboxLabelTarget.textContent = text;

    if (checkboxCode) {
      let stateStr = state === 'checked' ? 'MacCheckboxState::Checked' : (state === 'mixed' ? 'MacCheckboxState::Mixed' : 'MacCheckboxState::Unchecked');
      let c = `MacCheckbox::new("meta-check", ${stateStr})\n    .label("${text}")\n`;
      if (isDisabled) c += `    .disabled(true)\n`;
      c += `    .on_click(cx.listener(|this, state, cx| {\n        this.smart_previews = state.is_checked();\n    }))`;
      checkboxCode.textContent = c;
    }
  }

  if (checkboxRowTarget) {
    checkboxRowTarget.addEventListener('click', () => {
      if (checkboxRowTarget.classList.contains('disabled')) return;
      const current = knobCheckState?.value || 'checked';
      let next = current === 'checked' ? 'mixed' : (current === 'mixed' ? 'unchecked' : 'checked');
      if (knobCheckState) knobCheckState.value = next;
      updateCheckbox();
    });
    knobCheckState?.addEventListener('change', updateCheckbox);
    knobCheckDisabled?.addEventListener('change', updateCheckbox);
    knobCheckText?.addEventListener('input', updateCheckbox);
  }

  // ----------------------------------------------------
  // 4. MacRadio Group Playground
  // ----------------------------------------------------
  const radioGroup = document.getElementById('knob-target-radio-group');
  const radioSelect = document.getElementById('knob-radio-selected');
  const radioDisabled = document.getElementById('knob-radio-disabled');
  const radioCode = document.getElementById('dynamic-code-radio');

  function updateRadio(val) {
    if (!radioGroup) return;
    const selectedVal = val || radioSelect?.value || 'lossless';
    const isDisabled = radioDisabled?.checked || false;

    if (radioSelect) radioSelect.value = selectedVal;
    if (isDisabled) radioGroup.classList.add('disabled');
    else radioGroup.classList.remove('disabled');

    const rows = radioGroup.querySelectorAll('.mac-radio-row');
    rows.forEach(r => {
      if (r.getAttribute('data-value') === selectedVal) r.classList.add('selected');
      else r.classList.remove('selected');
    });

    if (radioCode) {
      radioCode.textContent = `MacRadioGroup::new("codec-group")\n    .selected("${selectedVal}")\n    .disabled(${isDisabled})\n    .on_change(cx.listener(|this, val, cx| {\n        this.selected_codec = val.to_string();\n    }))`;
    }
  }

  if (radioGroup) {
    const radioRows = radioGroup.querySelectorAll('.mac-radio-row');
    radioRows.forEach(row => {
      row.addEventListener('click', () => {
        if (radioGroup.classList.contains('disabled')) return;
        const val = row.getAttribute('data-value');
        updateRadio(val);
        const label = row.querySelector('.mac-radio-text')?.textContent.trim() || val;
        showToast(`Selected: ${label}`);
      });
    });
    radioSelect?.addEventListener('change', () => updateRadio(radioSelect.value));
    radioDisabled?.addEventListener('change', () => updateRadio());
  }

  // ----------------------------------------------------
  // 5. MacComboBox Playground
  // ----------------------------------------------------
  const comboTrigger = document.getElementById('knob-combobox-trigger');
  const comboDropdown = document.getElementById('knob-combobox-dropdown');
  const comboValue = document.getElementById('knob-combobox-value');
  const comboSelect = document.getElementById('knob-combobox-select');
  const comboDisabled = document.getElementById('knob-combobox-disabled');
  const comboTarget = document.getElementById('knob-target-combobox');
  const comboCode = document.getElementById('dynamic-code-combobox');

  function updateComboBox(val) {
    const selectedVal = val || comboSelect?.value || 'Apple ProRAW (14-bit)';
    const isDisabled = comboDisabled?.checked || false;
    if (comboValue) comboValue.textContent = selectedVal;
    if (comboSelect) comboSelect.value = selectedVal;

    if (comboDropdown) {
      comboDropdown.querySelectorAll('.mac-combobox-option').forEach(o => {
        if (o.getAttribute('data-value') === selectedVal || o.textContent.trim() === selectedVal) {
          o.classList.add('selected');
        } else {
          o.classList.remove('selected');
        }
      });
    }

    if (isDisabled) {
      comboTarget?.classList.add('disabled');
      comboDropdown?.classList.remove('open');
    } else {
      comboTarget?.classList.remove('disabled');
    }

    if (comboCode) {
      comboCode.textContent = `MacComboBox::new("raw-codec")\n    .placeholder("Select RAW Preset...")\n    .option("${selectedVal}")\n    .disabled(${isDisabled})\n    .selected_index(Some(0))\n    .on_select(cx.listener(|this, _idx, text, cx| {\n        this.active_profile = text.to_string();\n    }))`;
    }
  }

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
        updateComboBox(val);
        comboDropdown.classList.remove('open');
        showToast(`Selected: ${val}`);
      });
    });

    comboSelect?.addEventListener('change', () => updateComboBox(comboSelect.value));
    comboDisabled?.addEventListener('change', () => updateComboBox());
  }

  // ----------------------------------------------------
  // 6. MacDigitEntry Playground
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
  // 7. MacImageWell Playground
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
  // 8. MacDatePicker Playground & Interactive Calendar
  // ----------------------------------------------------
  const dateInput = document.getElementById('knob-date-input');
  const dateShowTime = document.getElementById('knob-date-showtime');
  const dateStyleKnob = document.getElementById('knob-date-style');
  const dateDisabledKnob = document.getElementById('knob-date-disabled');
  const dateDisp = document.getElementById('knob-date-val');
  const dateTrigger = document.getElementById('knob-date-trigger');
  const timeSeg = document.getElementById('knob-time-segment');
  const timeVal = document.getElementById('knob-time-val');
  const dateCode = document.getElementById('dynamic-code-date');

  const dateCompactView = document.getElementById('datepicker-compact-view');
  const dateGraphicalView = document.getElementById('datepicker-graphical-view');
  const dateStepperView = document.getElementById('datepicker-stepper-view');
  const datePopover = document.getElementById('datepicker-calendar-popover');
  const calHeaderTitle = document.getElementById('cal-header-title');
  const calDaysGrid = document.getElementById('cal-days-grid');
  const calPrevBtn = document.getElementById('cal-prev-btn');
  const calNextBtn = document.getElementById('cal-next-btn');
  const calTodayBtn = document.getElementById('cal-today-btn');
  const calFooterTime = document.getElementById('cal-footer-time');

  const calInlineTitle = document.querySelector('.cal-inline-title');
  const calInlineDaysGrid = document.querySelector('.cal-inline-days-grid');
  const calInlinePrev = document.querySelector('.cal-inline-prev');
  const calInlineNext = document.querySelector('.cal-inline-next');
  const calInlineToday = document.querySelector('.cal-inline-today');
  const calInlineTime = document.querySelector('.cal-inline-time');

  const stepperDateDisp = document.getElementById('stepper-date-disp');
  const stepperDateUp = document.getElementById('stepper-date-up');
  const stepperDateDown = document.getElementById('stepper-date-down');

  let curYear = 2026;
  let curMonth = 9; // 0-indexed: 9 = October
  let curDay = 21;
  const monthNames = ['January', 'February', 'March', 'April', 'May', 'June', 'July', 'August', 'September', 'October', 'November', 'December'];
  const shortMonthNames = ['Jan', 'Feb', 'Mar', 'Apr', 'May', 'Jun', 'Jul', 'Aug', 'Sep', 'Oct', 'Nov', 'Dec'];

  function buildCalendarCells(gridEl, isInline) {
    if (!gridEl) return;
    gridEl.innerHTML = '';

    const firstDayIndex = new Date(curYear, curMonth, 1).getDay();
    const daysInMonth = new Date(curYear, curMonth + 1, 0).getDate();
    const prevMonthDays = new Date(curYear, curMonth, 0).getDate();

    // Previous month trailing days
    for (let i = firstDayIndex - 1; i >= 0; i--) {
      const dayNum = prevMonthDays - i;
      const cell = document.createElement('div');
      cell.className = 'mac-cal-day other-month';
      cell.textContent = dayNum;
      gridEl.appendChild(cell);
    }

    // Current month days
    for (let day = 1; day <= daysInMonth; day++) {
      const cell = document.createElement('div');
      cell.className = 'mac-cal-day';
      if (day === curDay) cell.classList.add('selected');
      cell.textContent = day;

      cell.addEventListener('click', (e) => {
        e.stopPropagation();
        curDay = day;
        updateDatePicker();
        if (!isInline && datePopover) {
          datePopover.classList.remove('open');
          dateTrigger?.classList.remove('open');
        }
        showToast(`Date selected: ${shortMonthNames[curMonth]} ${curDay}, ${curYear}`);
      });

      gridEl.appendChild(cell);
    }

    // Next month leading days
    const totalRendered = firstDayIndex + daysInMonth;
    const remaining = (7 - (totalRendered % 7)) % 7;
    for (let next = 1; next <= remaining; next++) {
      const cell = document.createElement('div');
      cell.className = 'mac-cal-day other-month';
      cell.textContent = next;
      gridEl.appendChild(cell);
    }
  }

  function updateDatePicker() {
    const style = dateStyleKnob?.value || 'compact';
    const showTime = dateShowTime?.checked ?? true;
    const isDisabled = dateDisabledKnob?.checked ?? false;

    const dateFormatted = `${shortMonthNames[curMonth]} ${curDay}, ${curYear}`;
    const timeFormatted = '12:30 PM';

    if (dateDisp) dateDisp.textContent = dateFormatted;
    if (dateInput) dateInput.value = dateFormatted;
    if (stepperDateDisp) stepperDateDisp.textContent = dateFormatted;
    if (timeSeg) timeSeg.style.display = showTime ? 'inline-flex' : 'none';
    if (timeVal) timeVal.textContent = timeFormatted;
    if (calFooterTime) calFooterTime.textContent = timeFormatted;
    if (calInlineTime) calInlineTime.textContent = timeFormatted;

    // View Switching
    if (dateCompactView) dateCompactView.style.display = style === 'compact' ? 'inline-flex' : 'none';
    if (dateGraphicalView) dateGraphicalView.style.display = style === 'graphical' ? 'flex' : 'none';
    if (dateStepperView) dateStepperView.style.display = style === 'stepper' ? 'inline-flex' : 'none';

    if (style !== 'compact' && datePopover) {
      datePopover.classList.remove('open');
      dateTrigger?.classList.remove('open');
    }

    // Disabled state
    const container = document.getElementById('knob-target-datepicker-container');
    if (container) {
      if (isDisabled) container.classList.add('disabled');
      else container.classList.remove('disabled');
    }
    if (dateCompactView) {
      if (isDisabled) dateCompactView.classList.add('disabled');
      else dateCompactView.classList.remove('disabled');
    }
    if (dateGraphicalView) {
      if (isDisabled) dateGraphicalView.classList.add('disabled');
      else dateGraphicalView.classList.remove('disabled');
    }

    // Titles & Grid
    const titleText = `${monthNames[curMonth]} ${curYear}`;
    if (calHeaderTitle) calHeaderTitle.textContent = titleText;
    if (calInlineTitle) calInlineTitle.textContent = titleText;

    buildCalendarCells(calDaysGrid, false);
    buildCalendarCells(calInlineDaysGrid, true);

    if (dateCode) {
      const stylePascal = style === 'graphical' ? 'Graphical' : (style === 'stepper' ? 'Stepper' : 'Compact');
      let c = `MacDatePicker::new("date-picker")\n`;
      c += `    .picker_style(MacDatePickerStyle::${stylePascal})\n`;
      c += `    .date(${curYear}, ${curMonth + 1}, ${curDay})\n`;
      if (showTime) c += `    .time(12, 30)\n    .show_time(true)\n`;
      else c += `    .show_time(false)\n`;
      if (isDisabled) c += `    .disabled(true)\n`;
      c += `    .on_change(cx.listener(|this, y, m, d, cx| {\n        this.update_timestamp(y, m, d);\n    }))`;
      dateCode.textContent = c;
    }
  }

  // Event Listeners for DatePicker
  if (dateTrigger && datePopover) {
    dateTrigger.addEventListener('click', (e) => {
      e.stopPropagation();
      if (dateCompactView?.classList.contains('disabled')) return;
      const isOpen = datePopover.classList.toggle('open');
      dateTrigger.classList.toggle('open', isOpen);
    });

    document.addEventListener('click', (e) => {
      if (!datePopover.contains(e.target) && !dateTrigger.contains(e.target)) {
        datePopover.classList.remove('open');
        dateTrigger.classList.remove('open');
      }
    });
  }

  function prevMonth() {
    curMonth--;
    if (curMonth < 0) {
      curMonth = 11;
      curYear--;
    }
    curDay = Math.min(curDay, new Date(curYear, curMonth + 1, 0).getDate());
    updateDatePicker();
  }

  function nextMonth() {
    curMonth++;
    if (curMonth > 11) {
      curMonth = 0;
      curYear++;
    }
    curDay = Math.min(curDay, new Date(curYear, curMonth + 1, 0).getDate());
    updateDatePicker();
  }

  function setToday() {
    const today = new Date();
    curYear = today.getFullYear();
    curMonth = today.getMonth();
    curDay = today.getDate();
    updateDatePicker();
    showToast(`Jumped to today: ${shortMonthNames[curMonth]} ${curDay}, ${curYear}`);
  }

  calPrevBtn?.addEventListener('click', (e) => { e.stopPropagation(); prevMonth(); });
  calNextBtn?.addEventListener('click', (e) => { e.stopPropagation(); nextMonth(); });
  calTodayBtn?.addEventListener('click', (e) => { e.stopPropagation(); setToday(); });

  calInlinePrev?.addEventListener('click', (e) => { e.stopPropagation(); prevMonth(); });
  calInlineNext?.addEventListener('click', (e) => { e.stopPropagation(); nextMonth(); });
  calInlineToday?.addEventListener('click', (e) => { e.stopPropagation(); setToday(); });

  stepperDateUp?.addEventListener('click', () => {
    const maxDays = new Date(curYear, curMonth + 1, 0).getDate();
    curDay = curDay < maxDays ? curDay + 1 : 1;
    updateDatePicker();
  });

  stepperDateDown?.addEventListener('click', () => {
    const maxDays = new Date(curYear, curMonth + 1, 0).getDate();
    curDay = curDay > 1 ? curDay - 1 : maxDays;
    updateDatePicker();
  });

  dateStyleKnob?.addEventListener('change', updateDatePicker);
  dateShowTime?.addEventListener('change', updateDatePicker);
  dateDisabledKnob?.addEventListener('change', updateDatePicker);
  dateInput?.addEventListener('input', (e) => {
    const val = e.target.value;
    const parts = val.replace(',', '').split(/\s+/);
    if (parts.length >= 3) {
      const mIdx = shortMonthNames.findIndex(m => m.toLowerCase() === parts[0].toLowerCase().slice(0, 3));
      const d = parseInt(parts[1], 10);
      const y = parseInt(parts[2], 10);
      if (mIdx >= 0 && !isNaN(d) && !isNaN(y)) {
        curMonth = mIdx;
        curDay = d;
        curYear = y;
        updateDatePicker();
      }
    }
  });

  // ----------------------------------------------------
  // 9. MacSegmentedControl Playground
  // ----------------------------------------------------
  const segmentedTarget = document.getElementById('knob-target-segmented');
  const segmentedSelect = document.getElementById('knob-segment-active');
  const segmentedDisabled = document.getElementById('knob-segment-disabled');
  const segmentedDisplay = document.getElementById('segmented-live-display');
  const segmentedCode = document.getElementById('dynamic-code-segmented');

  const segmentLabels = ['Grid View (0)', 'List Details (1)', 'Map Geo (2)'];

  function updateSegmented(idx) {
    if (!segmentedTarget) return;
    const activeIdx = idx !== undefined ? String(idx) : (segmentedSelect?.value || '0');
    const isDisabled = segmentedDisabled?.checked || false;

    if (segmentedSelect) segmentedSelect.value = activeIdx;
    if (isDisabled) segmentedTarget.classList.add('disabled');
    else segmentedTarget.classList.remove('disabled');

    const segments = segmentedTarget.querySelectorAll('.mac-segment');
    segments.forEach(seg => {
      if (seg.getAttribute('data-index') === activeIdx) seg.classList.add('active');
      else seg.classList.remove('active');
    });

    const activeLabel = segmentLabels[parseInt(activeIdx, 10)] || `Mode ${activeIdx}`;
    if (segmentedDisplay) {
      segmentedDisplay.innerHTML = `Active Mode: <strong>${activeLabel}</strong>`;
    }

    if (segmentedCode) {
      segmentedCode.textContent = `MacSegmentedControl::new("view-switcher", ${activeIdx})\n    .segment(MacSegment::new("grid", "Grid View"))\n    .segment(MacSegment::new("list", "List Details"))\n    .segment(MacSegment::new("map", "Map Geo"))\n    .disabled(${isDisabled})\n    .on_change(cx.listener(|this, idx, cx| {\n        this.view_idx = idx;\n    }))`;
    }
  }

  if (segmentedTarget) {
    segmentedTarget.querySelectorAll('.mac-segment').forEach(seg => {
      seg.addEventListener('click', () => {
        if (segmentedTarget.classList.contains('disabled')) return;
        const idx = seg.getAttribute('data-index');
        updateSegmented(idx);
        showToast(`Segment: ${seg.textContent.trim()}`);
      });
    });
    segmentedSelect?.addEventListener('change', () => updateSegmented());
    segmentedDisabled?.addEventListener('change', () => updateSegmented());
  }

  // ----------------------------------------------------
  // 10. MacSlider Playground
  // ----------------------------------------------------
  const targetSlider = document.getElementById('knob-target-slider');
  const sliderBadge = document.getElementById('knob-slider-val-badge');
  const sliderNumInput = document.getElementById('knob-slider-num');
  const sliderDisabledCheck = document.getElementById('knob-slider-disabled');
  const sliderCode = document.getElementById('dynamic-code-slider');

  function updateSlider(val) {
    if (!targetSlider) return;
    const v = val !== undefined ? val : (targetSlider.value || '65');
    const isDisabled = sliderDisabledCheck?.checked || false;

    targetSlider.value = v;
    if (sliderNumInput) sliderNumInput.value = v;
    if (sliderBadge) sliderBadge.textContent = `${v}%`;
    targetSlider.disabled = isDisabled;

    if (sliderCode) {
      sliderCode.textContent = `MacSlider::new("exposure-slider", ${(v / 100).toFixed(2)})\n    .min(0.0)\n    .max(1.0)\n    .disabled(${isDisabled})\n    .on_change(cx.listener(|this, val, cx| {\n        this.exposure_value = val;\n    }))`;
    }
  }

  targetSlider?.addEventListener('input', () => updateSlider(targetSlider.value));
  sliderNumInput?.addEventListener('input', () => updateSlider(sliderNumInput.value));
  sliderDisabledCheck?.addEventListener('change', () => updateSlider(targetSlider?.value));

  // ----------------------------------------------------
  // 11. MacStepper Playground
  // ----------------------------------------------------
  const stepperValDisp = document.getElementById('knob-stepper-val-display');
  const stepperValInput = document.getElementById('knob-stepper-val-input');
  const stepperStepInput = document.getElementById('knob-stepper-step');
  const stepperUpBtn = document.getElementById('knob-stepper-up');
  const stepperDownBtn = document.getElementById('knob-stepper-down');
  const stepperCode = document.getElementById('dynamic-code-stepper');

  function updateStepper(newVal) {
    const val = newVal !== undefined ? newVal : parseInt(stepperValInput?.value || '42', 10);
    const step = parseInt(stepperStepInput?.value || '1', 10);
    if (stepperValDisp) stepperValDisp.textContent = val;
    if (stepperValInput) stepperValInput.value = val;

    if (stepperCode) {
      stepperCode.textContent = `MacStepper::new("photo-batch-count", ${val})\n    .step(${step})\n    .min(0)\n    .max(999)\n    .on_change(cx.listener(|this, val, cx| {\n        this.batch_count = val;\n    }))`;
    }
  }

  stepperUpBtn?.addEventListener('click', () => {
    const step = parseInt(stepperStepInput?.value || '1', 10);
    const cur = parseInt(stepperValInput?.value || '42', 10);
    updateStepper(cur + step);
    showToast(`Value: ${cur + step}`);
  });

  stepperDownBtn?.addEventListener('click', () => {
    const step = parseInt(stepperStepInput?.value || '1', 10);
    const cur = parseInt(stepperValInput?.value || '42', 10);
    updateStepper(Math.max(0, cur - step));
    showToast(`Value: ${Math.max(0, cur - step)}`);
  });

  stepperValInput?.addEventListener('input', () => updateStepper());
  stepperStepInput?.addEventListener('input', () => updateStepper());

  // ----------------------------------------------------
  // 12. MacColorWell Playground
  // ----------------------------------------------------
  const targetColorInput = document.getElementById('knob-target-color-input');
  const sideColorPicker = document.getElementById('knob-color-picker-side');
  const colorHexDisp = document.getElementById('knob-color-hex');
  const colorCode = document.getElementById('dynamic-code-color');

  function updateColor(hex) {
    const val = (hex || targetColorInput?.value || '#007AFF').toUpperCase();
    if (targetColorInput) targetColorInput.value = val;
    if (sideColorPicker) sideColorPicker.value = val;
    if (colorHexDisp) colorHexDisp.textContent = val;

    if (colorCode) {
      colorCode.textContent = `MacColorWell::new("accent-color")\n    .color(gpui::rgb(0x${val.replace('#', '')}))\n    .alpha(1.0)\n    .on_change(cx.listener(|this, color, cx| {\n        this.highlight_color = color;\n    }))`;
    }
  }

  targetColorInput?.addEventListener('input', (e) => updateColor(e.target.value));
  sideColorPicker?.addEventListener('input', (e) => updateColor(e.target.value));

  // ----------------------------------------------------
  // 13. MacTextField Playground
  // ----------------------------------------------------
  const textFieldTarget = document.getElementById('knob-target-text-field');
  const textInputTarget = document.getElementById('knob-target-text-input');
  const textSizeSelect = document.getElementById('knob-text-size');
  const textSecureCheck = document.getElementById('knob-text-secure');
  const textDisabledCheck = document.getElementById('knob-text-disabled');
  const textCode = document.getElementById('dynamic-code-text');

  function updateTextField() {
    if (!textFieldTarget || !textInputTarget) return;
    const size = textSizeSelect?.value || 'regular';
    const isSecure = textSecureCheck?.checked || false;
    const isDisabled = textDisabledCheck?.checked || false;
    const text = textInputTarget.value || '';

    textFieldTarget.classList.remove('small', 'large');
    if (size === 'small') textFieldTarget.classList.add('small');
    else if (size === 'large') textFieldTarget.classList.add('large');

    textInputTarget.type = isSecure ? 'password' : 'text';
    textInputTarget.disabled = isDisabled;
    if (isDisabled) textFieldTarget.classList.add('disabled');
    else textFieldTarget.classList.remove('disabled');

    if (textCode) {
      let c = `MacTextField::new("title-field")\n    .value("${text}")\n`;
      if (size === 'small') c += `    .size(MacFieldSize::Small)\n`;
      else if (size === 'large') c += `    .size(MacFieldSize::Large)\n`;
      if (isSecure) c += `    .secure(true)\n`;
      if (isDisabled) c += `    .disabled(true)\n`;
      c += `    .on_change(cx.listener(|this, val, cx| {\n        this.collection_title = val.to_string();\n    }))`;
      textCode.textContent = c;
    }
  }

  textInputTarget?.addEventListener('input', updateTextField);
  textSizeSelect?.addEventListener('change', updateTextField);
  textSecureCheck?.addEventListener('change', updateTextField);
  textDisabledCheck?.addEventListener('change', updateTextField);

  // ----------------------------------------------------
  // 14. MacPopupButton & MacPullDownButton Playground
  // ----------------------------------------------------
  const popupTargetBtn = document.getElementById('knob-target-popup-btn');
  const popupLabelDisp = document.getElementById('knob-popup-label');
  const popupDropdownMenu = document.getElementById('knob-popup-dropdown');
  const popupSelectKnob = document.getElementById('knob-popup-select');

  const pulldownTargetBtn = document.getElementById('knob-target-pulldown-btn');
  const pulldownLabelDisp = document.getElementById('knob-pulldown-label');
  const pulldownDropdownMenu = document.getElementById('knob-pulldown-dropdown');
  const pulldownSelectKnob = document.getElementById('knob-pulldown-label-select');

  const popupModeKnob = document.getElementById('knob-popup-mode');
  const popupModeBadge = document.getElementById('popup-mode-badge');
  const containerPopup = document.getElementById('container-popup-control');
  const containerPulldown = document.getElementById('container-pulldown-control');
  const rowKnobPopupProfile = document.getElementById('row-knob-popup-profile');
  const rowKnobPulldownAction = document.getElementById('row-knob-pulldown-action');

  const popupDisabledCheck = document.getElementById('knob-popup-disabled');
  const popupCode = document.getElementById('dynamic-code-popup');

  function updatePopupControls(selectedProfileVal, pulldownLabelVal) {
    const mode = popupModeKnob?.value || 'both';
    const isDisabled = popupDisabledCheck?.checked || false;
    const selectedProfile = selectedProfileVal || popupSelectKnob?.value || 'Display P3 (Wide Color)';
    const pulldownTitle = pulldownLabelVal || pulldownSelectKnob?.value || 'Export Actions';

    // Update Mode Visibility & Badges
    if (popupModeBadge) {
      if (mode === 'both') popupModeBadge.textContent = 'Side-by-Side';
      else if (mode === 'popup') popupModeBadge.textContent = 'Pop-Up Only';
      else if (mode === 'pulldown') popupModeBadge.textContent = 'Pull-Down Only';
    }

    if (containerPopup) {
      containerPopup.style.display = (mode === 'pulldown') ? 'none' : 'flex';
    }
    if (containerPulldown) {
      containerPulldown.style.display = (mode === 'popup') ? 'none' : 'flex';
    }
    if (rowKnobPopupProfile) {
      rowKnobPopupProfile.style.display = (mode === 'pulldown') ? 'none' : 'flex';
    }
    if (rowKnobPulldownAction) {
      rowKnobPulldownAction.style.display = (mode === 'popup') ? 'none' : 'flex';
    }

    // Update Pop-Up Button state
    if (popupLabelDisp) popupLabelDisp.textContent = selectedProfile;
    if (popupSelectKnob) popupSelectKnob.value = selectedProfile;

    if (popupDropdownMenu) {
      popupDropdownMenu.querySelectorAll('.mac-dropdown-item, .mac-popover-item').forEach(item => {
        const val = item.getAttribute('data-value') || item.querySelector('.mac-dropdown-title')?.textContent?.trim() || item.textContent.trim();
        const checkEl = item.querySelector('.mac-dropdown-check');
        if (val === selectedProfile) {
          item.classList.add('selected');
          if (checkEl) checkEl.textContent = '✓';
        } else {
          item.classList.remove('selected');
          if (checkEl) checkEl.textContent = '';
        }
      });
    }

    if (popupTargetBtn) {
      if (isDisabled) {
        popupTargetBtn.classList.add('disabled');
        popupTargetBtn.setAttribute('aria-disabled', 'true');
      } else {
        popupTargetBtn.classList.remove('disabled');
        popupTargetBtn.removeAttribute('aria-disabled');
      }
    }

    // Update Pull-Down Button state
    if (pulldownLabelDisp) pulldownLabelDisp.textContent = pulldownTitle;
    if (pulldownSelectKnob) pulldownSelectKnob.value = pulldownTitle;

    if (pulldownTargetBtn) {
      if (isDisabled) {
        pulldownTargetBtn.classList.add('disabled');
        pulldownTargetBtn.setAttribute('aria-disabled', 'true');
      } else {
        pulldownTargetBtn.classList.remove('disabled');
        pulldownTargetBtn.removeAttribute('aria-disabled');
      }
    }

    // Dynamic GPUI Code Generation
    if (popupCode) {
      const profileIndex = selectedProfile.includes('P3') ? 0
        : selectedProfile.includes('sRGB') ? 1
        : selectedProfile.includes('Adobe') ? 2 : 3;

      if (mode === 'both') {
        popupCode.textContent = `// 1. Mutually Exclusive Pop-Up Button (Double Chevrons)
MacPopupButton::new("color-space")
    .item(MacMenuItem::new("p3", "Display P3 (Wide Color)"))
    .item(MacMenuItem::new("srgb", "sRGB (Web Standard)"))
    .item(MacMenuItem::new("adobe", "Adobe RGB (1998)"))
    .item(MacMenuItem::new("prophoto", "ProPhoto RGB (Archival)"))
    .selected_index(${profileIndex})
    .disabled(${isDisabled})
    .on_change(cx.listener(|this, idx, cx| {
        this.selected_color_space = *idx;
    }))

// 2. Action Trigger Pull-Down Button (Single Chevron)
MacPullDownButton::new("export-actions", "${pulldownTitle}")
    .icon(IconName::Share)
    .disabled(${isDisabled})
    .on_click(cx.listener(|this, _event, cx| {
        this.show_export_menu(cx);
    }))`;
      } else if (mode === 'popup') {
        popupCode.textContent = `MacPopupButton::new("color-space")
    .item(MacMenuItem::new("p3", "Display P3 (Wide Color)"))
    .item(MacMenuItem::new("srgb", "sRGB (Web Standard)"))
    .item(MacMenuItem::new("adobe", "Adobe RGB (1998)"))
    .item(MacMenuItem::new("prophoto", "ProPhoto RGB (Archival)"))
    .selected_index(${profileIndex})
    .disabled(${isDisabled})
    .on_change(cx.listener(|this, idx, cx| {
        this.selected_color_space = *idx;
    }))`;
      } else {
        popupCode.textContent = `MacPullDownButton::new("export-actions", "${pulldownTitle}")
    .icon(IconName::Share)
    .disabled(${isDisabled})
    .on_click(cx.listener(|this, _event, cx| {
        this.show_export_menu(cx);
    }))`;
      }
    }
  }

  // Pop-Up Button Interactions
  if (popupTargetBtn && popupDropdownMenu) {
    popupTargetBtn.addEventListener('click', (e) => {
      e.stopPropagation();
      if (popupTargetBtn.classList.contains('disabled')) return;
      if (pulldownDropdownMenu) pulldownDropdownMenu.classList.remove('open');
      const isOpen = popupDropdownMenu.classList.toggle('open');
      popupTargetBtn.setAttribute('aria-expanded', String(isOpen));
    });

    popupDropdownMenu.querySelectorAll('.mac-dropdown-item, .mac-popover-item').forEach(item => {
      item.addEventListener('click', (e) => {
        e.stopPropagation();
        const val = item.getAttribute('data-value') || item.querySelector('.mac-dropdown-title')?.textContent?.trim() || item.textContent.trim();
        updatePopupControls(val);
        popupDropdownMenu.classList.remove('open');
        popupTargetBtn.setAttribute('aria-expanded', 'false');
        showToast(`Color Profile: ${val}`);
      });
    });
  }

  // Pull-Down Button Interactions
  if (pulldownTargetBtn && pulldownDropdownMenu) {
    pulldownTargetBtn.addEventListener('click', (e) => {
      e.stopPropagation();
      if (pulldownTargetBtn.classList.contains('disabled')) return;
      if (popupDropdownMenu) popupDropdownMenu.classList.remove('open');
      const isOpen = pulldownDropdownMenu.classList.toggle('open');
      pulldownTargetBtn.setAttribute('aria-expanded', String(isOpen));
    });

    pulldownDropdownMenu.querySelectorAll('.mac-dropdown-item, .mac-popover-item').forEach(item => {
      item.addEventListener('click', (e) => {
        e.stopPropagation();
        const action = item.getAttribute('data-action') || item.querySelector('.mac-dropdown-title')?.textContent?.trim() || item.textContent.trim();
        const shortcut = item.getAttribute('data-shortcut') || item.querySelector('.mac-kbd')?.textContent?.trim() || '';
        pulldownDropdownMenu.classList.remove('open');
        pulldownTargetBtn.setAttribute('aria-expanded', 'false');
        showToast(`Executed: ${action}${shortcut ? ' (' + shortcut + ')' : ''}`);
      });
    });
  }

  // Global Outside Click to dismiss both menus
  document.addEventListener('click', (e) => {
    if (popupDropdownMenu && popupTargetBtn && !popupTargetBtn.contains(e.target) && !popupDropdownMenu.contains(e.target)) {
      popupDropdownMenu.classList.remove('open');
      popupTargetBtn.setAttribute('aria-expanded', 'false');
    }
    if (pulldownDropdownMenu && pulldownTargetBtn && !pulldownTargetBtn.contains(e.target) && !pulldownDropdownMenu.contains(e.target)) {
      pulldownDropdownMenu.classList.remove('open');
      pulldownTargetBtn.setAttribute('aria-expanded', 'false');
    }
  });

  // Sidebar controls
  popupModeKnob?.addEventListener('change', () => updatePopupControls());
  popupSelectKnob?.addEventListener('change', () => updatePopupControls(popupSelectKnob.value));
  pulldownSelectKnob?.addEventListener('change', () => updatePopupControls(undefined, pulldownSelectKnob.value));
  popupDisabledCheck?.addEventListener('change', () => updatePopupControls());

  // Initialize
  updatePopupControls();

  // ----------------------------------------------------
  // 15. MacContextMenu Playground
  // ----------------------------------------------------
  const contextMenuCard = document.querySelector('.mac-context-menu-card');
  const menuWidthInput = document.getElementById('knob-menu-w');
  const menuShortcutsCheck = document.getElementById('knob-menu-sc');
  const contextMenuCode = document.getElementById('dynamic-code-contextmenu');

  function updateContextMenu() {
    if (!contextMenuCard) return;
    const w = parseInt(menuWidthInput?.value || '220', 10);
    const showShortcuts = menuShortcutsCheck?.checked ?? true;

    contextMenuCard.style.width = `${w}px`;
    contextMenuCard.querySelectorAll('.mac-context-row span:last-child').forEach(s => {
      s.style.display = showShortcuts ? 'inline' : 'none';
    });

    if (contextMenuCode) {
      contextMenuCode.textContent = `MacContextMenu::new("photo-actions")\n    .width(${w}.0)\n    .item("Open in Loupe", Some("Return"))\n    .item("Export DNG...", Some("Shift+Cmd+E"))\n    .separator()\n    .item("Copy Metadata", Some("Cmd+C"))\n    .item("Show in Finder", Some("Cmd+R"))\n    .separator()\n    .destructive_item("Delete Photo", Some("Cmd+Backspace"))`;
    }
  }

  menuWidthInput?.addEventListener('input', updateContextMenu);
  menuShortcutsCheck?.addEventListener('change', updateContextMenu);
  contextMenuCard?.querySelectorAll('.mac-context-row').forEach(row => {
    row.addEventListener('click', () => {
      showToast(`Action: ${row.querySelector('span:first-child')?.textContent.trim()}`);
    });
  });

  // ----------------------------------------------------
  // 16. MacMenuBar Playground
  // ----------------------------------------------------
  const menuBarTitleInput = document.getElementById('knob-menubar-title');
  const menuBarAppDisp = document.getElementById('knob-menubar-appname');
  const menuBarCode = document.getElementById('dynamic-code-menubar');

  function updateMenuBar() {
    const title = menuBarTitleInput?.value || 'Loupe';
    if (menuBarAppDisp) menuBarAppDisp.textContent = title;
    if (menuBarCode) {
      menuBarCode.textContent = `MacMenuBar::new("sys-menubar", "${title}")\n    .menus(vec!["File", "Edit", "View", "Window", "Help"])\n    .active_menu(Some(0))\n    .on_item_click(cx.listener(|this, menu, item, cx| {\n        this.handle_menu_cmd(menu, item);\n    }))`;
    }
  }

  menuBarTitleInput?.addEventListener('input', updateMenuBar);
  document.querySelectorAll('.mac-menubar-item').forEach(item => {
    item.addEventListener('click', () => {
      document.querySelectorAll('.mac-menubar-item').forEach(i => i.classList.remove('active'));
      item.classList.add('active');
      showToast(`Menu: ${item.textContent.trim()}`);
    });
  });

  // ----------------------------------------------------
  // 17. MacSearchField Playground
  // ----------------------------------------------------
  const searchInputTarget = document.getElementById('knob-target-search-input');
  const searchPlaceholderKnob = document.getElementById('knob-search-placeholder');
  const searchHintDisp = document.getElementById('search-results-hint');
  const searchCode = document.getElementById('dynamic-code-search');

  function updateSearchField() {
    const val = searchInputTarget?.value || '';
    const placeholder = searchPlaceholderKnob?.value || 'Search photos, tags, metadata...';
    if (searchInputTarget) searchInputTarget.placeholder = placeholder;

    if (searchHintDisp) {
      if (val.trim()) {
        searchHintDisp.innerHTML = `Found <strong>4 matches</strong> for "<em>${val}</em>" in RAW catalog`;
      } else {
        searchHintDisp.textContent = 'Type above to see live filter response';
      }
    }

    if (searchCode) {
      searchCode.textContent = `MacSearchField::new("photo-search")\n    .placeholder("${placeholder}")\n    .value("${val}")\n    .shortcut("Cmd+F")\n    .on_change(cx.listener(|this, query, cx| {\n        this.filter_library(query);\n    }))`;
    }
  }

  searchInputTarget?.addEventListener('input', updateSearchField);
  searchPlaceholderKnob?.addEventListener('input', updateSearchField);

  // ----------------------------------------------------
  // 18. MacTabBar Playground
  // ----------------------------------------------------
  const tabSelectKnob = document.getElementById('knob-tab-active-select');
  const tabTargetBar = document.getElementById('knob-target-tabbar');
  const tabCode = document.getElementById('dynamic-code-tabs');

  function updateTabBar(tabId) {
    const cur = tabId || tabSelectKnob?.value || 'general';
    if (tabSelectKnob) tabSelectKnob.value = cur;

    if (tabTargetBar) {
      tabTargetBar.querySelectorAll('.mac-tab-item').forEach(tab => {
        if (tab.getAttribute('data-tab') === cur) tab.classList.add('active');
        else tab.classList.remove('active');
      });
    }

    if (tabCode) {
      tabCode.textContent = `MacTabBar::new("prefs-tabs")\n    .tab("general", "General")\n    .tab("rendering", "Color & Display")\n    .tab("shortcuts", "Shortcuts")\n    .active_tab("${cur}")\n    .on_change(cx.listener(|this, tab, cx| {\n        this.active_settings_tab = tab.to_string();\n    }))`;
    }
  }

  if (tabTargetBar) {
    tabTargetBar.querySelectorAll('.mac-tab-item').forEach(tab => {
      tab.addEventListener('click', () => {
        const tabId = tab.getAttribute('data-tab');
        updateTabBar(tabId);
        showToast(`Tab: ${tab.querySelector('span')?.textContent || tabId}`);
      });
    });
    tabSelectKnob?.addEventListener('change', () => updateTabBar(tabSelectKnob.value));
  }

  // ----------------------------------------------------
  // 19. MacPathControl Playground
  // ----------------------------------------------------
  const pathLeafInput = document.getElementById('knob-path-leaf');
  const pathCurrentCrumb = document.querySelector('.mac-path-crumb.current');
  const pathCode = document.getElementById('dynamic-code-path');

  function updatePathControl() {
    const leaf = pathLeafInput?.value || 'photo-engine';
    if (pathCurrentCrumb) pathCurrentCrumb.textContent = `📷 ${leaf}`;

    if (pathCode) {
      pathCode.textContent = `MacPathControl::new("finder-path")\n    .item("Macintosh HD")\n    .item("Users")\n    .item("mayank")\n    .item("${leaf}")\n    .on_select(cx.listener(|this, path, cx| {\n        this.reveal_in_finder(path);\n    }))`;
    }
  }

  pathLeafInput?.addEventListener('input', updatePathControl);
  document.querySelectorAll('.mac-path-crumb').forEach(crumb => {
    crumb.addEventListener('click', () => {
      showToast(`Path: ${crumb.textContent.trim()}`);
    });
  });

  // ----------------------------------------------------
  // 20. MacTokenField Playground
  // ----------------------------------------------------
  const tokenContainer = document.getElementById('knob-target-tokenfield');
  const tokenAddInput = document.getElementById('knob-token-add-input');
  const tokenCode = document.getElementById('dynamic-code-token');

  function updateTokenCode() {
    if (!tokenCode || !tokenContainer) return;
    const chips = [...tokenContainer.querySelectorAll('.mac-token-chip span:first-child')].map(s => `"${s.textContent.trim()}"`);
    tokenCode.textContent = `MacTokenField::new("metadata-tags")\n    .placeholder("Add keywords...")\n    .tokens(vec![${chips.join(', ')}])\n    .on_add(cx.listener(|this, token, cx| {\n        this.add_tag(token);\n    }))\n    .on_remove(cx.listener(|this, idx, cx| {\n        this.remove_tag(idx);\n    }))`;
  }

  function bindTokenCloses() {
    tokenContainer?.querySelectorAll('.mac-token-chip .close-btn').forEach(btn => {
      btn.onclick = (e) => {
        e.stopPropagation();
        const chip = btn.closest('.mac-token-chip');
        const name = chip?.querySelector('span')?.textContent || 'tag';
        chip?.remove();
        updateTokenCode();
        showToast(`Removed tag: ${name}`);
      };
    });
  }

  bindTokenCloses();
  tokenAddInput?.addEventListener('keydown', (e) => {
    if (e.key === 'Enter') {
      const val = tokenAddInput.value.trim();
      if (val && tokenContainer) {
        const newChip = document.createElement('div');
        newChip.className = 'mac-token-chip';
        newChip.innerHTML = `<span>${val}</span><span class="close-btn">×</span>`;
        tokenContainer.insertBefore(newChip, tokenContainer.lastElementChild);
        tokenAddInput.value = '';
        bindTokenCloses();
        updateTokenCode();
        showToast(`Added tag: ${val}`);
      }
    }
  });

  // ----------------------------------------------------
  // 21. MacDialog Playground
  // ----------------------------------------------------
  const dialogTitleDisp = document.getElementById('knob-dialog-title-display');
  const dialogTitleInput = document.getElementById('knob-dialog-title-input');
  const dialogDestructiveCheck = document.getElementById('knob-dialog-destructive');
  const dialogCancelBtn = document.getElementById('knob-dialog-cancel-btn');
  const dialogActionBtn = document.getElementById('knob-dialog-action-btn');
  const dialogCode = document.getElementById('dynamic-code-dialog');

  function updateDialog() {
    const title = dialogTitleInput?.value || 'Remove Photo Index?';
    const isDestructive = dialogDestructiveCheck?.checked ?? true;

    if (dialogTitleDisp) dialogTitleDisp.textContent = title;
    if (dialogActionBtn) {
      if (isDestructive) dialogActionBtn.className = 'mac-button destructive';
      else dialogActionBtn.className = 'mac-button prominent';
    }

    if (dialogCode) {
      dialogCode.textContent = `MacDialog::alert("confirm-delete", "${title}")\n    .message("This removes the local embedding index from disk.")\n    .destructive(${isDestructive})\n    .cancel_button("Cancel")\n    .action_button("Remove")\n    .on_action(cx.listener(|this, _event, cx| {\n        this.purge_cache();\n    }))`;
    }
  }

  dialogTitleInput?.addEventListener('input', updateDialog);
  dialogDestructiveCheck?.addEventListener('change', updateDialog);
  dialogCancelBtn?.addEventListener('click', () => showToast('Dialog cancelled'));
  dialogActionBtn?.addEventListener('click', () => showToast('Dialog action executed'));

  // ----------------------------------------------------
  // 22. MacPopover Playground
  // ----------------------------------------------------
  const popoverCard = document.querySelector('.mac-popover-card');
  const popoverWidthInput = document.getElementById('knob-popover-w');
  const popoverCode = document.getElementById('dynamic-code-popover');

  function updatePopover() {
    if (!popoverCard) return;
    const w = parseInt(popoverWidthInput?.value || '240', 10);
    popoverCard.style.width = `${w}px`;

    if (popoverCode) {
      popoverCode.textContent = `MacPopover::new("histogram-popover")\n    .width(${w}.0)\n    .title("Histogram Inspector")\n    .content(histogram_chart_view())\n    .footer("ISO 100 • 35mm • ƒ/2.8 • 1/500s")`;
    }
  }

  popoverWidthInput?.addEventListener('input', updatePopover);

  // ----------------------------------------------------
  // 23. MacPanel Playground
  // ----------------------------------------------------
  const panelTarget = document.getElementById('knob-target-panel');
  const panelTitleDisp = document.getElementById('knob-panel-title-disp');
  const panelTitleInput = document.getElementById('knob-panel-title-input');
  const panelHudCheck = document.getElementById('knob-panel-hud');
  const panelCode = document.getElementById('dynamic-code-panel');

  function updatePanel() {
    if (!panelTarget) return;
    const title = panelTitleInput?.value || 'Adjustments Inspector';
    const isHud = panelHudCheck?.checked || false;

    if (panelTitleDisp) panelTitleDisp.textContent = title;
    if (isHud) panelTarget.classList.add('hud');
    else panelTarget.classList.remove('hud');

    if (panelCode) {
      panelCode.textContent = `MacPanel::new("adjustments-inspector", "${title}")\n    .hud_style(${isHud})\n    .closable(true)\n    .row("Vibrance", "+24")\n    .row("Highlights", "-12")`;
    }
  }

  panelTitleInput?.addEventListener('input', updatePanel);
  panelHudCheck?.addEventListener('change', updatePanel);

  // ----------------------------------------------------
  // 24. MacPageControl Playground
  // ----------------------------------------------------
  const pageContainer = document.getElementById('knob-target-pagecontrol');
  const pageStatus = document.getElementById('knob-page-status');
  const pageTotalInput = document.getElementById('knob-page-total');
  const pageCurrInput = document.getElementById('knob-page-curr');
  const pageCode = document.getElementById('dynamic-code-pagecontrol');

  function updatePageControl() {
    if (!pageContainer) return;
    const total = Math.max(1, Math.min(10, parseInt(pageTotalInput?.value || '5', 10)));
    const curr = Math.max(1, Math.min(total, parseInt(pageCurrInput?.value || '2', 10)));

    let html = '';
    for (let i = 1; i <= total; i++) {
      html += `<div class="mac-page-dot ${i === curr ? 'active' : ''}" data-page="${i}"></div>`;
    }
    pageContainer.innerHTML = html;

    if (pageStatus) pageStatus.textContent = `Page ${curr} of ${total}`;

    pageContainer.querySelectorAll('.mac-page-dot').forEach(dot => {
      dot.addEventListener('click', () => {
        const p = parseInt(dot.getAttribute('data-page'), 10);
        if (pageCurrInput) pageCurrInput.value = p;
        updatePageControl();
        showToast(`Page ${p}`);
      });
    });

    if (pageCode) {
      pageCode.textContent = `MacPageControl::new("gallery-pager", ${total})\n    .current_page(${curr - 1})\n    .on_change(cx.listener(|this, page, cx| {\n        this.active_slide = page;\n    }))`;
    }
  }

  pageTotalInput?.addEventListener('input', updatePageControl);
  pageCurrInput?.addEventListener('input', updatePageControl);

  // ----------------------------------------------------
  // 25. MacWindow Playground
  // ----------------------------------------------------
  const winTitleDisp = document.getElementById('knob-win-title-disp');
  const winSubDisp = document.getElementById('knob-win-sub-disp');
  const winTitleInput = document.getElementById('knob-win-title-input');
  const winSubInput = document.getElementById('knob-win-sub-input');
  const winCode = document.getElementById('dynamic-code-win');

  function updateWindow() {
    const title = winTitleInput?.value || 'Loupe — RAW Canvas';
    const sub = winSubInput?.value || 'Edited — DSC04921.ARW';

    if (winTitleDisp) winTitleDisp.textContent = title;
    if (winSubDisp) winSubDisp.textContent = sub;

    if (winCode) {
      winCode.textContent = `MacWindow::new("canvas-win")\n    .title("${title}")\n    .subtitle("${sub}")\n    .traffic_lights(true)\n    .content(photo_canvas_view())`;
    }
  }

  winTitleInput?.addEventListener('input', updateWindow);
  winSubInput?.addEventListener('input', updateWindow);

  // ----------------------------------------------------
  // 26. MacProgress Playground
  // ----------------------------------------------------
  const progSlider = document.getElementById('knob-prog-slider');
  const progIndetCheck = document.getElementById('knob-prog-indet');
  const progPctDisp = document.getElementById('knob-prog-pct');
  const progBarContainer = document.getElementById('knob-target-prog-bar');
  const progFillTarget = document.getElementById('knob-target-prog-fill');
  const progCode = document.getElementById('dynamic-code-prog');

  function updateProgress() {
    const val = parseInt(progSlider?.value || '60', 10);
    const isIndet = progIndetCheck?.checked || false;

    if (isIndet) {
      if (progPctDisp) progPctDisp.textContent = 'Indeterminate';
      if (progBarContainer) progBarContainer.classList.add('indeterminate');
    } else {
      if (progPctDisp) progPctDisp.textContent = `${val}%`;
      if (progBarContainer) progBarContainer.classList.remove('indeterminate');
      if (progFillTarget) {
        progFillTarget.style.width = `${val}%`;
      }
    }

    if (progCode) {
      progCode.textContent = `MacProgress::new("export-progress")\n    .indeterminate(${isIndet})\n    .value(${(val / 100).toFixed(2)})\n    .label("Exporting JPEG Proxies...")`;
    }
  }

  progSlider?.addEventListener('input', updateProgress);
  progIndetCheck?.addEventListener('change', updateProgress);

  // ----------------------------------------------------
  // 27. MacGauge Playground
  // ----------------------------------------------------
  const gaugeSlider = document.getElementById('knob-gauge-slider');
  const gaugeValDisp = document.getElementById('knob-gauge-val-disp');
  const gaugeFill = document.getElementById('knob-target-gauge-fill');
  const gaugeCode = document.getElementById('dynamic-code-gauge');

  function updateGauge() {
    const val = parseInt(gaugeSlider?.value || '82', 10);
    if (gaugeValDisp) gaugeValDisp.textContent = `${val}%`;
    if (gaugeFill) {
      gaugeFill.style.width = `${val}%`;
      if (val >= 90) gaugeFill.style.backgroundColor = 'var(--apple-system-red)';
      else if (val >= 70) gaugeFill.style.backgroundColor = 'var(--apple-system-orange)';
      else gaugeFill.style.backgroundColor = 'var(--apple-accent)';
    }

    if (gaugeCode) {
      gaugeCode.textContent = `MacGauge::new("disk-gauge", ${(val / 100).toFixed(2)})\n    .label("Scratch Disk Cache")\n    .thresholds(0.70, 0.90)`;
    }
  }

  gaugeSlider?.addEventListener('input', updateGauge);

  // ----------------------------------------------------
  // 28. MacRatingIndicator Playground
  // ----------------------------------------------------
  const ratingTarget = document.getElementById('knob-target-rating');
  const ratingLabel = document.getElementById('knob-rating-label');
  const ratingSelect = document.getElementById('knob-rating-select');
  const ratingCode = document.getElementById('dynamic-code-rating');

  function setRating(score) {
    const stars = ratingTarget?.querySelectorAll('.mac-rating-star') || [];
    stars.forEach((star, idx) => {
      if (idx < score) star.classList.add('filled');
      else star.classList.remove('filled');
    });
    if (ratingLabel) ratingLabel.textContent = `${score} of 5 Stars`;
    if (ratingSelect) ratingSelect.value = String(score);

    if (ratingCode) {
      ratingCode.textContent = `MacRatingIndicator::new("photo-rating", ${score})\n    .max_rating(5)\n    .on_rate(cx.listener(|this, stars, cx| {\n        this.photo_stars = stars;\n    }))`;
    }
  }

  if (ratingTarget) {
    const stars = ratingTarget.querySelectorAll('.mac-rating-star');
    stars.forEach((star, idx) => {
      star.addEventListener('click', () => {
        setRating(idx + 1);
        showToast(`Rated: ${idx + 1} Stars`);
      });
    });
    ratingSelect?.addEventListener('change', () => setRating(parseInt(ratingSelect.value, 10)));
  }

  // ----------------------------------------------------
  // 29. MacActivityRings Playground
  // ----------------------------------------------------
  const ringsSvg = document.querySelector('.mac-rings-svg');
  const ringsSizeInput = document.getElementById('knob-rings-size');
  const ringsCode = document.getElementById('dynamic-code-rings');

  function updateRings() {
    const size = parseInt(ringsSizeInput?.value || '80', 10);
    if (ringsSvg) {
      ringsSvg.style.width = `${size}px`;
      ringsSvg.style.height = `${size}px`;
    }

    if (ringsCode) {
      ringsCode.textContent = `MacActivityRings::new("perf-rings")\n    .size(${size}.0)\n    .ring("CPU", 0.68, gpui::rgb(0xFF3B30))\n    .ring("GPU", 0.65, gpui::rgb(0x34C759))\n    .ring("RAM", 0.80, gpui::rgb(0x007AFF))`;
    }
  }

  ringsSizeInput?.addEventListener('input', updateRings);

  // ----------------------------------------------------
  // 30. MacBadge Playground
  // ----------------------------------------------------
  const badgeTarget = document.getElementById('knob-target-badge');
  const badgeVariantSelect = document.getElementById('knob-badge-variant');
  const badgeTextInput = document.getElementById('knob-badge-text');
  const badgeCode = document.getElementById('dynamic-code-badge');

  function updateBadge() {
    if (!badgeTarget) return;
    const variant = badgeVariantSelect?.value || 'accent';
    const text = badgeTextInput?.value || '4,281 items indexed';

    badgeTarget.className = `mac-badge ${variant}`;
    badgeTarget.textContent = text;

    if (badgeCode) {
      let varPascal = variant.charAt(0).toUpperCase() + variant.slice(1);
      badgeCode.textContent = `MacBadge::new("catalog-count", "${text}")\n    .variant(MacBadgeVariant::${varPascal})`;
    }
  }

  badgeVariantSelect?.addEventListener('change', updateBadge);
  badgeTextInput?.addEventListener('input', updateBadge);

  // ----------------------------------------------------
  // 31. MacTableView Playground & Zebra Striping
  // ----------------------------------------------------
  const tableTarget = document.getElementById('knob-target-table') || document.querySelector('.mac-table-card');
  const tableZebraCheck = document.getElementById('knob-table-zebra');
  const tableCode = document.getElementById('dynamic-code-table');

  function updateTableView() {
    if (!tableTarget) return;
    const isZebra = tableZebraCheck?.checked ?? true;

    if (isZebra) {
      tableTarget.classList.remove('no-zebra');
    } else {
      tableTarget.classList.add('no-zebra');
    }

    const rows = tableTarget.querySelectorAll('.mac-table-row');
    let selectedIdx = -1;
    rows.forEach((r, idx) => {
      if (r.classList.contains('selected')) selectedIdx = idx;
    });

    if (tableCode) {
      tableCode.textContent = `MacTableView::new("photo-library-table")\n    .zebra_striping(${isZebra})\n    .column("Name", 140.0)\n    .column("Size", 80.0)\n    .column("Format", 100.0)\n    .row(vec!["DSC04921.ARW", "34.2 MB", "Sony RAW"])\n    .row(vec!["DSC04922.ARW", "32.8 MB", "Sony RAW"])\n    .row(vec!["IMG_0084.CR3", "41.0 MB", "Canon RAW"])\n    .selected_row(${selectedIdx >= 0 ? `Some(${selectedIdx})` : 'None'})\n    .on_select(cx.listener(|this, row_idx, cx| {\n        this.handle_row_select(row_idx);\n    }))`;
    }
  }

  if (tableTarget) {
    tableTarget.querySelectorAll('.mac-table-row').forEach((row, idx) => {
      row.addEventListener('click', () => {
        tableTarget.querySelectorAll('.mac-table-row').forEach(r => r.classList.remove('selected'));
        row.classList.add('selected');
        updateTableView();
        const name = row.querySelector('span:first-child')?.textContent.trim() || `Row ${idx + 1}`;
        showToast(`Selected: ${name}`);
      });
    });
    tableZebraCheck?.addEventListener('change', () => {
      updateTableView();
      showToast(tableZebraCheck.checked ? 'Zebra striping enabled' : 'Zebra striping disabled');
    });
  }

  // ----------------------------------------------------
  // 32. MacSplitView Playground
  // ----------------------------------------------------
  const splitWidthSlider = document.getElementById('knob-split-width');
  const splitFirstPane = document.querySelector('.mac-split-demo > div:first-child');
  const splitCode = document.getElementById('dynamic-code-split');

  function updateSplitView() {
    const w = splitWidthSlider?.value || '140';
    if (splitFirstPane) splitFirstPane.style.width = `${w}px`;
    if (splitCode) {
      splitCode.textContent = `MacSplitView::new("main-split")\n    .leading_width(${w}.0)\n    .leading(MacSidebar::new("sidebar"))\n    .trailing(photo_canvas)`;
    }
  }

  splitWidthSlider?.addEventListener('input', updateSplitView);

  // ----------------------------------------------------
  // 33. MacOutlineView Playground
  // ----------------------------------------------------
  const outlineTarget = document.getElementById('knob-target-outline') || document.querySelector('.mac-outline-demo');
  const outlineCode = document.getElementById('dynamic-code-outline');

  if (outlineTarget) {
    const outlineRows = outlineTarget.querySelectorAll('.mac-outline-row');
    outlineRows.forEach((row, idx) => {
      row.addEventListener('click', () => {
        outlineRows.forEach(r => r.classList.remove('selected'));
        row.classList.add('selected');
        const name = row.querySelector('span:last-child')?.textContent.trim() || `Item ${idx}`;
        if (outlineCode) {
          outlineCode.textContent = `MacOutlineView::new("library-outline")\n    .section("LIBRARY")\n    .item("All Photos", "icon-photos")\n    .item("Favorites", "icon-star")\n    .section("FOLDERS")\n    .item("2026 Yosemite", "icon-folder")\n    .selected_row(Some(${idx}))\n    .on_select(cx.listener(|this, item, cx| {\n        this.navigate_folder(item);\n    }))`;
        }
        showToast(`Selected: ${name}`);
      });
    });
  }

  // ----------------------------------------------------
  // 34. MacColumnView Playground
  // ----------------------------------------------------
  const colWidthInput = document.getElementById('knob-col-w');
  const columnPanes = document.querySelectorAll('.mac-column-demo .mac-column-pane');
  const columnCode = document.getElementById('dynamic-code-column');

  function updateColumnView() {
    const w = parseInt(colWidthInput?.value || '140', 10);
    columnPanes.forEach(pane => {
      pane.style.width = `${w}px`;
    });

    if (columnCode) {
      columnCode.textContent = `MacColumnView::new("finder-columns")\n    .column_width(${w}.0)\n    .pane(vec!["📁 Pictures", "📁 Documents"])\n    .pane(vec!["📁 2026", "📁 2025"])\n    .pane(vec!["📄 DSC04921.ARW"])\n    .on_select(cx.listener(|this, col, item, cx| {\n        this.navigate_column(col, item);\n    }))`;
    }
  }

  colWidthInput?.addEventListener('input', updateColumnView);
  document.querySelectorAll('.mac-column-row').forEach(row => {
    row.addEventListener('click', () => {
      const pane = row.closest('.mac-column-pane');
      pane?.querySelectorAll('.mac-column-row').forEach(r => r.classList.remove('selected'));
      row.classList.add('selected');
      showToast(`Selected: ${row.textContent.trim()}`);
    });
  });

  // ----------------------------------------------------
  // 35. MacTextView Playground
  // ----------------------------------------------------
  const tvLinesCheck = document.getElementById('knob-tv-lines');
  const tvGutter = document.querySelector('.mac-textview-gutter');
  const tvCode = document.getElementById('dynamic-code-textview');

  function updateTextView() {
    const showLines = tvLinesCheck?.checked ?? true;
    if (tvGutter) tvGutter.style.display = showLines ? 'block' : 'none';

    if (tvCode) {
      tvCode.textContent = `MacTextView::new("notes-editor")\n    .line_numbers(${showLines})\n    .editable(true)\n    .font_size(13.0)\n    .on_change(cx.listener(|this, text, cx| {\n        this.notes_content = text.to_string();\n    }))`;
    }
  }

  tvLinesCheck?.addEventListener('change', updateTextView);

  // ----------------------------------------------------
  // 36. MacImageView Playground
  // ----------------------------------------------------
  const imgRadiusInput = document.getElementById('knob-img-radius');
  const imgBoxTarget = document.querySelector('#image-view .playground-stage div > div:first-child');
  const imgCode = document.getElementById('dynamic-code-imageview');

  function updateImageView() {
    const r = parseInt(imgRadiusInput?.value || '8', 10);
    if (imgBoxTarget) imgBoxTarget.style.borderRadius = `${r}px`;

    if (imgCode) {
      imgCode.textContent = `MacImageView::new("hero-image")\n    .source("yosemite_sunrise.raw")\n    .corner_radius(${r}.0)\n    .aspect_ratio(16.0 / 10.0)\n    .shadow(true)`;
    }
  }

  imgRadiusInput?.addEventListener('input', updateImageView);

  // ----------------------------------------------------
  // 37. MacChart Playground
  // ----------------------------------------------------
  const chartPeakSlider = document.getElementById('knob-chart-peak');
  const chartWedBar = document.querySelector('.mac-chart-bar-item:nth-child(3) .mac-chart-bar-rect');
  const chartCode = document.getElementById('dynamic-code-chart');

  function updateChart() {
    const h = chartPeakSlider?.value || '90';
    if (chartWedBar) chartWedBar.style.height = `${h}px`;

    if (chartCode) {
      chartCode.textContent = `MacChart::new("ingest-chart")\n    .title("Weekly Photo Ingest Volume")\n    .point("Mon", 0.40)\n    .point("Tue", 0.70)\n    .point_colored("Wed", ${(h / 100).toFixed(2)}, MacAccentColor::Green)\n    .point("Thu", 0.50)\n    .point("Fri", 0.85)`;
    }
  }

  chartPeakSlider?.addEventListener('input', updateChart);

  // ----------------------------------------------------
  // Initial Synchronization across all playgrounds
  // ----------------------------------------------------
  updateSwitch();
  updateButton();
  updateCheckbox();
  updateRadio();
  updateComboBox();
  updateDigitEntry();
  updateImageWell();
  updateDatePicker();
  updateSegmented();
  updateSlider();
  updateStepper();
  updateColor();
  updateTextField();
  updatePopup();
  updateContextMenu();
  updateMenuBar();
  updateSearchField();
  updateTabBar();
  updatePathControl();
  updateTokenCode();
  updateDialog();
  updatePopover();
  updatePanel();
  updatePageControl();
  updateWindow();
  updateProgress();
  updateGauge();
  setRating(4);
  updateRings();
  updateBadge();
  updateTableView();
  updateSplitView();
  updateColumnView();
  updateTextView();
  updateImageView();
  updateChart();
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
