# Oxyde Prompt Template System

## Overview

The Oxyde Prompt Template System allows you to customize NPC dialogue, emotional responses, memory formatting, and goal-driven behaviors **without touching any Rust code**. All prompts are configured via external TOML/JSON/YAML files.

---

## Quick Start

### 1. **Using Default Prompts**

The SDK ships with built-in defaults. Just create an agent:

```rust
use oxyde::{Agent, AgentConfig};

let config = AgentConfig::from_file("agent.json")?;
let agent = Agent::new(config); // Uses bundled default prompts
agent.start().await?;
```

### 2. **Customizing Prompts**

Copy the default configuration and modify it:

```bash
# Copy from SDK installation or create your own
cp config/defaults.toml my_game/prompts.toml

# Edit prompts.toml to customize
```

Then load it:

```rust
let mut config = AgentConfig::from_file("agent.json")?;
config.prompts = Some(PromptConfig::from_file("prompts.toml")?);

let agent = Agent::new(config);
```

Or place `prompts.toml` in the same directory as your agent config file, and it will load automatically!

---

## Configuration File Structure

### Supported Formats

- **TOML** (recommended): `prompts.toml`
- **JSON**: `prompts.json`
- **YAML**: `prompts.yaml` or `prompts.yml`

---

## File Sections

### 1. **System Prompts** (Character Personality)

Define how NPCs behave based on their role:

```toml
[system_prompts.merchant]
base_description = "You are {npc_name}, a merchant in a fantasy RPG..."

behavior_guidelines = [
    "Be friendly but business-minded",
    "Show interest in making deals",
    "Mention your wares occasionally"
]

response_constraints = "Keep responses under 50 words."

context_template = """
Recent conversation:
{conversation_history}

Relevant memories:
{memory_context}
"""
```

**Template Variables:**
- `{npc_name}` - Agent's name
- `{npc_role}` - Agent's role
- `{conversation_history}` - Previous messages
- `{memory_context}` - Retrieved memories

**Predefined Roles:**
- `merchant` - Business-focused trader
- `guard` - Law enforcement officer
- `villager` - Friendly local resident
- `innkeeper` - Tavern/inn owner
- `blacksmith` - Weapon/armor craftsperson

**Adding Custom Roles:**

```toml
[system_prompts.wizard]
base_description = "You are {npc_name}, an ancient wizard..."
behavior_guidelines = [
    "Speak cryptically about magic",
    "Reference ancient knowledge",
    "Be mysterious and wise"
]
response_constraints = "Use archaic language. Keep responses mystical."
context_template = "Previous interactions:\n{conversation_history}"
```

---

### 2. **Emotional Prompts** (Dynamic Reactions)

Control how emotions affect dialogue:

```toml
[emotional_prompts]
modifier_template = "{emotional_state} {response_style}"

[emotional_prompts.emotional_states]
happy = "You're in a cheerful mood and speak warmly."
angry = "You're irritated and speak curtly."
fearful = "You're nervous and speak cautiously."
# Add more emotions...

[emotional_prompts.response_styles]
friendly = "Respond in a warm, friendly manner with enthusiasm."
hostile = "Respond curtly and show irritation. Keep answers brief."
nervous = "Respond cautiously and show hesitation."
# Add more styles...
```

**How It Works:**
1. The emotion engine calculates the NPC's current emotional state
2. System selects the dominant emotion (e.g., "angry")
3. System selects the response style (e.g., "hostile")
4. These get combined: "You're irritated and speak curtly. Respond curtly and show irritation."

**Adding Custom Emotions:**

```toml
[emotional_prompts.emotional_states]
drunk = "You're tipsy and speak with slurred words."
exhausted = "You're tired and speak slowly with pauses."

[emotional_prompts.response_styles]
sarcastic = "Respond with witty sarcasm and dry humor."
poetic = "Respond in verse and rhyme when possible."
```

---

### 3. **Goal-Driven Prompts** (Motivations)

Define how NPC goals influence dialogue:

