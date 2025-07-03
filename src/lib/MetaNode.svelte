<script>
  import { createEventDispatcher } from 'svelte';
  import { Handle, Position } from '@xyflow/svelte';
  
  const { data } = $props();
  
  const dispatch = createEventDispatcher();
  
  let isExpanded = $state(false);
  
  function handleInputChange(field, value) {
    const updatedMeta = { ...data.meta, [field]: value };
    dispatch('update', { ...data, meta: updatedMeta });
  }
  
  function toggleExpand() {
    isExpanded = !isExpanded;
  }
</script>

<div class="meta-node" class:collapsed={!isExpanded}>
  <Handle 
    type="target" 
    position={Position.Top} 
    id="meta-target"
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
      <span class="item-id">{data.meta.lesson_id}</span>
    </div>
  </div>
  
  {#if isExpanded}
    <div class="node-content">
      <div class="field-group">
        <label>Lesson ID:</label>
        <input 
          type="text" 
          value={data.meta.lesson_id} 
          on:input={(e) => handleInputChange('lesson_id', e.target.value)}
        />
      </div>
      <div class="field-group">
        <label>Title:</label>
        <input 
          type="text" 
          value={data.meta.title} 
          on:input={(e) => handleInputChange('title', e.target.value)}
        />
      </div>
      <div class="field-group">
        <label>Date:</label>
        <input 
          type="date" 
          value={data.meta.date} 
          on:input={(e) => handleInputChange('date', e.target.value)}
        />
      </div>
      <div class="field-group">
        <label>Description:</label>
        <textarea 
          value={data.meta.description} 
          on:input={(e) => handleInputChange('description', e.target.value)}
        ></textarea>
      </div>
      <div class="field-group">
        <label>Seconds per Word:</label>
        <input 
          type="number" 
          step="0.1" 
          value={data.meta.seconds_per_word} 
          on:input={(e) => handleInputChange('seconds_per_word', parseFloat(e.target.value))}
        />
      </div>
    </div>
  {/if}
  
  <Handle 
    type="source" 
    position={Position.Bottom} 
    id="meta-source"
    style="background: #4a90e2;"
  />
</div>

<style>
  .meta-node {
    background: rgba(255,255,255,0.1);
    border: 1px solid rgba(255,255,255,0.2);
    border-radius: 8px;
    padding: 1rem;
    min-width: 250px;
    backdrop-filter: blur(10px);
    transition: all 0.3s ease;
  }

  .meta-node.collapsed {
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
</style> 