<script>
  import { createEventDispatcher } from 'svelte';
  import { Handle, Position } from '@xyflow/svelte';
  import { invoke } from '@tauri-apps/api/core';
  
  const { data } = $props();
  
  const dispatch = createEventDispatcher();
  
  let isExpanded = $state(false);
  
  function handleInputChange(field, value) {
    const updatedChild = { ...data.child, [field]: value };
    dispatch('update', { ...data, child: updatedChild });
  }
  
  function handleGateChange(index, field, value) {
    const updatedGates = [...data.child.gates];
    if (field === 'type') {
      updatedGates[index] = { ...updatedGates[index], type: value };
    } else if (field === 'speed') {
      updatedGates[index] = { 
        ...updatedGates[index], 
        payload: { ...updatedGates[index].payload, speed: parseInt(value) }
      };
    }
    const updatedChild = { ...data.child, gates: updatedGates };
    dispatch('update', { ...data, child: updatedChild });
  }
  
  function handleActionChange(index, field, value) {
    const updatedActions = [...data.child.actions];
    if (field === 'type') {
      updatedActions[index] = { ...updatedActions[index], type: value };
    } else if (field === 'speed') {
      updatedActions[index] = { 
        ...updatedActions[index], 
        payload: { ...updatedActions[index].payload, speed: parseInt(value) }
      };
    }
    const updatedChild = { ...data.child, actions: updatedActions };
    dispatch('update', { ...data, child: updatedChild });
  }
  
  function toggleExpand() {
    isExpanded = !isExpanded;
  }
  
  function toggleGated() {
    const updatedChild = { ...data.child, gated: !data.child.gated };
    dispatch('update', { ...data, child: updatedChild });
  }
</script>

