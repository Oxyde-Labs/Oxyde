using System;
using System.Collections.Generic;
using System.Runtime.InteropServices;
using UnityEngine;
using Newtonsoft.Json;

namespace Oxyde.Unity
{
    /// <summary>
    /// C# bindings for the Oxyde SDK in Unity
    /// </summary>
    public static class OxydeUnity
    {
        #region Native Methods

        // Import native methods from the Oxyde dynamic library
        
        [DllImport("oxyde", EntryPoint = "oxyde_unity_init")]
        private static extern bool NativeInit();

        [DllImport("oxyde", EntryPoint = "oxyde_unity_create_agent")]
        private static extern IntPtr NativeCreateAgent(string configPath);

        [DllImport("oxyde", EntryPoint = "oxyde_unity_create_agent_from_json")]
        private static extern IntPtr NativeCreateAgentFromJson(string jsonConfig);

        [DllImport("oxyde", EntryPoint = "oxyde_unity_update_agent")]
        private static extern bool NativeUpdateAgent(string agentId, string contextJson);

        [DllImport("oxyde", EntryPoint = "oxyde_unity_process_input")]
        private static extern IntPtr NativeProcessInput(string agentId, string input);

        [DllImport("oxyde", EntryPoint = "oxyde_unity_get_agent_state")]
        private static extern IntPtr NativeGetAgentState(string agentId);

        [DllImport("oxyde", EntryPoint = "oxyde_unity_get_emotion_vector")]
        private static extern IntPtr NativeGetEmotionVector(string agentId);

        [DllImport("oxyde", EntryPoint = "oxyde_unity_get_emotion_vector_raw")]
        private static extern bool NativeGetEmotionVectorRaw(
            string agentId,
            out float joy,
            out float trust,
            out float fear,
            out float surprise,
            out float sadness,
            out float disgust,
            out float anger,
            out float anticipation
        );

        [DllImport("oxyde", EntryPoint = "oxyde_unity_free_string")]
        private static extern void NativeFreeString(IntPtr ptr);

        // Memory System
        [DllImport("oxyde", EntryPoint = "oxyde_unity_add_memory")]
        private static extern bool NativeAddMemory(string agentId, string category, string content, double importance);

        [DllImport("oxyde", EntryPoint = "oxyde_unity_add_emotional_memory")]
        private static extern bool NativeAddEmotionalMemory(string agentId, string category, string content, double importance, double valence, double intensity);

        [DllImport("oxyde", EntryPoint = "oxyde_unity_get_memory_count")]
        private static extern uint NativeGetMemoryCount(string agentId);

        [DllImport("oxyde", EntryPoint = "oxyde_unity_clear_memories")]
        private static extern uint NativeClearMemories(string agentId);

        [DllImport("oxyde", EntryPoint = "oxyde_unity_get_memories_by_category")]
        private static extern IntPtr NativeGetMemoriesByCategory(string agentId, string category);

        [DllImport("oxyde", EntryPoint = "oxyde_unity_retrieve_relevant_memories")]
        private static extern IntPtr NativeRetrieveRelevantMemories(string agentId, string query, uint limit);

        [DllImport("oxyde", EntryPoint = "oxyde_unity_forget_memory")]
        private static extern bool NativeForgetMemory(string agentId, string memoryId);

        [DllImport("oxyde", EntryPoint = "oxyde_unity_forget_memories_by_category")]
        private static extern uint NativeForgetMemoriesByCategory(string agentId, string category);

        #endregion

        #region Helper Methods

        /// <summary>
        /// Convert an unmanaged UTF-8 string pointer to a managed string and free the unmanaged memory
        /// </summary>
        private static string PtrToStringAndFree(IntPtr ptr)
        {
            if (ptr == IntPtr.Zero)
                return string.Empty;
                
            string result = Marshal.PtrToStringUTF8(ptr);
            NativeFreeString(ptr);
            return result;
        }

        #endregion

        #region Public API

        /// <summary>
        /// Initialize the Oxyde SDK
        /// </summary>
        /// <returns>True if initialization was successful</returns>
        public static bool Init()
        {
            try
            {
                bool result = NativeInit();
                Debug.Log($"Oxyde SDK initialized: {result}");
                return result;
            }
            catch (Exception ex)
            {
                Debug.LogError($"Failed to initialize Oxyde SDK: {ex.Message}");
                return false;
            }
        }

