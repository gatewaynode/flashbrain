<script>
  import { goto } from '$app/navigation';
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';

  let isLoaded = $state(false);
  let learningPaths = $state([]);
  let isLoading = $state(true);
  let error = $state(null);

  function goBack() {
    goto('/');
  }

  async function loadLearningPaths() {
    try {
      isLoading = true;
      error = null;
      console.log('Calling get_learning_paths command...');
      const paths = await invoke('get_learning_paths');
      console.log('Received paths:', paths);
      learningPaths = paths;
    } catch (err) {
      console.error('Full error object:', err);
      console.error('Error type:', typeof err);
      console.error('Error message:', err?.message);
      console.error('Error toString:', err?.toString());
      error = err?.message || err?.toString() || 'Failed to load learning paths';
    } finally {
      isLoading = false;
    }
  }

  function formatDate(dateString) {
    try {
      const date = new Date(dateString);
      return date.toLocaleDateString('en-US', {
        year: 'numeric',
        month: 'long',
        day: 'numeric'
      });
    } catch {
      return dateString;
    }
  }

  function selectLearningPath(pathId) {
    // TODO: Navigate to the learning session with the selected path
    console.log('Selected learning path:', pathId);
    // For now, just show an alert
    alert(`Starting session: ${pathId}`);
  }

  onMount(() => {
    setTimeout(() => {
      isLoaded = true;
    }, 100);
    
    // Test JSON parsing first
    testJsonParsing();
    loadLearningPaths();
  });

  async function testJsonParsing() {
    try {
      console.log('Testing JSON parsing...');
      const result = await invoke('test_json_parsing');
      console.log('JSON parsing test result:', result);
    } catch (err) {
      console.error('JSON parsing test failed:', err);
    }
  }
</script>

<svelte:head>
  <title>New Session - FlashBrain</title>
</svelte:head>