```toml
[goal_prompts]
goal_context_template = "Current focus: {goal_description} ({goal_type}, {priority}, {progress} complete, motivation: {motivation})"

[goal_prompts.goal_type_descriptions]
Economic = "business goal"
Social = "relationship goal"
Knowledge = "learning goal"
Protection = "security goal"
# Add more types...
```

**Example Output:**
```
Current focus: Earn 1000 gold coins this month (business goal, 
This is your highest priority, 35% complete, motivation: 8.5/10)
```

**Adding Custom Goal Types:**

```toml
[goal_prompts.goal_type_descriptions]
Conquest = "territorial expansion"
Redemption = "atonement quest"
Discovery = "exploration mission"
```

---

### 4. **Memory Formatting**

Control how memories appear in prompts:

```toml
[memory_prompts]
context_header = "Relevant memories:"
memory_format = "- [{category}] {content} {importance}"

[memory_prompts.category_labels]
Episodic = "Event"
Semantic = "Fact"
Procedural = "Skill"
Emotional = "Feeling"

[memory_prompts.importance_indicators]
high = "***"    # or "🔥" or "CRITICAL"
medium = "**"   # or "⚠️" or "IMPORTANT"
low = "*"       # or "ℹ️" or "NOTE"
```

**Example Output:**
```
Relevant memories:
- [Event] Player helped defend the town ***
- [Fact] The blacksmith owes me money **
- [Feeling] I trust this traveler *
```

**Customization Examples:**

```toml
# Emoji-based importance
[memory_prompts.importance_indicators]
high = "🔥🔥🔥"
medium = "⚡⚡"
low = "💭"

# Text-based importance
[memory_prompts.importance_indicators]
high = "[CRITICAL]"
medium = "[IMPORTANT]"
low = "[NOTE]"

# Custom categories
[memory_prompts.category_labels]
Episodic = "📖 Story"
Semantic = "🧠 Knowledge"
Procedural = "⚙️ Skill"
Emotional = "❤️ Feeling"
```

---

### 5. **Greetings & Farewells**

Random phrases when starting/ending conversations:

```toml
[greetings]
merchant = [
    "Welcome to my shop!",
    "Looking for something special today?",
    "Greetings, traveler!"
]

guard = [
    "Halt! State your business.",
    "What brings you here?",
    "Keep moving, citizen."
]

[farewells]
merchant = [
    "Come back anytime!",
    "Safe travels!",
    "Thank you for your business!"
]
```

**Usage in Code:**

```rust
let greeting = prompts.get_greeting("merchant");
// Returns: "Welcome to my shop!" (randomly selected)

let farewell = prompts.get_farewell("guard");
// Returns: "Move along now." (randomly selected)
```

---

## API Reference

### Loading Prompts

```rust
use oxyde::PromptConfig;

// Load from file
let prompts = PromptConfig::from_file("prompts.toml")?;

// Load from file with fallback to bundled defaults
let prompts = PromptConfig::from_file_or_default("prompts.toml")?;

// Use bundled defaults only
let prompts = PromptConfig::from_bundled_default()?;

// Save prompts to file
prompts.save_to_file("my_prompts.toml")?;
```

### Generating Prompts

```rust
// System prompt
let prompt = prompts.generate_system_prompt(
    "Marcus",           // NPC name
    "merchant",         // NPC role
    &conversation_history  // Previous messages
);

// Emotional modifier
let modifier = prompts.generate_emotional_modifier(
    "happy",           // Dominant emotion
    "friendly"         // Response style
);

// Goal-driven prompt
let goal_prompt = prompts.generate_goal_prompt(
    "Earn 1000 gold",  // Goal description
    "Economic",        // Goal type
    0.8,               // Priority (0.0-1.0)
    0.35,              // Progress (0.0-1.0)
    0.85               // Motivation (0.0-1.0)
);

// Memory context
let memories = vec![
    ("Player helped me".to_string(), "Episodic".to_string(), 0.9),
    ("I sell weapons".to_string(), "Semantic".to_string(), 0.7),
];
let memory_context = prompts.format_memory_context(&memories);

// Random greeting/farewell
let greeting = prompts.get_greeting("merchant");
let farewell = prompts.get_farewell("guard");
```