<div class="training-child-node" class:collapsed={!isExpanded}>
  <Handle 
    type="target" 
    position={Position.Top} 
    id="child-target"
    style="background: #8e44ad;"
  />
  
  <div class="node-header">
    <div class="header-content">
      <h4>{data.label}</h4>
      <button class="expand-button" onclick={toggleExpand}>
        {isExpanded ? '▼' : '▶'}
      </button>
    </div>
    <div class="collapsed-info">
      <span class="item-id">{data.child.lesson_id}</span>
      {#if data.child.gated}
        <span class="gated-indicator">🔒</span>
      {/if}
    </div>
  </div>
  
  {#if isExpanded}
    <div class="node-content">
      <div class="field-group">
        <label>Lesson ID:</label>
        <input 
          type="text" 
          value={data.child.lesson_id} 
          oninput={(e) => handleInputChange('lesson_id', e.target.value)}
        />
      </div>
      <div class="field-group">
        <label>Title:</label>
        <input 
          type="text" 
          value={data.child.title} 
          oninput={(e) => handleInputChange('title', e.target.value)}
        />
      </div>
      <div class="field-group">
        <label>Default Order:</label>
        <input 
          type="number" 
          value={data.child.default_order} 
          oninput={(e) => handleInputChange('default_order', parseInt(e.target.value))}
        />
      </div>
      <div class="field-group">
        <label>Gated:</label>
        <div class="toggle-container">
          <button 
            class="toggle-button" 
            class:active={data.child.gated}
            onclick={toggleGated}
          >
            {data.child.gated ? 'Yes' : 'No'}
          </button>
        </div>
      </div>
      <div class="field-group">
        <label>Image Path:</label>
        <div class="image-input">
          <input 
            type="text" 
            value={data.child.image} 
            oninput={(e) => handleInputChange('image', e.target.value)}
          />
        </div>
      </div>
      
      <!-- Gates Section -->
      <div class="section-header">
        <h5>Gates ({data.child.gates.length})</h5>
      </div>
      {#each data.child.gates as gate, gateIndex}
        <div class="gate-item">
          <div class="field-group">
            <label>Gate {gateIndex + 1} Type:</label>
            <input 
              type="text" 
              value={gate.type} 
              oninput={(e) => handleGateChange(gateIndex, 'type', e.target.value)}
            />
          </div>
          <div class="field-group">
            <label>Gate {gateIndex + 1} Speed:</label>
            <input 
              type="number" 
              min="1" 
              max="11" 
              value={gate.payload.speed} 
              oninput={(e) => handleGateChange(gateIndex, 'speed', e.target.value)}
            />
          </div>
        </div>
      {/each}
      
      <!-- Actions Section -->
      <div class="section-header">
        <h5>Actions ({data.child.actions.length})</h5>
      </div>
      {#each data.child.actions as action, actionIndex}
        <div class="action-item">
          <div class="field-group">
            <label>Action {actionIndex + 1} Type:</label>
            <input 
              type="text" 
              value={action.type} 
              oninput={(e) => handleActionChange(actionIndex, 'type', e.target.value)}
            />
          </div>
          <div class="field-group">
            <label>Action {actionIndex + 1} Speed:</label>
            <input 
              type="number" 
              min="1" 
              max="11" 
              value={action.payload.speed} 
              oninput={(e) => handleActionChange(actionIndex, 'speed', e.target.value)}
            />
          </div>
        </div>
      {/each}
    </div>
  {/if}
  
  <Handle 
    type="source" 
    position={Position.Bottom} 
    id="child-source"
    style="background: #8e44ad;"
  />
</div>

<style>
  .training-child-node {
    background: linear-gradient(135deg, #2c3e50, #34495e);
    border: 2px solid #8e44ad;
    border-radius: 12px;
    padding: 16px;
    min-width: 280px;
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.3);
    backdrop-filter: blur(10px);
    transition: all 0.3s ease;
  }
  
  .training-child-node:hover {
    transform: translateY(-2px);
    box-shadow: 0 12px 40px rgba(0, 0, 0, 0.4);
  }
  
  .training-child-node.collapsed {
    min-width: 200px;
  }
  
  .node-header {
    margin-bottom: 12px;
  }
  
  .header-content {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 8px;
  }
  
  .header-content h4 {
    margin: 0;
    color: #ecf0f1;
    font-size: 14px;
    font-weight: 600;
  }
  
  .expand-button {
    background: #8e44ad;
    border: none;
    color: white;
    width: 24px;
    height: 24px;
    border-radius: 4px;
    cursor: pointer;
    font-size: 12px;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: background 0.2s ease;
  }
  
  .expand-button:hover {
    background: #9b59b6;
  }
  
  .collapsed-info {
    display: flex;
    align-items: center;
    gap: 8px;
  }
  
  .item-id {
    color: #bdc3c7;
    font-size: 12px;
    font-weight: 500;
  }
  
  .gated-indicator {
    font-size: 12px;
  }
  
  .node-content {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }
  
  .field-group {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }
  
  .field-group label {
    color: #bdc3c7;
    font-size: 11px;
    font-weight: 500;
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }
  
  .field-group input {
    background: rgba(255, 255, 255, 0.1);
    border: 1px solid rgba(255, 255, 255, 0.2);
    border-radius: 6px;
    padding: 8px 12px;
    color: #ecf0f1;
    font-size: 12px;
    transition: all 0.2s ease;
  }
  
  .field-group input:focus {
    outline: none;
    border-color: #8e44ad;
    background: rgba(255, 255, 255, 0.15);
  }
  
  .toggle-container {
    display: flex;
    align-items: center;
  }
  
  .toggle-button {
    background: rgba(255, 255, 255, 0.1);
    border: 1px solid rgba(255, 255, 255, 0.2);
    border-radius: 6px;
    padding: 6px 12px;
    color: #ecf0f1;
    font-size: 12px;
    cursor: pointer;
    transition: all 0.2s ease;
  }
  
  .toggle-button.active {
    background: #8e44ad;
    border-color: #8e44ad;
  }
  
  .toggle-button:hover {
    background: rgba(255, 255, 255, 0.2);
  }
  
  .toggle-button.active:hover {
    background: #9b59b6;
  }
  
  .section-header {
    margin-top: 16px;
    padding-top: 12px;
    border-top: 1px solid rgba(255, 255, 255, 0.1);
  }
  
  .section-header h5 {
    margin: 0 0 8px 0;
    color: #8e44ad;
    font-size: 12px;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }
  
  .gate-item, .action-item {
    background: rgba(255, 255, 255, 0.05);
    border-radius: 6px;
    padding: 8px;
    margin-bottom: 8px;
  }
  
  .gate-item:last-child, .action-item:last-child {
    margin-bottom: 0;
  }
</style> 