<main class="page" class:loaded={isLoaded}>
  <div class="background-gradient"></div>
  
  <div class="content">
    <header class="header">
      <button class="back-button" onclick={goBack}>
        ← Back
      </button>
      <h1 class="title">New Session</h1>
      <p class="subtitle">Choose a learning path to begin</p>
    </header>

    <div class="learning-paths-container">
      {#if isLoading}
        <div class="loading-state">
          <div class="loading-spinner"></div>
          <p>Loading learning paths...</p>
        </div>
      {:else if error}
        <div class="error-state">
          <h3>Error Loading Paths</h3>
          <p>{error}</p>
          <button class="retry-button" onclick={loadLearningPaths}>
            Try Again
          </button>
        </div>
      {:else if learningPaths.length === 0}
        <div class="empty-state">
          <h3>No Learning Paths Available</h3>
          <p>No training data found in the static/classes directory.</p>
          <p>Add JSON files with training data to get started.</p>
        </div>
      {:else}
        <div class="paths-grid">
          {#each learningPaths as path}
            <button 
              class="path-card" 
              onclick={() => selectLearningPath(path.id)}
              onkeydown={(e) => e.key === 'Enter' && selectLearningPath(path.id)}
              aria-label="Start learning session: {path.title}"
            >
              <div class="path-header">
                <h3 class="path-title">{path.title}</h3>
                <span class="path-date">{formatDate(path.date)}</span>
              </div>
              <p class="path-description">{path.description}</p>
              <div class="path-actions">
                <span class="start-button">Start Learning</span>
              </div>
            </button>
          {/each}
        </div>
      {/if}
    </div>
  </div>
</main>

<style>
  .page {
    min-height: 100vh;
    display: flex;
    align-items: center;
    justify-content: center;
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

  .content {
    text-align: center;
    max-width: 1200px;
    padding: 2rem;
    width: 100%;
  }

  .header {
    margin-bottom: 3rem;
  }

  .back-button {
    position: absolute;
    top: 2rem;
    left: 2rem;
    background: transparent;
    border: 1px solid rgba(255,255,255,0.2);
    color: #fff;
    padding: 0.75rem 1.5rem;
    border-radius: 8px;
    cursor: pointer;
    font-size: 0.9rem;
    transition: all 0.3s ease;
  }

  .back-button:hover {
    background: rgba(255,255,255,0.1);
    border-color: rgba(255,255,255,0.3);
  }

  .title {
    font-size: clamp(2.5rem, 6vw, 4rem);
    font-weight: 900;
    line-height: 0.9;
    margin-bottom: 1rem;
    letter-spacing: -0.02em;
    background: linear-gradient(135deg, #fff 0%, #a0a0a0 100%);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
  }

  .subtitle {
    font-size: 1.1rem;
    color: #888;
    font-weight: 400;
    letter-spacing: 0.02em;
  }

  .learning-paths-container {
    display: flex;
    justify-content: center;
    align-items: center;
    min-height: 400px;
  }

  .loading-state, .error-state, .empty-state {
    text-align: center;
    color: #aaa;
  }

  .loading-spinner {
    width: 40px;
    height: 40px;
    border: 3px solid rgba(255,255,255,0.1);
    border-top: 3px solid #fff;
    border-radius: 50%;
    animation: spin 1s linear infinite;
    margin: 0 auto 1rem;
  }

  @keyframes spin {
    0% { transform: rotate(0deg); }
    100% { transform: rotate(360deg); }
  }

  .retry-button {
    background: linear-gradient(135deg, #fff 0%, #a0a0a0 100%);
    color: #000;
    border: none;
    padding: 0.75rem 1.5rem;
    border-radius: 8px;
    cursor: pointer;
    font-size: 0.9rem;
    font-weight: 600;
    margin-top: 1rem;
    transition: all 0.3s ease;
  }

  .retry-button:hover {
    transform: translateY(-2px);
    box-shadow: 0 8px 25px rgba(255,255,255,0.2);
  }

  .paths-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(350px, 1fr));
    gap: 1.5rem;
    width: 100%;
    max-width: 1000px;
  }

  .path-card {
    background: linear-gradient(135deg, rgba(255,255,255,0.05) 0%, rgba(255,255,255,0.02) 100%);
    border: 1px solid rgba(255,255,255,0.1);
    border-radius: 16px;
    padding: 2rem;
    text-align: left;
    cursor: pointer;
    transition: all 0.3s cubic-bezier(0.16, 1, 0.3, 1);
    position: relative;
    overflow: hidden;
    font-family: inherit;
    color: inherit;
  }

  .path-card::before {
    content: '';
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: linear-gradient(135deg, rgba(255,255,255,0.05) 0%, rgba(255,255,255,0.02) 100%);
    border-radius: 16px;
    opacity: 0;
    transition: all 0.3s ease;
  }

  .path-card:hover {
    transform: translateY(-4px);
    border-color: rgba(255,255,255,0.2);
  }

  .path-card:hover::before {
    opacity: 1;
  }

  .path-card:focus {
    outline: 2px solid rgba(255,255,255,0.3);
    outline-offset: 2px;
  }

  .path-header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    margin-bottom: 1rem;
  }

  .path-title {
    font-size: 1.4rem;
    font-weight: 700;
    color: #fff;
    margin: 0;
    flex: 1;
  }

  .path-date {
    font-size: 0.85rem;
    color: #888;
    font-weight: 400;
    white-space: nowrap;
    margin-left: 1rem;
  }

  .path-description {
    color: #aaa;
    line-height: 1.6;
    margin-bottom: 1.5rem;
    font-size: 0.95rem;
  }

  .path-actions {
    display: flex;
    justify-content: flex-end;
  }

  .start-button {
    background: linear-gradient(135deg, #fff 0%, #a0a0a0 100%);
    color: #000;
    padding: 0.5rem 1rem;
    border-radius: 6px;
    font-size: 0.85rem;
    font-weight: 600;
    transition: all 0.3s ease;
  }

  .path-card:hover .start-button {
    transform: scale(1.05);
  }

  @media (max-width: 768px) {
    .content {
      padding: 1rem;
    }
    
    .back-button {
      top: 1rem;
      left: 1rem;
    }
    
    .paths-grid {
      grid-template-columns: 1fr;
      gap: 1rem;
    }
    
    .path-card {
      padding: 1.5rem;
    }
    
    .path-header {
      flex-direction: column;
      align-items: flex-start;
    }
    
    .path-date {
      margin-left: 0;
      margin-top: 0.5rem;
    }
  }
</style> 