        /// <summary>
        /// Create a new agent from a configuration file
        /// </summary>
        /// <param name="configPath">Path to the agent configuration file</param>
        /// <returns>Agent ID string or empty if failed</returns>
        public static string CreateAgent(string configPath)
        {
            try
            {
                IntPtr resultPtr = NativeCreateAgent(configPath);
                string agentId = PtrToStringAndFree(resultPtr);
                
                if (string.IsNullOrEmpty(agentId))
                {
                    Debug.LogError($"Failed to create agent from config: {configPath}");
                }
                else
                {
                    Debug.Log($"Created agent with ID: {agentId}");
                }
                
                return agentId;
            }
            catch (Exception ex)
            {
                Debug.LogError($"Error creating agent: {ex.Message}");
                return string.Empty;
            }
        }

        /// <summary>
        /// Create a new agent from a configuration JSON string
        /// </summary>
        /// <param name="jsonConfig">The agent configuration as a JSON string</param>
        /// <returns>Agent ID string or empty if failed</returns>
        public static string CreateAgentFromJson(string jsonConfig)
        {
            try
            {
                IntPtr resultPtr = NativeCreateAgentFromJson(jsonConfig);
                string agentId = PtrToStringAndFree(resultPtr);
                
                if (string.IsNullOrEmpty(agentId))
                {
                    Debug.LogError("Failed to create agent from JSON config");
                }
                else
                {
                    Debug.Log($"Created agent with ID: {agentId}");
                }
                
                return agentId;
            }
            catch (Exception ex)
            {
                Debug.LogError($"Error creating agent from JSON: {ex.Message}");
                return string.Empty;
            }
        }

        /// <summary>
        /// Create a new agent from a configuration file located in the Resources folder.
        /// This is the easiest way to create an agent in Unity.
        /// </summary>
        /// <param name="resourcePath">Path to the config file relative to Resources folder (without extension)</param>
        /// <returns>Agent ID string or empty if failed</returns>
        public static string CreateAgentFromResource(string resourcePath)
        {
            try
            {
                TextAsset configAsset = Resources.Load<TextAsset>(resourcePath);
                if (configAsset == null)
                {
                    Debug.LogError($"Could not find config file in Resources: {resourcePath}");
                    return string.Empty;
                }
                
                return CreateAgentFromJson(configAsset.text);
            }
            catch (Exception ex)
            {
                Debug.LogError($"Error creating agent from resource: {ex.Message}");
                return string.Empty;
            }
        }

        /// <summary>
        /// Update an agent with new context data
        /// </summary>
        /// <param name="agentId">Agent ID string</param>
        /// <param name="contextJson">JSON string with context data</param>
        /// <returns>True if update was successful</returns>
        public static bool UpdateAgentContext(string agentId, string contextJson)
        {
            try
            {
                return NativeUpdateAgent(agentId, contextJson);
            }
            catch (Exception ex)
            {
                Debug.LogError($"Error updating agent context: {ex.Message}");
                return false;
            }
        }

        /// <summary>
        /// Process input for an agent
        /// </summary>
        /// <param name="agentId">Agent ID string</param>
        /// <param name="input">Input text</param>
        /// <returns>Agent's response</returns>
        public static string ProcessInput(string agentId, string input)
        {
            try
            {
                IntPtr resultPtr = NativeProcessInput(agentId, input);
                return PtrToStringAndFree(resultPtr);
            }
            catch (Exception ex)
            {
                Debug.LogError($"Error processing input: {ex.Message}");
                return $"Error: {ex.Message}";
            }
        }

        /// <summary>
        /// Get the current state of an agent
        /// </summary>
        /// <param name="agentId">Agent ID string</param>
        /// <returns>JSON string with agent state</returns>
        public static string GetAgentState(string agentId)
        {
            try
            {
                IntPtr resultPtr = NativeGetAgentState(agentId);
                return PtrToStringAndFree(resultPtr);
            }
            catch (Exception ex)
            {
                Debug.LogError($"Error getting agent state: {ex.Message}");
                return "{}";
            }
        }

