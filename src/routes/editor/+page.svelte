<script>
  import { goto } from '$app/navigation';
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { SvelteFlow, Background, Controls, MiniMap } from '@xyflow/svelte';
  import '@xyflow/svelte/dist/style.css';
  import MetaNode from '$lib/MetaNode.svelte';
  import ItemNode from '$lib/ItemNode.svelte';
  import CustomEdge from '$lib/CustomEdge.svelte';

  let isLoaded = $state(false);
  let mode = $state('');
  let lessonName = $state('');
  let filePath = $state('');
  let lessonData = $state(null);
  let isLoading = $state(true);
  let error = $state(null);

  // Svelte-Flow state
  let nodes = $state([]);
  let edges = $state([]);
  let nodeTypes = $state({
    metaNode: MetaNode,
    itemNode: ItemNode
  });
  let edgeTypes = $state({
    customEdge: CustomEdge
  });

  function goBack() {
    goto('/create');
  }

  async function loadLessonData() {
    if (!filePath) return;
    
    try {
      isLoading = true;
      error = null;
      console.log('Loading lesson data from:', filePath);
      
      // Extract class_id from file path
      const pathParts = filePath.split('/');
      const classId = pathParts[pathParts.length - 2]; // Get directory name
      
      const data = await invoke('load_training_data', { classId });
      console.log('Loaded lesson data:', data);
      lessonData = data;
      
      // Convert lesson data to Svelte-Flow nodes
      createNodesFromData(data);
    } catch (err) {
      console.error('Failed to load lesson data:', err);
      error = err?.message || err?.toString() || 'Failed to load lesson data';
    } finally {
      isLoading = false;
    }
  }

  function createNodesFromData(data) {
    const newNodes = [];
    const newEdges = [];
    
    console.log('Creating nodes from data:', data);
    
    // Create metadata node (parent)
    const metaNode = {
      id: 'meta',
      type: 'metaNode',
      position: { x: 400, y: 100 },
      data: {
        label: 'Metadata',
        meta: data.meta
      }
    };
    newNodes.push(metaNode);
    console.log('Created meta node:', metaNode);
    
    // Create item nodes (children) with better spacing
    data.items.forEach((item, index) => {
      const itemNode = {
        id: `item-${item.item_id}`,
        type: 'itemNode',
        position: { x: 400, y: 350 + (index * 300) }, // Increased spacing from 150 to 300
        data: {
          label: `Item ${item.item_id}`,
          item: item
        }
      };
      newNodes.push(itemNode);
      console.log(`Created item node ${index}:`, itemNode);
      
      // Create edge from metadata to first item, or from previous item to current item
      if (index === 0) {
        const edge = {
          id: `edge-meta-${item.item_id}`,
          source: 'meta',
          target: `item-${item.item_id}`,
          type: 'customEdge',
          style: {
            stroke: '#ff0000',
            strokeWidth: 4,
            strokeDasharray: '10,5'
          },
          animated: true
        };
        newEdges.push(edge);
        console.log('Created edge from meta to first item:', edge);
      } else {
        const prevItem = data.items[index - 1];
        console.log('prevItem', prevItem);
        console.log('item', item);
        const edge = {
          id: `edge-${prevItem.item_id}-${item.item_id}`,
          source: `item-${prevItem.item_id}`,
          target: `item-${item.item_id}`,
          type: 'customEdge',
          style: {
            stroke: '#ff0000',
            strokeWidth: 4,
            strokeDasharray: '10,5'
          },
          animated: true
        };
        newEdges.push(edge);
        console.log(`Created edge from item ${prevItem.item_id} to item ${item.item_id}:`, edge);
      }
    });
    
    console.log('Final nodes array:', newNodes);
    console.log('Final edges array:', newEdges);
    
    nodes = newNodes;
    edges = newEdges;
  }

  function handleNodeUpdate(event) {
    const { id, data } = event.detail;
    nodes = nodes.map(node => 
      node.id === id ? { ...node, data } : node
    );
  }

  function handleImageSelect(event) {
    const { itemId } = event.detail;
    console.log('Opening image selector for item:', itemId);
    // TODO: Implement file dialog for image selection
  }

  function handleFlowError(error) {
    console.error('SvelteFlow error:', error);
    console.error('Error details:', {
      message: error.message,
      type: error.type,
      source: error.source,
      target: error.target
    });
  }

  onMount(() => {
    setTimeout(() => {
      isLoaded = true;
    }, 100);

    // Parse URL parameters
    const urlParams = new URLSearchParams(window.location.search);
    mode = urlParams.get('mode') || '';
    lessonName = urlParams.get('name') || '';
    filePath = urlParams.get('file') || '';

    console.log('Editor mode:', mode);
    console.log('Lesson name:', lessonName);
    console.log('File path:', filePath);

    // Load lesson data if we have a file path
    if (filePath) {
      loadLessonData();
    }
  });
</script>

<svelte:head>
  <title>Lesson Editor - FlashBrain</title>
</svelte:head>

