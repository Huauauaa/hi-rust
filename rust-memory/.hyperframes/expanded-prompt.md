# Rust 内存 — Expanded Production Prompt

## Title + Style

**Title:** Rust 内存安全入门
**Palette:** bg `#0D1B2A`, fg `#E0E1DD`, accent `#CE422B`, code `#1B263B`, highlight `#F4A261`
**Fonts:** IBM Plex Sans (headlines/body), JetBrains Mono (code)
**Mood:** Clean technical explainer — precise, confident, developer audience

## Rhythm

`hook-PUNCH-explain-explain-explain-CTA` — fast title, then steady educational beats, calm close.

## Global Rules

- Persistent dark navy background across all scenes
- 2–4 decoratives per scene: radial glow, ghost "OWNERSHIP" text, subtle grid, accent hairline
- Ambient breath on glows (scale 1→1.04, 4s, yoyo finite)
- Transitions: push slide (energy), blur crossfade (concept shifts), focus pull (code scene)
- No exit tweens before transitions — transitions handle exits

## Scene 1 — Hook (0–5s)

**Concept:** Viewer lands in a dark dev workspace. "Rust 内存" SLAMS in — this isn't C's wild pointers, it's a system with rules.
**Depth:** BG radial rust glow + ghost "SAFE"; MG title + subtitle; FG accent line + crab emoji accent
**Choreography:** title SLAMS (y:80→0, expo.out), subtitle CASCADE, decoratives breathe
**Transition @5s:** Push slide right

## Scene 2 — Stack vs Heap (5–10s)

**Concept:** Two columns — stack (fast, fixed) vs heap (dynamic, allocated). Visual metaphor: stacked blocks vs floating heap nodes.
**Choreography:** column labels SLIDE in, stack blocks DROP staggered, heap node FLOATS
**Transition @10s:** Blur crossfade

## Scene 3 — Ownership (10–15s)

**Concept:** One owner per value. Diagram: box with owner label, arrow to value. "离开作用域 → drop"
**Choreography:** rule text FADES in, diagram DRAWS, drop label STAMPS
**Transition @15s:** Focus pull

## Scene 4 — Move (15–20s)

**Concept:** `let s1 = String::from("hi"); let s2 = s1;` — ownership transfers, s1 invalidated.
**Choreography:** code block SLIDES up, move arrow ANIMATES, strikethrough on s1
**Transition @20s:** Blur crossfade

## Scene 5 — Borrowing (20–25s)

**Concept:** `&T` shared borrow, `&mut T` exclusive borrow — read many, write one.
**Choreography:** two cards CASCADE, icons FADE in, rule line TYPES on
**Transition @25s:** Push slide

## Scene 6 — CTA (25–30s)

**Concept:** "hi-rust 系列" — continue learning. Calm hold, fade to black at end.
**Choreography:** series title FADES in, URL/link FLOATS, final fade to black (only exit allowed)

## Negative Prompt

No cyan-on-dark, no gradient text, no purple-blue gradients, no lazy #333/#3b82f6, no exit tweens before transitions.