        /// <summary>
        /// Get the agent's emotion vector as a float array
        /// </summary>
        /// <param name="agentId">Agent ID string</param>
        /// <returns>Float array with emotion values [joy, trust, fear, surprise, sadness, disgust, anger, anticipation]</returns>
        public static float[] GetAgentEmotionVector(string agentId)
        {
            try
            {
                float joy, trust, fear, surprise, sadness, disgust, anger, anticipation;
                bool success = NativeGetEmotionVectorRaw(
                    agentId,
                    out joy,
                    out trust,
                    out fear,
                    out surprise,
                    out sadness,
                    out disgust,
                    out anger,
                    out anticipation
                );
                
                if (success)
                {
                    return new float[] { joy, trust, fear, surprise, sadness, disgust, anger, anticipation };
                }
                else
                {
                    Debug.LogWarning($"Failed to get emotion vector for agent {agentId}");
                    return new float[] { 0.0f, 0.0f, 0.0f, 0.0f, 0.0f, 0.0f, 0.0f, 0.0f };
                }
            }
            catch (Exception ex)
            {
                Debug.LogError($"Error getting agent emotion vector: {ex.Message}");
                return new float[] { 0.0f, 0.0f, 0.0f, 0.0f, 0.0f, 0.0f, 0.0f, 0.0f };
            }
        }

        // ==================== Memory System ====================

        /// <summary>
        /// Memory category types
        /// </summary>
        public enum MemoryCategory
        {
            Episodic,    // Personal experiences and events
            Semantic,    // Facts and knowledge
            Procedural,  // Skills and how-to knowledge
            Emotional    // Emotionally charged memories
        }

        /// <summary>
        /// Represents a single memory
        /// </summary>
        public class Memory
        {
            public string id;
            public string category;
            public string content;
            public double importance;
            public double? valence;      // Optional: -1.0 to 1.0
            public double? intensity;    // Optional: 0.0 to 1.0
            public List<string> tags;
            public long timestamp;
        }

        /// <summary>
        /// Add a memory to an agent's memory system
        /// </summary>
        public static bool AddMemory(string agentId, MemoryCategory category, string content, double importance = 0.5)
        {
            try
            {
                return NativeAddMemory(agentId, category.ToString().ToLower(), content, importance);
            }
            catch (Exception ex)
            {
                Debug.LogError($"Error adding memory: {ex.Message}");
                return false;
            }
        }

        /// <summary>
        /// Add an emotional memory to an agent's memory system
        /// </summary>
        public static bool AddEmotionalMemory(string agentId, MemoryCategory category, string content, 
            double importance, double valence, double intensity)
        {
            try
            {
                return NativeAddEmotionalMemory(agentId, category.ToString().ToLower(), content, 
                    importance, valence, intensity);
            }
            catch (Exception ex)
            {
                Debug.LogError($"Error adding emotional memory: {ex.Message}");
                return false;
            }
        }

        /// <summary>
        /// Get the number of memories stored by an agent
        /// </summary>
        public static uint GetMemoryCount(string agentId)
        {
            try
            {
                return NativeGetMemoryCount(agentId);
            }
            catch (Exception ex)
            {
                Debug.LogError($"Error getting memory count: {ex.Message}");
                return 0;
            }
        }

        /// <summary>
        /// Clear all non-permanent memories from an agent
        /// </summary>
        public static uint ClearMemories(string agentId)
        {
            try
            {
                return NativeClearMemories(agentId);
            }
            catch (Exception ex)
            {
                Debug.LogError($"Error clearing memories: {ex.Message}");
                return 0;
            }
        }

        /// <summary>
        /// Get memories by category
        /// </summary>
        public static List<Memory> GetMemoriesByCategory(string agentId, MemoryCategory category)
        {
            try
            {
                IntPtr resultPtr = NativeGetMemoriesByCategory(agentId, category.ToString().ToLower());
                string json = PtrToStringAndFree(resultPtr);
                
                if (string.IsNullOrEmpty(json) || json == "[]")
                    return new List<Memory>();
                    
                return JsonConvert.DeserializeObject<List<Memory>>(json) ?? new List<Memory>();
            }
            catch (Exception ex)
            {
                Debug.LogError($"Error getting memories by category: {ex.Message}");
                return new List<Memory>();
            }
        }

        /// <summary>
        /// Retrieve memories relevant to a query
        /// </summary>
        public static List<Memory> RetrieveRelevantMemories(string agentId, string query, uint limit = 5)
        {
            try
            {
                IntPtr resultPtr = NativeRetrieveRelevantMemories(agentId, query, limit);
                string json = PtrToStringAndFree(resultPtr);
                
                if (string.IsNullOrEmpty(json) || json == "[]")
                    return new List<Memory>();
                    
                return JsonConvert.DeserializeObject<List<Memory>>(json) ?? new List<Memory>();
            }
            catch (Exception ex)
            {
                Debug.LogError($"Error retrieving relevant memories: {ex.Message}");
                return new List<Memory>();
            }
        }

