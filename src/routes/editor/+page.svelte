<script>
  import { goto } from '$app/navigation';
  import { onMount } from 'svelte';
  import { page } from '$app/stores';

  let isLoaded = $state(false);
  let mode = $state('');
  let lessonName = $state('');
  let filePath = $state('');

  function goBack() {
    goto('/create');
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
  });
</script>

<svelte:head>
  <title>Lesson Editor - FlashBrain</title>
</svelte:head>

<main class="page" class:loaded={isLoaded}>
  <div class="background-gradient"></div>
  
  <div class="content">
    <header class="header">
      <button class="back-button" onclick={goBack}>
        ← Back
      </button>
      <h1 class="title">Lesson Editor</h1>
      <p class="subtitle">Create and edit your lessons</p>
    </header>

    <div class="editor-container">
      <div class="editor-card">
        <h2>Editor Status</h2>
        
        <div class="status-info">
          <div class="status-item">
            <span class="status-label">Mode:</span>
            <span class="status-value">{mode || 'Unknown'}</span>
          </div>
          
          {#if lessonName}
            <div class="status-item">
              <span class="status-label">Lesson Name:</span>
              <span class="status-value">{lessonName}</span>
            </div>
          {/if}
          
          {#if filePath}
            <div class="status-item">
              <span class="status-label">File Path:</span>
              <span class="status-value">{filePath}</span>
            </div>
          {/if}
        </div>

        <div class="editor-placeholder">
          <h3>Editor Coming Soon</h3>
          <p>This is where you'll be able to:</p>
          <ul class="feature-list">
            <li>Add and edit lesson items</li>
            <li>Upload and manage images</li>
            <li>Set text content and timing</li>
            <li>Configure lesson metadata</li>
            <li>Preview your lessons</li>
          </ul>
        </div>

        <div class="editor-actions">
          <button class="action-button secondary" onclick={goBack}>
            Cancel
          </button>
          <button class="action-button primary" disabled>
            Save Lesson
          </button>
        </div>
      </div>
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
    max-width: 1000px;
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

  .editor-container {
    display: flex;
    justify-content: center;
  }

  .editor-card {
    background: linear-gradient(135deg, rgba(255,255,255,0.05) 0%, rgba(255,255,255,0.02) 100%);
    border: 1px solid rgba(255,255,255,0.1);
    border-radius: 16px;
    padding: 3rem;
    max-width: 800px;
    width: 100%;
    text-align: left;
  }

  .editor-card h2 {
    font-size: 1.8rem;
    font-weight: 700;
    margin-bottom: 2rem;
    color: #fff;
    text-align: center;
  }

  .status-info {
    background: rgba(255,255,255,0.03);
    border: 1px solid rgba(255,255,255,0.05);
    border-radius: 8px;
    padding: 1.5rem;
    margin-bottom: 2rem;
  }

  .status-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0.5rem 0;
    border-bottom: 1px solid rgba(255,255,255,0.05);
  }

  .status-item:last-child {
    border-bottom: none;
  }

  .status-label {
    color: #888;
    font-weight: 500;
  }

  .status-value {
    color: #fff;
    font-weight: 600;
    font-family: monospace;
    background: rgba(255,255,255,0.1);
    padding: 0.25rem 0.5rem;
    border-radius: 4px;
  }

  .editor-placeholder {
    text-align: center;
    margin-bottom: 2rem;
  }

  .editor-placeholder h3 {
    font-size: 1.4rem;
    font-weight: 600;
    margin-bottom: 1rem;
    color: #fff;
  }

  .editor-placeholder p {
    color: #aaa;
    margin-bottom: 1rem;
  }

  .feature-list {
    list-style: none;
    padding: 0;
    margin: 0;
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .feature-list li {
    color: #888;
    font-size: 0.95rem;
    padding: 0.5rem;
    background: rgba(255,255,255,0.02);
    border-radius: 6px;
    border-left: 3px solid rgba(255,255,255,0.1);
  }

  .editor-actions {
    display: flex;
    gap: 1rem;
    justify-content: center;
    margin-top: 2rem;
  }

  .action-button {
    padding: 0.75rem 1.5rem;
    border-radius: 8px;
    font-size: 0.9rem;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.3s ease;
    border: none;
  }

  .action-button.primary {
    background: linear-gradient(135deg, #fff 0%, #a0a0a0 100%);
    color: #000;
  }

  .action-button.primary:disabled {
    background: rgba(255,255,255,0.1);
    color: #666;
    cursor: not-allowed;
  }

  .action-button.secondary {
    background: transparent;
    color: #fff;
    border: 1px solid rgba(255,255,255,0.2);
  }

  .action-button:hover:not(:disabled) {
    transform: translateY(-2px);
    box-shadow: 0 4px 15px rgba(255,255,255,0.2);
  }

  .action-button.secondary:hover {
    background: rgba(255,255,255,0.1);
    border-color: rgba(255,255,255,0.3);
  }

  @media (max-width: 768px) {
    .content {
      padding: 1rem;
    }
    
    .back-button {
      top: 1rem;
      left: 1rem;
    }
    
    .editor-card {
      padding: 2rem;
    }

    .status-item {
      flex-direction: column;
      align-items: flex-start;
      gap: 0.5rem;
    }

    .editor-actions {
      flex-direction: column;
    }
  }
</style> 