<main class="page" class:loaded={isLoaded}>
  <div class="background-gradient"></div>
  
  <!-- Top Menu Bar -->
  <div class="menu-bar">
    <div class="menu-left">
      <button class="menu-button" onclick={goBack}>
        ← Back
      </button>
      <span class="menu-title">Lesson Editor</span>
    </div>
    <div class="menu-center">
      <button class="menu-button">File</button>
      <button class="menu-button">Edit</button>
      <button class="menu-button">View</button>
      <button class="menu-button">Help</button>
    </div>
    <div class="menu-right">
      <button class="menu-button primary">Save</button>
    </div>
  </div>

  <div class="editor-layout">
    <!-- Left Sidebar - Tools Panel -->
    <div class="tools-panel">
      <h3>Tools</h3>
      <div class="tool-group">
        <button class="tool-button">Add Item</button>
        <button class="tool-button">Delete Item</button>
        <button class="tool-button">Duplicate Item</button>
      </div>
      <div class="tool-group">
        <button class="tool-button">Import Image</button>
        <button class="tool-button">Export JSON</button>
      </div>
      <div class="tool-group">
        <h4>Properties</h4>
        <div class="property-item">
          <label>Mode: {mode}</label>
        </div>
        {#if lessonName}
          <div class="property-item">
            <label>Name: {lessonName}</label>
          </div>
        {/if}
      </div>
    </div>

    <!-- Main Editor Area -->
    <div class="editor-main">
      {#if isLoading}
        <div class="loading-overlay">
          <div class="loading-spinner"></div>
          <p>Loading lesson data...</p>
        </div>
      {:else if error}
        <div class="error-overlay">
          <h3>Error Loading Lesson</h3>
          <p>{error}</p>
          <button class="retry-button" onclick={loadLessonData}>
            Try Again
          </button>
        </div>
      {:else if !filePath}
        <div class="welcome-overlay">
          <h3>Welcome to the Lesson Editor</h3>
          <p>Select a lesson file to edit or create a new lesson.</p>
        </div>
      {:else}
        <div class="flow-container">
          <SvelteFlow
            {nodes}
            {edges}
            {nodeTypes}
            {edgeTypes}
            on:nodeUpdate={handleNodeUpdate}
            on:imageSelect={handleImageSelect}
            on:error={handleFlowError}
            fitView
            colorMode="system"
            class="flow-canvas"
          >
            <Background />
            <Controls />
            <MiniMap />
          </SvelteFlow>
        </div>
      {/if}
    </div>
  </div>
</main>

<style>
  .page {
    min-height: 100vh;
    display: flex;
    flex-direction: column;
    position: relative;
    opacity: 0;
    transform: translateY(20px);
    transition: all 0.8s cubic-bezier(0.16, 1, 0.3, 1);
  }

  .page.loaded {
    opacity: 1;
    transform: translateY(0);
  }

  .background-gradient {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: linear-gradient(135deg, #0a0a0a 0%, #1a1a1a 50%, #0a0a0a 100%);
    z-index: -1;
  }

  /* Menu Bar */
  .menu-bar {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0.5rem 1rem;
    background: rgba(255,255,255,0.05);
    border-bottom: 1px solid rgba(255,255,255,0.1);
    backdrop-filter: blur(10px);
  }

  .menu-left, .menu-center, .menu-right {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .menu-title {
    color: #fff;
    font-weight: 600;
    margin-left: 1rem;
  }

  .menu-button {
    background: transparent;
    border: 1px solid rgba(255,255,255,0.2);
    color: #fff;
    padding: 0.5rem 1rem;
    border-radius: 4px;
    cursor: pointer;
    font-size: 0.9rem;
    transition: all 0.3s ease;
  }

  .menu-button:hover {
    background: rgba(255,255,255,0.1);
  }

  .menu-button.primary {
    background: linear-gradient(135deg, #4a90e2 0%, #357abd 100%);
    border-color: transparent;
  }

  /* Editor Layout */
  .editor-layout {
    display: flex;
    flex: 1;
    height: calc(100vh - 60px);
  }

  /* Tools Panel */
  .tools-panel {
    width: 250px;
    background: rgba(255,255,255,0.03);
    border-right: 1px solid rgba(255,255,255,0.1);
    padding: 1rem;
    overflow-y: auto;
  }

  .tools-panel h3 {
    color: #fff;
    margin-bottom: 1rem;
    font-size: 1.1rem;
  }

  .tool-group {
    margin-bottom: 1.5rem;
  }

  .tool-group h4 {
    color: #bebebe;
    margin-bottom: 0.5rem;
    font-size: 1.1rem;
  }

  .tool-group:label {
    color: #bebebe;
    margin-bottom: 0.5rem;
    font-size: 1.1rem;
  }

  .tool-button {
    display: block;
    width: 100%;
    background: rgba(255,255,255,0.05);
    border: 1px solid rgba(255,255,255,0.1);
    color: #fff;
    padding: 0.75rem;
    border-radius: 6px;
    cursor: pointer;
    margin-bottom: 0.5rem;
    transition: all 0.3s ease;
  }

  .tool-button:hover {
    background: rgba(255,255,255,0.1);
  }

  .property-item {
    margin-bottom: 0.5rem;
  }

  .property-item label {
    color: #aaa;
    font-size: 0.9rem;
  }

  /* Main Editor Area */
  .editor-main {
    flex: 1;
    position: relative;
  }

  .flow-container {
    width: 100%;
    height: 100%;
  }

  .flow-canvas {
    background: #000000;
  }

  /* Overlays */
  .loading-overlay, .error-overlay, .welcome-overlay {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    background: rgba(0,0,0,0.8);
    color: #fff;
  }

  .loading-spinner {
    width: 40px;
    height: 40px;
    border: 3px solid rgba(255,255,255,0.1);
    border-top: 3px solid #fff;
    border-radius: 50%;
    animation: spin 1s linear infinite;
    margin-bottom: 1rem;
  }

  @keyframes spin {
    0% { transform: rotate(0deg); }
    100% { transform: rotate(360deg); }
  }

  .retry-button {
    background: linear-gradient(135deg, #4a90e2 0%, #357abd 100%);
    color: #fff;
    border: none;
    padding: 0.75rem 1.5rem;
    border-radius: 6px;
    cursor: pointer;
    margin-top: 1rem;
  }
</style> 