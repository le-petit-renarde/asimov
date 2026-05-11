//! Professor Mascot — Procedurally generated ASCII art
//!
//! Generates ASCII art representations of "Professor" - the Asimov mascot.
//! A distinguished scholar with glasses and a pipe.

/// ASCII art frames for the Professor mascot
pub mod frames {
    /// Idle frame - peaceful, scholarly
    pub const IDLE: &str = r#"
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
        │      │      ┌─────┴────┐   │      │
        │      │      │  BOOK    │   │      │  ← Leather book
        │      │      └──────────┘   │      │
        │      └────────────────────┘      │
        │                                  │
        ╰──────────────────────────────────╯
                    "Omniscient Companion"
"#;

    /// Thinking frame - contemplative
    pub const THINKING: &str = r#"
        ╭──────────────────────────────────╮
        │                                  │
        │      ┌────────────────────┐      │
        │      │   ╭───────────╮    │      │
        │      │   │  ◉    ◉   │    │      │  ← Focused eyes
        │      │   ╰───────────╯    │      │
        │      │       ‿‿‿         │      │  ← Furrowed brow
        │      │      ═══╧══       │      │
        │      │    ╭───────────╮   │      │
        │      │    │ ▓▓▓▓▓▓▓▓▓ │   │      │
        │      │    │ ░░VEST░░░░ │   │      │
        │      │    │ ▓▓▓▓▓▓▓▓▓ │   │      │
        │      │    ╰─────┬─────╯   │      │
        │      │        ┌─┴─┐       │      │
        │      │        │ ~~~│      │      │  ← Pensive smoke
        │      │        └─┬─┘       │      │
        │      │      ╭───┴───╮     │      │
        │      │      │  IDEA │     │      │  ← Lightbulb moment
        │      │      └───────┘     │      │
        │      └────────────────────┘      │
        │                                  │
        ╰──────────────────────────────────╯
                    "Let me think..."
"#;

    /// Searching frame - alert, seeking
    pub const SEARCHING: &str = r#"
        ╭──────────────────────────────────╮
        │                                  │
        │      ┌────────────────────┐      │
        │      │   ╭───────────╮    │      │
        │      │   │  ⊙    ⊙   │    │      │  ← Wide scanning eyes
        │      │   ╰───────────╯    │      │
        │      │       ═══╧══        │      │
        │      │    ╭───────────╮   │      │
        │      │    │ ▓▓▓▓▓▓▓▓▓ │   │      │
        │      │    │ ░░VEST░░░░ │   │      │
        │      │    │ ▓▓▓▓▓▓▓▓▓ │   │      │
        │      │    ╰─────┬─────╯   │      │
        │      │        ┌─┴─┐       │      │
        │      │     ┌──│ ~ │──┐   │      │  ← Active smoke
        │      │     │  └─┬─┘  │   │      │
        │      │     │    │    │   │      │
        │      │     │  ╔═╧═╗  │   │      │  ← Magnifying glass
        │      │     │  ║ ? ║  │   │      │
        │      │     │  ╚═══╝  │   │      │
        │      └─────│─────────│─────────╯
        │                                  │
        ╰──────────────────────────────────╯
                    "Searching..."
"#;

    /// Happy frame - pleased
    pub const HAPPY: &str = r#"
        ╭──────────────────────────────────╮
        │                                  │
        │      ┌────────────────────┐      │
        │      │   ╭───────────╮    │      │
        │      │   │  ◠    ◠   │    │      │  ← Happy eyes (^‿^)
        │      │   ╰───────────╯    │      │
        │      │       ω           │      │
        │      │      ══█══        │      │  ← Contented smile
        │      │    ╭───────────╮   │      │
        │      │    │ ▓▓▓▓▓▓▓▓▓ │   │      │
        │      │    │ ░░VEST░░░░ │   │      │
        │      │    │ ▓▓▓▓▓▓▓▓▓ │   │      │
        │      │    ╰─────┬─────╯   │      │
        │      │        ┌─┴─┐       │      │
        │      │       ~│~~~│~      │      │  ← Satisfied smoke
        │      │        └─┬─┘       │      │
        │      │      ╭───┴───╮     │      │
        │      │      │ ✓ YES │     │      │  ← Thumbs up!
        │      │      └───────┘     │      │
        │      └────────────────────┘      │
        │                                  │
        ╰──────────────────────────────────╯
                    "Excellent!"
"#;
}

/// Generate a frame with custom pose
pub fn generate_frame(pose: &str) -> &'static str {
    match pose {
        "idle" | "" => frames::IDLE,
        "thinking" => frames::THINKING,
        "searching" => frames::SEARCHING,
        "happy" | "pleased" => frames::HAPPY,
        _ => frames::IDLE,
    }
}

/// Get all frame names
pub fn frame_names() -> Vec<&'static str> {
    vec!["idle", "thinking", "searching", "happy"]
}

/// Animated sprite sheet as text (simplified)
pub fn sprite_sheet() -> &'static str {
    r#"
┌─────────────┬─────────────┬─────────────┬─────────────┐
│             │             │             │             │
│    IDLE     │  THINKING   │  SEARCHING  │   HAPPY     │
│             │             │             │             │
│   ◔    ◔    │   ◉    ◉    │   ⊙    ⊙    │   ◠    ◠    │
│             │             │             │             │
│    ‿‿‿      │    ‿‿‿      │    ═══      │     █       │
│             │             │             │             │
│      ~      │     ~~~     │     ~~~     │     ~~~     │
│             │             │             │             │
└─────────────┴─────────────┴─────────────┴─────────────┘
    "#
}

/// Print ASCII mascot to console
pub fn print_mascot(pose: &str) {
    println!("{}", generate_frame(pose));
}

/// Print all frames in sequence
pub fn print_animation() {
    for frame in frame_names() {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char); // Clear screen
        println!("\n=== PROFESSOR ({}) ===\n", frame.to_uppercase());
        print_mascot(frame);
        println!("\n");
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_frame_generation() {
        assert!(!generate_frame("idle").is_empty());
        assert!(!generate_frame("thinking").is_empty());
        assert_eq!(generate_frame("unknown"), generate_frame("idle"));
    }
    
    #[test]
    fn test_frame_names() {
        let names = frame_names();
        assert!(names.contains(&"idle"));
        assert!(names.contains(&"thinking"));
        assert!(names.contains(&"searching"));
    }
}
