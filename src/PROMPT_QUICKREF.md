# Oxyde Prompt Templates - Quick Reference

## File Structure

```toml
version = "1.0"

[default_system_prompt]
[system_prompts.role_name]
[emotional_prompts]
[goal_prompts]
[memory_prompts]
[greetings]
[farewells]
```

---

## Common Tasks

### Add a New NPC Role

```toml
[system_prompts.pirate]
base_description = "You are {npc_name}, a {npc_role}..."
behavior_guidelines = ["Talk like a pirate", "Mention treasure"]
response_constraints = "Under 50 words. Use 'arr' and 'matey'."
context_template = "Previous: {conversation_history}"
```

### Add a New Emotion

```toml
[emotional_prompts.emotional_states]
excited = "You're thrilled and speak rapidly."

[emotional_prompts.response_styles]
excited = "Respond with lots of energy and exclamation marks!"
```

### Add New Greetings

```toml
[greetings.pirate]
pirate = [
    "Arr! What brings ye to me ship?",
    "Ahoy there, matey!",
    "Welcome aboard, landlubber!"
]
```

---

## Template Variables

| Variable | Description | Example |
|----------|-------------|---------|
| `{npc_name}` | Agent's name | "Marcus" |
| `{npc_role}` | Agent's role | "merchant" |
| `{conversation_history}` | Previous messages | Player: "Hello"\nMarcus: "Greetings!" |
| `{memory_context}` | Formatted memories | - [Event] Met player *** |
| `{goal_description}` | Current goal | "Earn 1000 gold" |
| `{goal_type}` | Goal category | "business goal" |
| `{priority}` | Goal importance | "high priority" |
| `{progress}` | Completion % | "35%" |
| `{motivation}` | Motivation level | "8.5/10" |
| `{emotional_state}` | Current emotion | "You're cheerful..." |
| `{response_style}` | Speaking style | "Respond warmly..." |

---

## Code Snippets

### Load Prompts

```rust
// Auto-load (checks for prompts.toml in config dir)
let config = AgentConfig::from_file("agent.json")?;
let agent = Agent::new(config);

// Explicit load with fallback
let prompts = PromptConfig::from_file_or_default("prompts.toml")?;

// Bundled defaults only
let prompts = PromptConfig::from_bundled_default()?;
```

### Use in Agent

```rust
let agent = Agent::new(config);
let prompts = agent.prompts();  // Get Arc<PromptConfig>

let greeting = prompts.get_greeting("merchant");
let farewell = prompts.get_farewell("guard");

let system_prompt = prompts.generate_system_prompt(
    &agent.name(),
    &agent.role(),
    &conversation_history
);
```

---

## File Formats

### TOML (Recommended)
```toml
[system_prompts.merchant]
base_description = "You are {npc_name}..."
```

### JSON
```json
{
  "system_prompts": {
    "merchant": {
      "base_description": "You are {npc_name}..."
    }
  }
}
```

### YAML
```yaml
system_prompts:
  merchant:
    base_description: "You are {npc_name}..."
```

---

## Troubleshooting

| Problem | Solution |
|---------|----------|
| Changes not appearing | Check file path, syntax, reload agent |
| Role not found | Add to `[system_prompts.role_name]` |
| Variables not replacing | Check spelling: `{npc_name}` not `{npcname}` |
| File not loading | Verify path: same dir as agent config |

---

## Quick Examples

### Minimal Role
```toml
[system_prompts.villager]
base_description = "You are {npc_name}, a friendly villager."
behavior_guidelines = ["Be helpful", "Share gossip"]
response_constraints = "Keep under 50 words."
context_template = "Recent: {conversation_history}"
```

### With Emotions
```toml
[emotional_prompts.emotional_states]
angry = "You're furious and speak harshly."

[emotional_prompts.response_styles]
hostile = "Respond curtly with irritation."
```

### With Goals
```toml
[goal_prompts.goal_type_descriptions]
Economic = "business goal"
Social = "friendship goal"
```

### With Greetings
```toml
[greetings.merchant]
merchant = ["Welcome!", "Hello traveler!", "Good day!"]

[farewells.merchant]
merchant = ["Farewell!", "Come again!", "Safe travels!"]
```

---

## Default Roles Included

- `merchant` - Trader/shopkeeper
- `guard` - Law enforcement
- `villager` - Local resident
- `innkeeper` - Tavern owner
- `blacksmith` - Weapon/armor smith

---

## Reference Links

- Full Documentation: `PROMPT_TEMPLATES.md`
- Example File: `config/defaults.toml`