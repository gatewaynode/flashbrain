<script>
  import { createEventDispatcher } from 'svelte';
  import { Handle, Position } from '@xyflow/svelte';
  import { invoke } from '@tauri-apps/api/core';
  
  const { data } = $props();
  
  const dispatch = createEventDispatcher();
  
  let isExpanded = $state(false);
  
  function handleInputChange(field, value) {
    const updatedItem = { ...data.item, [field]: value };
    dispatch('update', { ...data, item: updatedItem });
  }
  
  function handleSpeedChange(value) {
    const updatedItem = {
      ...data.item,
      actions: [
        {
          ...data.item.actions[0],
          payload: { ...data.item.actions[0].payload, speed: parseInt(value) }
        }
      ]
    };
    dispatch('update', { ...data, item: updatedItem });
  }
  
  async function handleImageSelect() {
    try {
      console.log('Opening image file dialog for item:', data.item.item_id);
      
      const result = await invoke('open_file_dialog', {
        title: 'Select Image File',
        filters: [
          ['Image Files', ['png', 'jpg', 'jpeg', 'gif', 'bmp', 'webp']]
        ]
      });
      
      console.log('Image file dialog result:', result);
      
      if (result.success && result.file_path) {
        // Update the image path in the item
        handleInputChange('image', result.file_path);
        console.log('Updated image path to:', result.file_path);
      }
    } catch (error) {
      console.error('Failed to open image file dialog:', error);
    }
  }
  
  function toggleExpand() {
    isExpanded = !isExpanded;
  }
</script>

<div class="item-node" class:collapsed={!isExpanded}>
  <Handle 
    type="target" 
    position={Position.Top} 
    id={`item-${data.item.item_id}-target`}
    style="background: #4a90e2;"
  />
  
  <div class="node-header">
    <div class="header-content">
      <h4>{data.label}</h4>
      <button class="expand-button" on:click={toggleExpand}>
        {isExpanded ? '▼' : '▶'}
      </button>
    </div>
    <div class="collapsed-info">
      <span class="item-id">{data.item.item_id}</span>
    </div>
  </div>
  
  {#if isExpanded}
    <div class="node-content">
      <div class="field-group">
        <label>Item ID:</label>
        <input 
          type="text" 
          value={data.item.item_id} 
          on:input={(e) => handleInputChange('item_id', e.target.value)}
        />
      </div>
      <div class="field-group">
        <label>Text:</label>
        <textarea 
          value={data.item.text} 
          on:input={(e) => handleInputChange('text', e.target.value)}
        ></textarea>
      </div>
      <div class="field-group">
        <label>Image Path:</label>
        <div class="image-input">
          <input 
            type="text" 
            value={data.item.image} 
            on:input={(e) => handleInputChange('image', e.target.value)}
          />
          <button class="image-button" on:click={handleImageSelect}>
            📁
          </button>
        </div>
      </div>
      <div class="field-group">
        <label>Speed:</label>
        <input 
          type="number" 
          min="1" 
          max="11" 
          value={data.item.actions[0].payload.speed} 
          on:input={(e) => handleSpeedChange(e.target.value)}
        />
      </div>
    </div>
  {/if}
  
  <Handle 
    type="source" 
    position={Position.Bottom} 
    id={`item-${data.item.item_id}-source`}
    style="background: #4a90e2;"
  />
</div>

<style>
  .item-node {
    background: rgba(255,255,255,0.1);
    border: 1px solid rgba(255,255,255,0.2);
    border-radius: 8px;
    padding: 1rem;
    min-width: 250px;
    backdrop-filter: blur(10px);
    transition: all 0.3s ease;
  }

  .item-node.collapsed {
    min-width: 200px;
    padding: 0.75rem;
  }

  .node-header {
    margin-bottom: 1rem;
  }

  .header-content {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 0.5rem;
  }

  .node-header h4 {
    color: #fff;
    margin: 0;
    font-size: 1rem;
  }

  .expand-button {
    background: transparent;
    border: none;
    color: #fff;
    cursor: pointer;
    font-size: 0.8rem;
    padding: 0.25rem;
    border-radius: 3px;
    transition: background-color 0.2s ease;
  }

  .expand-button:hover {
    background: rgba(255,255,255,0.1);
  }

  .collapsed-info {
    display: flex;
    align-items: center;
  }

  .item-id {
    color: #4a90e2;
    font-weight: 600;
    font-size: 0.9rem;
  }

  .field-group {
    margin-bottom: 0.75rem;
  }

  .field-group label {
    display: block;
    color: #aaa;
    font-size: 0.8rem;
    margin-bottom: 0.25rem;
  }

  .field-group input, .field-group textarea {
    width: 100%;
    background: rgba(0,0,0,0.3);
    border: 1px solid rgba(255,255,255,0.2);
    color: #fff;
    padding: 0.5rem;
    border-radius: 4px;
    font-size: 0.9rem;
  }

  .field-group textarea {
    min-height: 60px;
    resize: vertical;
  }

  .image-input {
    display: flex;
    gap: 0.5rem;
  }

  .image-input input {
    flex: 1;
  }

  .image-button {
    background: rgba(255,255,255,0.1);
    border: 1px solid rgba(255,255,255,0.2);
    color: #fff;
    padding: 0.5rem;
    border-radius: 4px;
    cursor: pointer;
    font-size: 1rem;
  }

  .image-button:hover {
    background: rgba(255,255,255,0.2);
  }
</style> 