---

## Advanced Usage

### Per-NPC Custom Prompts

```rust
// Create unique prompts for specific NPCs
let mut marcus_prompts = PromptConfig::from_file("base_prompts.toml")?;

// Override merchant prompts for Marcus specifically
marcus_prompts.system_prompts.insert(
    "merchant".to_string(),
    SystemPromptTemplate {
        base_description: "You are Marcus, the shrewdest merchant in town...".to_string(),
        behavior_guidelines: vec![
            "Always mention your rare items".to_string(),
            "Boast about your connections".to_string(),
        ],
        response_constraints: "Speak with confidence. Be slightly arrogant.".to_string(),
        context_template: "Recent dealings:\n{conversation_history}".to_string(),
    }
);
```

### Runtime Prompt Modification

```rust
// Modify prompts during gameplay
let mut prompts = agent.prompts();

// Add new emotional state
prompts.emotional_prompts.emotional_states.insert(
    "possessed".to_string(),
    "You speak with a demonic voice and dark intentions.".to_string()
);

// Add new greeting after special event
if player_saved_town {
    prompts.greetings.entry("merchant".to_string())
        .or_default()
        .push("Hero! Welcome back!".to_string());
}
```

### Localization

```rust
// Load different prompt files based on language
let language = get_player_language(); // "en", "es", "fr", etc.

let prompt_file = format!("prompts_{}.toml", language);
let prompts = PromptConfig::from_file_or_default(&prompt_file)?;
```

---

## Best Practices

### 1. **Keep Prompts Concise**
```toml
# ❌ Too verbose
base_description = "You are Marcus, and you've been a merchant for 30 years, traveling across the kingdom selling rare artifacts and exotic goods from distant lands. You have a wife and three children back home in the capital city..."

# ✅ Concise and clear
base_description = "You are {npc_name}, a veteran merchant specializing in rare artifacts and exotic goods."
```

### 2. **Use Template Variables**
```toml
# ❌ Hardcoded
base_description = "You are Marcus, a merchant..."

# ✅ Reusable
base_description = "You are {npc_name}, a {npc_role}..."
```

### 3. **Consistent Formatting**
```toml
# ✅ Clear structure
[system_prompts.merchant]
base_description = "..."
behavior_guidelines = [
    "Guideline 1",
    "Guideline 2"
]
response_constraints = "..."
```

### 4. **Test Prompts Incrementally**
- Change one section at a time
- Test with actual gameplay
- Get feedback from players/writers
- Iterate based on results

### 5. **Version Your Prompts**
```toml
version = "1.2"  # Track changes

# Add comments about updates
# v1.2 - Made merchants more aggressive
# v1.1 - Added emotional variety
# v1.0 - Initial release
```

---

## Troubleshooting

### Prompt Not Loading

**Problem:** Changes to `prompts.toml` not appearing in game

**Solutions:**
1. Verify file path is correct
2. Check file syntax (use TOML validator)
3. Ensure file is in same directory as agent config
4. Try explicit loading:
   ```rust
   config.prompts = Some(PromptConfig::from_file("prompts.toml")?);
   ```

### Role Not Found

**Problem:** Custom role prompts not working

**Solutions:**
1. Verify role name matches exactly (case-sensitive)
2. Add to `system_prompts` section:
   ```toml
   [system_prompts.my_custom_role]
   base_description = "..."
   ```
3. Or falls back to `default_system_prompt`

### Template Variables Not Replacing

**Problem:** Seeing `{npc_name}` in actual dialogue

**Solutions:**
1. Ensure variable names are exact: `{npc_name}`, not `{npcname}`
2. Check spelling: `{conversation_history}` not `{conversationhistory}`
3. Verify agent has these fields set:
   ```rust
   let mut context = HashMap::new();
   context.insert("name".to_string(), json!(agent.name()));
   context.insert("role".to_string(), json!(agent.role()));
   agent.update_context(context).await;
   ```