        /// <summary>
        /// Forget a specific memory by ID
        /// </summary>
        public static bool ForgetMemory(string agentId, string memoryId)
        {
            try
            {
                return NativeForgetMemory(agentId, memoryId);
            }
            catch (Exception ex)
            {
                Debug.LogError($"Error forgetting memory: {ex.Message}");
                return false;
            }
        }

        /// <summary>
        /// Forget all memories of a specific category
        /// </summary>
        public static uint ForgetMemoriesByCategory(string agentId, MemoryCategory category)
        {
            try
            {
                return NativeForgetMemoriesByCategory(agentId, category.ToString().ToLower());
            }
            catch (Exception ex)
            {
                Debug.LogError($"Error forgetting memories by category: {ex.Message}");
                return 0;
            }
        }

        #endregion
    }

#if !OXYDE_DISABLE_BUILTIN_MANAGER
    /// <summary>
    /// Minimal in-file manager so the bindings work without adding a separate script.
    /// You can disable this by defining OXYDE_DISABLE_BUILTIN_MANAGER in your project settings.
    /// </summary>
    public class OxydeAgentManager : MonoBehaviour
    {
        public static OxydeAgentManager Instance { get; private set; }
        private readonly List<OxydeAgent> agents = new List<OxydeAgent>();

        private void Awake()
        {
            if (Instance == null)
            {
                Instance = this;
                DontDestroyOnLoad(gameObject);
                OxydeUnity.Init();
            }
            else
            {
                Destroy(gameObject);
            }
        }

        public void RegisterAgent(OxydeAgent agent)
        {
            if (agent != null && !agents.Contains(agent))
            {
                agents.Add(agent);
            }
        }

        public void UnregisterAgent(OxydeAgent agent)
        {
            if (agent != null)
            {
                agents.Remove(agent);
            }
        }

        public void UpdateAgentContext(Transform player, Dictionary<string, object> additionalContext = null)
        {
            foreach (var agent in agents)
            {
                agent.UpdatePlayerContext(player, additionalContext);
            }
        }
    }
#endif

    /// <summary>
    /// Base class for Oxyde agents in Unity
    /// </summary>
    public abstract class OxydeAgent : MonoBehaviour
    {
        /// <summary>
        /// Agent ID returned by the Oxyde SDK
        /// </summary>
        protected string AgentId { get; private set; }
        
        /// <summary>
        /// Name of the agent
        /// </summary>
        public string AgentName { get; protected set; } = "Unknown";
        
        /// <summary>
        /// Whether the agent is initialized
        /// </summary>
        public bool IsInitialized { get; private set; }
        
        /// <summary>
        /// Last response from the agent
        /// </summary>
        public string LastResponse { get; private set; }

        /// <summary>
        /// Current emotion vector [joy, trust, fear, surprise, sadness, disgust, anger, anticipation]
        /// </summary>
        public float[] EmotionVector { get; private set; } = new float[] { 0.0f, 0.0f, 0.0f, 0.0f, 0.0f, 0.0f, 0.0f, 0.0f };
        
        /// <summary>
        /// Initialize the agent with a configuration file
        /// </summary>
        /// <param name="configResourcePath">Path to the config file in Resources folder</param>
        protected virtual void InitializeAgent(string configResourcePath)
        {
            try
            {
                // Load config from Resources
                TextAsset configAsset = Resources.Load<TextAsset>(configResourcePath);
                if (configAsset == null)
                {
                    Debug.LogError($"Config file not found: {configResourcePath}");
                    return;
                }
                
                // Write to temporary file for native code to access
                string tempPath = System.IO.Path.Combine(
                    Application.temporaryCachePath,
                    $"agent_config_{Guid.NewGuid()}.json"
                );
                
                System.IO.File.WriteAllText(tempPath, configAsset.text);
                
                // Create agent using Oxyde SDK
                AgentId = OxydeUnity.CreateAgent(tempPath);
                
                if (!string.IsNullOrEmpty(AgentId))
                {
                    IsInitialized = true;
                    Debug.Log($"Initialized agent: {AgentName} (ID: {AgentId})");
                    
                    // Register with agent manager if available
                    var manager = FindObjectOfType<OxydeAgentManager>();
                    if (manager != null)
                    {
                        manager.RegisterAgent(this);
                    }
                }
                
                // Clean up temporary file
                try { System.IO.File.Delete(tempPath); } catch { }
            }
            catch (Exception ex)
            {
                Debug.LogError($"Failed to initialize agent: {ex.Message}");
            }
        }
        
