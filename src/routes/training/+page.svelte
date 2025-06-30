<script>
  import { goto } from '$app/navigation';
  import { onMount, onDestroy } from 'svelte';
  import { currentTrainingData, trainingSession, resetTrainingSession, globalSettings } from '$lib/stores.js';
  import ItemDisplay from '$lib/ItemDisplay.svelte';
  import { countWords, formatWordCount } from '$lib/utils.js';

  let isLoaded = $state(false);
  let currentItem = $state(null);
  let currentItemIndex = $state(0);
  let trainingData = $state(null);
  let debugMode = $state(false);
  let secondsPerWord = $state(0.5); // Default value

  function goBack() {
    console.log('Back button clicked - stopping training session');
    
    // Reset training session
    resetTrainingSession();
    
    // Navigate back to new session page
    console.log('Navigating back to /new');
    goto('/new');
  }

  function handleTransitionComplete() {
    console.log('Item transition complete, moving to next item');
    nextItem();
  }

  function nextItem() {
    console.log('Moving to next item. Current index:', currentItemIndex);
    
    if (!trainingData || !trainingData.items || trainingData.items.length === 0) {
      console.error('No training data available for next item');
      return;
    }
    
    let nextIndex = currentItemIndex + 1;
    
    // Loop back to first item if we've reached the end
    if (nextIndex >= trainingData.items.length) {
      console.log('Reached end of items, looping back to first item');
      nextIndex = 0;
    }
    
    console.log('Next item index:', nextIndex, 'Total items:', trainingData.items.length);
    
    // Update current item index
    currentItemIndex = nextIndex;
    
    // Update current item
    currentItem = trainingData.items[nextIndex];
    console.log('Current item updated:', currentItem?.item_id, currentItem?.text?.substring(0, 50) + '...');
    
    // Update session state
    trainingSession.update(s => ({
      ...s,
      currentItemIndex: nextIndex,
      isImageVisible: true,
      isTextVisible: false
    }));
  }

  onMount(() => {
    console.log('Training page mounted');
    
    setTimeout(() => {
      isLoaded = true;
    }, 100);

    // Subscribe to global settings for debug mode
    const unsubscribeSettings = globalSettings.subscribe(settings => {
      debugMode = settings.debugMode;
    });

    // Subscribe to training data changes
    const unsubscribeTraining = currentTrainingData.subscribe(data => {
      console.log('Training data updated:', data);
      
      if (data && data.items && data.items.length > 0) {
        trainingData = data;
        currentItem = data.items[0];
        currentItemIndex = 0;
        secondsPerWord = data.meta.seconds_per_word || 0.5;
        
        console.log('Training session started with', data.items.length, 'items');
        console.log('Seconds per word:', secondsPerWord);
        console.log('First item:', currentItem?.item_id, currentItem?.text?.substring(0, 50) + '...');
      } else {
        console.error('No valid training data received');
        alert('No training data available. Please try again.');
        goto('/new');
      }
    });

    return () => {
      unsubscribeSettings();
      unsubscribeTraining();
    };
  });

  onDestroy(() => {
    console.log('Training page destroyed');
  });
</script>

<svelte:head>
  <title>Training - FlashBrain</title>
</svelte:head>

<main class="page" class:loaded={isLoaded}>
  <div class="background-gradient"></div>
  
  <div class="content">
    <header class="header">
      <button class="back-button" onclick={goBack}>
        ← Back
      </button>
      <h1 class="title">Training Session</h1>
    </header>

    <div class="training-container">
      {#if currentItem}
        <ItemDisplay 
          item={currentItem}
          onTransitionComplete={handleTransitionComplete}
          autoStart={true}
          secondsPerWord={secondsPerWord}
        />
        
        <!-- Debug info - only show when debug mode is enabled -->
        {#if debugMode}
          <div class="debug-info">
            <p>Item {currentItemIndex + 1} of {trainingData?.items?.length || 0}</p>
            <p>Item ID: {currentItem.item_id}</p>
            <p>Word count: {formatWordCount(countWords(currentItem.text))}</p>
            <p>Seconds per word: {secondsPerWord}</p>
            <p>Debug mode: Enabled</p>
            <button class="debug-button" onclick={nextItem}>Manual Next Item</button>
          </div>
        {/if}
      {:else}
        <div class="loading-state">
          <div class="loading-spinner"></div>
          <p>Loading training data...</p>
          <p>Training data available: {trainingData ? 'Yes' : 'No'}</p>
          <p>Items count: {trainingData?.items?.length || 0}</p>
          {#if debugMode}
            <p>Debug mode: Enabled</p>
            <p>Seconds per word: {secondsPerWord}</p>
          {/if}
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
    margin-bottom: 2rem;
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
    font-size: clamp(2rem, 4vw, 3rem);
    font-weight: 900;
    line-height: 0.9;
    margin-bottom: 1rem;
    letter-spacing: -0.02em;
    background: linear-gradient(135deg, #fff 0%, #a0a0a0 100%);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
  }

  .training-container {
    display: flex;
    justify-content: center;
    align-items: center;
    min-height: 60vh;
    position: relative;
  }

  .loading-state {
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

  .debug-info {
    position: fixed;
    bottom: 2rem;
    left: 2rem;
    background: rgba(0,0,0,0.8);
    color: #fff;
    padding: 1rem;
    border-radius: 8px;
    font-size: 0.9rem;
    max-width: 300px;
    z-index: 1000;
  }

  .debug-info p {
    margin: 0.25rem 0;
    font-family: monospace;
  }

  .debug-button {
    background: #007acc;
    color: white;
    border: none;
    padding: 0.5rem 1rem;
    border-radius: 4px;
    cursor: pointer;
    margin-top: 0.5rem;
    font-size: 0.8rem;
  }

  .debug-button:hover {
    background: #005a9e;
  }

  @keyframes spin {
    0% { transform: rotate(0deg); }
    100% { transform: rotate(360deg); }
  }

  @media (max-width: 768px) {
    .content {
      padding: 1rem;
    }
    
    .back-button {
      top: 1rem;
      left: 1rem;
    }
  }
</style> 