# Professor — Asimov's Mascot

## Concept Description

A distinguished scholar figure — think classic detective meets university professor — representing the omniscient, thoughtful nature of Asimov.

## Programmatic Generation

The mascot is generated programmatically using the `generate_mascot` example in the search crate:

```bash
cd src-rust/crates/search
cargo run --example generate_mascot
```

This generates PNG images in the `public/` directory:
- `Professor.png` - Main idle frame
- `Professor-Think.png` - Thinking variant
- `Professor-Search.png` - Searching variant
- `Professor-Sprite.png` - Horizontal sprite sheet
- `Asimov-Logo.png` - Simple logo variant

## ASCII Art (Built-in)

The mascot is also available as ASCII art directly in the source code:

```rust
use asimov_search::mascot;

// Print a specific pose
mascot::print_mascot("idle");
mascot::print_mascot("thinking");
mascot::print_mascot("searching");
mascot::print_mascot("happy");

// Get frame by name
let frame = mascot::generate_frame("idle");

// Print animation sequence
mascot::print_animation();

// Get sprite sheet
let sprites = mascot::sprite_sheet();
```

### ASCII Art Frames

```
╭──────────────────────────────────╮
│                                  │
│      ┌────────────────────┐      │
│      │   ╭───────────╮    │      │
│      │   │  ◔    ◔   │    │      │  ← Round glasses
│      │   ╰───────────╯    │      │
│      │        ω          │      │  ← Distinguished nose
│      │      ══╧══        │      │  ← Handlebar mustache
│      │    ╭───────────╮   │      │
│      │    │ ▓▓▓▓▓▓▓▓▓ │   │      │  ← White collar
│      │    │ ░░VEST░░░░ │   │      │  ← Brown vest
│      │    │ ▓▓▓▓▓▓▓▓▓ │   │      │
│      │    ╰─────┬─────╯   │      │
│      │        ┌─┴─┐       │      │
│      │        │ ~ │       │      │  ← Pipe with smoke
│      │        └─┬─┘       │      │
│      │      ┌───┴───╮     │      │
│      │      │  BOOK │     │      │  ← Leather book
│      │      └───────┘     │      │
│      └────────────────────┘      │
│                                  │
╰──────────────────────────────────╯
                    "Omniscient Companion"
```

## Color Palette (Rust Constants)

```rust
mod colors {
    pub const BACKGROUND: Rgba<u8> = Rgba([26, 26, 46, 255]);       // Dark blue-purple
    pub const SKIN: Rgba<u8> = Rgba([245, 230, 211, 255]);          // Warm peach
    pub const GLASSES_FRAME: Rgba<u8> = Rgba([139, 115, 85, 255]); // Antique gold
    pub const GLASSES_LENS: Rgba<u8> = Rgba([200, 200, 220, 100]); // Translucent blue
    pub const SHIRT: Rgba<u8> = Rgba([245, 245, 245, 255]);        // Crisp white
    pub const VEST: Rgba<u8> = Rgba([139, 69, 19, 255]);            // Warm brown
    pub const PIPE: Rgba<u8> = Rgba([222, 184, 135, 255]);         // Burlywood
    pub const SMOKE: Rgba<u8> = Rgba([192, 192, 192, 180]);        // Soft gray
    pub const BOOK: Rgba<u8> = Rgba([101, 67, 33, 255]);           // Dark leather
    pub const HAIR: Rgba<u8> = Rgba([100, 100, 100, 255]);          // Gray hair
    pub const MUSTACHE: Rgba<u8> = Rgba([80, 60, 50, 255]);         // Dark gray-brown
    pub const ACCENT: Rgba<u8> = Rgba([255, 215, 0, 255]);          // Gold accent
}
```

## Animation Frames

### Idle Frame
- Relaxed eyes (◔ ◔)
- Slight smile (‿‿)
- Pipe still
- Light smoke wisps

### Thinking Frame
- Focused eyes (◉ ◉)
- Furrowed brow (‿‿‿)
- Pipe bobbing
- Smoke curling

### Searching Frame
- Widened eyes (⊙ ⊙)
- Determined expression
- Magnifying glass
- Active smoke

### Happy Frame
- Happy eyes (◠ ◠)
- Contented smile (█)
- Thumbs up
- Satisfied smoke

## File Naming Convention

```
public/
├── Professor.png          # Main idle frame (512x512)
├── Professor-Search.png   # Searching variant
├── Professor-Think.png    # Thinking variant
├── Professor-Sprite.png   # Horizontal sprite sheet for animation
├── Asimov-Logo.png        # Simple logo variant
└── social-image.png       # Updated branding
```

## Prompt for External Image Generation (if needed)

```
A distinguished elderly professor character, minimalist flat design style.
Wearing round golden wire-frame glasses, white collared shirt, brown 
cardigan vest, holding a curved wooden pipe with gentle smoke wisps.
One hand holds an open leather-bound book. Warm color palette on dark
blue-purple background. Friendly but scholarly expression. No background
details, clean flat illustration style. PNG format with transparent background.
Character centered, portrait orientation.
```