---

## Examples

### Fantasy RPG Merchant

```toml
[system_prompts.merchant]
base_description = "You are {npc_name}, a cunning merchant in a medieval fantasy world. You trade in weapons, armor, potions, and rare artifacts."

behavior_guidelines = [
    "Always assess the customer's wealth before pricing",
    "Mention rare items casually to gauge interest",
    "Use fantasy-appropriate language (gold, tavern, adventurer)",
    "Be friendly but never give discounts without haggling"
]

response_constraints = "Keep responses under 40 words. Use medieval fantasy terminology."

[greetings.merchant]
merchant = [
    "Well met, adventurer! Seeking steel or sorcery today?",
    "Ah, a new face! My wares are the finest in the realm!",
    "Welcome, welcome! I have just what a hero like you needs!"
]
```

### Sci-Fi Cyberpunk Hacker

```toml
[system_prompts.hacker]
base_description = "You are {npc_name}, a streetwise hacker in a dystopian cyberpunk city. You deal in information, illegal software, and black market tech."

behavior_guidelines = [
    "Use tech jargon and slang (ICE, netrunner, chrome)",
    "Be paranoid about corporate surveillance",
    "Speak in short, clipped sentences",
    "Show distrust of authority and corporations"
]

response_constraints = "Under 35 words. Use cyberpunk slang. Be suspicious."

[greetings.hacker]
hacker = [
    "You're not corpo, are you? What do you need?",
    "Keep it quiet. What brings you to my corner of the net?",
    "Got creds? Got a job? Talk fast."
]
```

### Horror Game Possessed NPC

```toml
[emotional_prompts.emotional_states]
possessed = "You speak with an otherworldly, sinister tone. Your voice echoes strangely."

[emotional_prompts.response_styles]
demonic = "Respond with cryptic threats and dark prophecies. Laugh maniacally between sentences."

[system_prompts.possessed_villager]
base_description = "You are {npc_name}, a villager possessed by an ancient evil entity."

behavior_guidelines = [
    "Alternate between normal speech and demonic outbursts",
    "Reference ancient dark powers",
    "Threaten dire consequences",
    "Speak in riddles about the player's fate"
]

response_constraints = "Keep responses eerie and unsettling. 30 words max."
```

---

## Migration from Hardcoded Prompts

If you were using hardcoded prompts before:

### Before (Hardcoded):
```rust
fn create_system_prompt(name: &str, role: &str) -> String {
    match role {
        "merchant" => format!("You are {}, a merchant...", name),
        "guard" => format!("You are {}, a guard...", name),
        _ => format!("You are {}...", name)
    }
}
```

### After (Template System):
```toml
# prompts.toml
[system_prompts.merchant]
base_description = "You are {npc_name}, a merchant..."

[system_prompts.guard]
base_description = "You are {npc_name}, a guard..."
```

```rust
// In code
let prompt = prompts.generate_system_prompt(name, role, &history);
```

---

## FAQ

**Q: Can I use JSON instead of TOML?**  
A: Yes! Just save as `.json` instead of `.toml`. The format is auto-detected.

**Q: How do I add a completely new NPC type?**  
A: Add a new section under `[system_prompts.your_type]` with all required fields.

**Q: Can prompts be changed at runtime?**  
A: Yes, but changes won't persist unless you call `save_to_file()`.

**Q: What happens if a role isn't defined?**  
A: It falls back to `default_system_prompt`.

**Q: Can I have multiple prompt files?**  
A: Yes! Load different files for different NPCs or situations.

**Q: How do I test my prompts?**  
A: Create a simple test agent and run conversations in your game.

---

## Support

- **Issues:** Report bugs or request features on GitHub
- **Documentation:** Full API docs at docs.rs
- **Examples:** See `examples/` directory for working samples
- **Community:** Join our Discord for prompt-writing tips

---

## License

Same as Oxyde SDK - MIT or Apache 2.0, your choice.