        /// <summary>
        /// Process input and return the agent's response
        /// </summary>
        /// <param name="input">Input text</param>
        /// <returns>Agent's response</returns>
        public virtual string ProcessInput(string input)
        {
            if (!IsInitialized)
            {
                return "Agent not initialized";
            }
            
            LastResponse = OxydeUnity.ProcessInput(AgentId, input);
            return LastResponse;
        }
        
        /// <summary>
        /// Update the agent's context with player position
        /// </summary>
        /// <param name="player">Player transform</param>
        /// <param name="additionalContext">Additional context data</param>
        public virtual void UpdatePlayerContext(Transform player, Dictionary<string, object> additionalContext = null)
        {
            if (!IsInitialized || player == null)
            {
                return;
            }
            
            // Calculate distance to player
            float distance = Vector3.Distance(transform.position, player.position);
            
            // Build context data
            var context = new Dictionary<string, object>
            {
                { "player_x", player.position.x },
                { "player_y", player.position.y },
                { "player_z", player.position.z },
                { "player_distance", distance },
                { "npc_x", transform.position.x },
                { "npc_y", transform.position.y },
                { "npc_z", transform.position.z }
            };
            
            // Add additional context if provided
            if (additionalContext != null)
            {
                foreach (var kvp in additionalContext)
                {
                    context[kvp.Key] = kvp.Value;
                }
            }
            
            // Convert to JSON
            string contextJson = JsonConvert.SerializeObject(context);
            
            // Update context
            UpdateContext(context);
        }
        
        /// <summary>
        /// Update the agent's context
        /// </summary>
        /// <param name="context">Context data</param>
        public virtual void UpdateContext(Dictionary<string, object> context)
        {
            if (!IsInitialized)
            {
                return;
            }
            
            string contextJson = JsonConvert.SerializeObject(context);
            OxydeUnity.UpdateAgentContext(AgentId, contextJson);
        }
        
        /// <summary>
        /// Initialize and setup
        /// </summary>
        protected virtual void Start()
        {
            // Register with agent manager if available
            if (IsInitialized)
            {
                var manager = FindObjectOfType<OxydeAgentManager>();
                if (manager != null)
                {
                    manager.RegisterAgent(this);
                }
            }
        }
        
        /// <summary>
        /// Update logic
        /// </summary>
        protected virtual void Update()
        {
            // Update emotion data if agent is initialized
            if (IsInitialized)
            {
                UpdateEmotionData();
            }
        }

        /// <summary>
        /// Update emotion data from the agent
        /// </summary>
        protected virtual void UpdateEmotionData()
        {
            try
            {
                // Get emotion vector directly as float array
                EmotionVector = OxydeUnity.GetAgentEmotionVector(AgentId);
            }
            catch (Exception ex)
            {
                Debug.LogWarning($"Failed to update emotion data for agent {AgentName}: {ex.Message}");
            }
        }

        /// <summary>
        /// Set animator float parameters based on emotion vector
        /// </summary>
        /// <param name="animator">Animator component to update</param>
        public virtual void UpdateAnimatorWithEmotions(Animator animator)
        {
            if (animator == null || !IsInitialized || EmotionVector == null || EmotionVector.Length < 8)
                return;

            try
            {
                // Set float parameters for each emotion
                animator.SetFloat("Joy", EmotionVector[0]);
                animator.SetFloat("Trust", EmotionVector[1]);
                animator.SetFloat("Fear", EmotionVector[2]);
                animator.SetFloat("Surprise", EmotionVector[3]);
                animator.SetFloat("Sadness", EmotionVector[4]);
                animator.SetFloat("Disgust", EmotionVector[5]);
                animator.SetFloat("Anger", EmotionVector[6]);
                animator.SetFloat("Anticipation", EmotionVector[7]);
            }
            catch (Exception ex)
            {
                Debug.LogWarning($"Failed to update animator with emotions: {ex.Message}");
            }
        }
        
        /// <summary>
        /// Clean up when destroyed
        /// </summary>
        protected virtual void OnDestroy()
        {
            // Unregister from agent manager
            if (IsInitialized)
            {
                var manager = FindObjectOfType<OxydeAgentManager>();
                if (manager != null)
                {
                    manager.UnregisterAgent(this);
                }
            }
        }
    }
}
