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

        [DllImport("oxyde", EntryPoint = "oxyde_unity_update_agent")]
        private static extern bool NativeUpdateAgent(string agentId, string contextJson);

        [DllImport("oxyde", EntryPoint = "oxyde_unity_process_input")]
        private static extern IntPtr NativeProcessInput(string agentId, string input);

        [DllImport("oxyde", EntryPoint = "oxyde_unity_get_agent_state")]
        private static extern IntPtr NativeGetAgentState(string agentId);

        [DllImport("oxyde", EntryPoint = "oxyde_unity_free_string")]
        private static extern void NativeFreeString(IntPtr ptr);

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

        #endregion
    }

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
            // Base implementation does nothing
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
