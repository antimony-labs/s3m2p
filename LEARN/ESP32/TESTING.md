# Testing the Electronics Course

## Quick Start

### 1. Build and Run the Electronics Course

```bash
# From the repo root
cd LEARN/ESP32
trunk serve --port 8104 --open
```

Or use the dev script:
```bash
./dev up esp32
```

The course will be available at: **http://localhost:8104**

---

## Testing Checklist

### ✅ Home Page
- [ ] Home page shows "Electronics" title (not "ESP32")
- [ ] Subtitle mentions "From Basic Circuits to ESP32 Capstone"
- [ ] Lessons are grouped by phase:
  - Phase 0: The Promise + Safety
  - Phase 1: DC Circuits
  - Phase 2: Components
  - Phase 3: Microcontrollers
  - Phase 4: ESP32 Deep Dive
  - Phase 5: Capstone
- [ ] All 21 lesson cards are visible
- [ ] Clicking a lesson card navigates to that lesson

### ✅ Lesson Pages
- [ ] Each lesson displays:
  - Icon, title, subtitle
  - "Why It Matters" section
  - "The Idea" section with intuition
  - Demo section (if applicable)
  - Key Takeaways
  - Going Deeper (expandable)
  - Math Details (expandable)
  - Implementation Guide (expandable)
- [ ] Previous/Next navigation works
- [ ] Back button returns to home

### ✅ Interactive Demos

#### Lesson 2: Ohm's Law + Power
- [ ] Demo canvas appears
- [ ] Voltage slider works (0-12V)
- [ ] Resistance slider works (10-10000Ω)
- [ ] Current and power are calculated correctly
- [ ] Visual indicators show when limits are exceeded
- [ ] Reset and Pause buttons work

#### Lesson 5: RC Time Constant
- [ ] Demo canvas appears
- [ ] Resistance slider works (1k-100kΩ)
- [ ] Capacitance slider works (1-1000µF)
- [ ] Charging curve animates
- [ ] τ (tau) marker is visible
- [ ] Auto-resets when fully charged

#### Lesson 8: GPIO Debounce
- [ ] Demo canvas appears
- [ ] All controls work (bounce severity, sample rate, etc.)
- [ ] Timeline shows raw vs debounced signals

#### Lesson 9: PWM Control
- [ ] Demo canvas appears
- [ ] Duty, frequency, resolution sliders work
- [ ] Waveform updates in real-time

#### Lesson 10: ADC Reading
- [ ] Demo canvas appears
- [ ] All ADC controls work
- [ ] Noise and averaging effects visible

#### Lesson 11: I²C Communication
- [ ] Demo canvas appears
- [ ] I²C waveform displays correctly
- [ ] ACK/NACK indicators work

#### Lesson 19: Power Budget
- [ ] Demo canvas appears
- [ ] All sliders work (active current, active time, sleep current, sleep time)
- [ ] Battery lifetime calculation updates
- [ ] Energy per cycle bar chart displays

### ✅ Static Lessons
- [ ] Lessons without demos (Static type) don't show canvas
- [ ] Calculator lessons show calculator widget placeholder
- [ ] Content renders correctly

### ✅ Math Rendering
- [ ] KaTeX formulas render correctly (look for `$$` blocks)
- [ ] Mermaid diagrams render correctly
- [ ] Diagrams switch theme with light/dark mode

### ✅ Implementation Guides
- [ ] Implementation Guide section is expandable
- [ ] Code blocks are formatted correctly
- [ ] Lab exercises are numbered correctly
- [ ] LLM prompts are in code blocks

### ✅ Navigation & Redirects

#### WELCOME Page
- [ ] Navigate to http://localhost:8080 (or your WELCOME port)
- [ ] Click "Learn" category
- [ ] Verify only **one** "Electronics" bubble (not separate Arduino/ESP32)
- [ ] Click Electronics → should go to ESP32 course

#### Arduino Redirect Page
- [ ] Navigate to http://localhost:8103 (Arduino port)
- [ ] Page shows "Arduino Content Moved" message
- [ ] Auto-redirects to Electronics course after 3 seconds
- [ ] Manual "Go to Electronics Course" link works

---

## Manual Testing Commands

### Build Only (No Server)
```bash
cd LEARN/ESP32
trunk build
```

### Check for Compilation Errors
```bash
cargo check --workspace
```

### Run Unit Tests
```bash
cargo test --workspace
```

### Check Linter
```bash
cargo clippy --workspace
```

---

## Common Issues & Fixes

### Demo Not Showing
- Check browser console for errors
- Verify lesson ID matches demo runner mapping
- Ensure canvas element exists in DOM

### Math Not Rendering
- Check that KaTeX script loaded
- Verify `renderKaTeX()` is called after DOM update
- Check for syntax errors in math blocks

### Phase Grouping Not Working
- Verify lessons have `phase` field set
- Check renderer groups by phase correctly
- Ensure phase names match exactly

### Navigation Broken
- Check hash routing (`#/lesson/N`)
- Verify `go_to_lesson()` function exists
- Check browser console for JS errors

---

## Browser Testing

Test in multiple browsers:
- [ ] Chrome/Chromium
- [ ] Firefox
- [ ] Safari (if on macOS)
- [ ] Mobile viewport (responsive design)

---

## Performance Checks

- [ ] Page loads quickly (< 2s)
- [ ] Demos run smoothly (60fps)
- [ ] No memory leaks (check DevTools Memory)
- [ ] WASM bundle size reasonable

---

## Accessibility

- [ ] Keyboard navigation works
- [ ] Screen reader friendly (check with NVDA/JAWS)
- [ ] Color contrast sufficient
- [ ] Focus indicators visible

