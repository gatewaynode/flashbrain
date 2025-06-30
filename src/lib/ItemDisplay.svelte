<script>
  import { onMount, onDestroy } from 'svelte';
  import { countWords, calculateTextDisplayDuration, formatWordCount, splitTextIntoWords, calculateWordHighlightDuration } from '$lib/utils.js';

  // Props
  export let item = null;
  export let onTransitionComplete = null;
  export let autoStart = true;
  export let imageDisplayTime = 2000; // 2 seconds
  export let fadeDuration = 1000; // 1 second
  export let secondsPerWord = 0.5; // Default: 0.5 seconds per word
  export let minTextDuration = 2000; // Minimum 2 seconds
  export let maxTextDuration = 10000; // Maximum 10 seconds

  // State
  let imageOpacity = 1;
  let textOpacity = 0;
  let isTransitioning = false;
  let fadeInterval = null;
  let timeoutId = null;
  let wordCount = 0;
  let calculatedTextDuration = 3000; // Default fallback
  
  // Word highlighting state
  let words = [];
  let currentWordIndex = -1;
  let wordHighlightInterval = null;
  let wordHighlightDuration = 0;

  // Animation constants
  const IMAGE_FADE_TARGET = 0.3; // 30% transparency
  const TEXT_FADE_TARGET = 0.7; // 70% transparency
  const ANIMATION_STEPS = 60; // Smooth animation

  function calculateDuration() {
    if (item && item.text) {
      wordCount = countWords(item.text);
      calculatedTextDuration = calculateTextDisplayDuration(
        item.text, 
        secondsPerWord, 
        minTextDuration, 
        maxTextDuration
      );
      console.log(`Text: "${item.text.substring(0, 50)}..."`);
      console.log(`Word count: ${wordCount} (${formatWordCount(wordCount)})`);
      console.log(`Seconds per word: ${secondsPerWord}`);
      console.log(`Calculated duration: ${calculatedTextDuration}ms`);
      
      // Split text into words for highlighting using utility function
      words = splitTextIntoWords(item.text);
      wordHighlightDuration = calculateWordHighlightDuration(calculatedTextDuration, words.length);
      console.log(`Words array:`, words);
      console.log(`Word highlight duration: ${wordHighlightDuration}ms per word`);
    }
  }

  function startWordHighlighting() {
    if (words.length === 0) {
      console.log('No words to highlight');
      return;
    }
    
    console.log('Starting word highlighting sequence');
    currentWordIndex = 0;
    
    wordHighlightInterval = setInterval(() => {
      currentWordIndex++;
      
      if (currentWordIndex >= words.length) {
        // Word highlighting complete
        clearInterval(wordHighlightInterval);
        wordHighlightInterval = null;
        console.log('Word highlighting sequence complete');
      } else {
        console.log(`Highlighting word ${currentWordIndex + 1}/${words.length}: "${words[currentWordIndex]}"`);
      }
    }, wordHighlightDuration);
  }

  function stopWordHighlighting() {
    if (wordHighlightInterval) {
      clearInterval(wordHighlightInterval);
      wordHighlightInterval = null;
    }
    currentWordIndex = -1;
  }

  function startTransition() {
    if (isTransitioning || !item) {
      console.log('Transition already in progress or no item available');
      return;
    }
    
    console.log('Starting refined crossfade transition for item:', item.item_id);
    isTransitioning = true;
    
    const stepDuration = fadeDuration / ANIMATION_STEPS;
    const imageOpacityStep = (1 - IMAGE_FADE_TARGET) / ANIMATION_STEPS;
    const textOpacityStep = TEXT_FADE_TARGET / ANIMATION_STEPS;
    
    let step = 0;
    
    fadeInterval = setInterval(() => {
      step++;
      
      // Image fades from 1.0 to 0.3 (30% transparency)
      imageOpacity = 1 - (step * imageOpacityStep);
      
      // Text fades from 0.0 to 0.7 (70% transparency)
      textOpacity = step * textOpacityStep;
      
      if (step >= ANIMATION_STEPS) {
        clearInterval(fadeInterval);
        fadeInterval = null;
        isTransitioning = false;
        
        console.log('Crossfade transition complete, starting word highlighting');
        
        // Start word highlighting after crossfade completes
        startWordHighlighting();
        
        // Wait for calculated text display time before calling completion callback
        timeoutId = setTimeout(() => {
          if (onTransitionComplete) {
            onTransitionComplete();
          }
        }, calculatedTextDuration);
      }
    }, stepDuration);
  }

  function resetDisplay() {
    // Clear any active intervals/timeouts
    if (fadeInterval) {
      clearInterval(fadeInterval);
      fadeInterval = null;
    }
    if (timeoutId) {
      clearTimeout(timeoutId);
      timeoutId = null;
    }
    if (wordHighlightInterval) {
      clearInterval(wordHighlightInterval);
      wordHighlightInterval = null;
    }
    
    // Reset state
    isTransitioning = false;
    imageOpacity = 1;
    textOpacity = 0;
    currentWordIndex = -1;
    words = [];
  }

  function manualStart() {
    if (!isTransitioning) {
      startTransition();
    }
  }

  // Watch for item changes
  $: if (item) {
    console.log('Item changed, resetting display:', item.item_id);
    resetDisplay();
    calculateDuration();
    
    if (autoStart) {
      // Start transition after image display time
      timeoutId = setTimeout(() => {
        startTransition();
      }, imageDisplayTime);
    }
  }

  onMount(() => {
    console.log('ItemDisplay component mounted');
  });

  onDestroy(() => {
    console.log('ItemDisplay component destroyed - cleaning up');
    if (fadeInterval) {
      clearInterval(fadeInterval);
    }
    if (timeoutId) {
      clearTimeout(timeoutId);
    }
    if (wordHighlightInterval) {
      clearInterval(wordHighlightInterval);
    }
  });
</script>

<div class="item-display">
  <!-- Image with refined fade -->
  <div 
    class="image-container" 
    style="opacity: {imageOpacity};"
  >
    <img 
      src={item?.image} 
      alt="Training image"
      class="training-image"
    />
  </div>
  
  <!-- Text with refined background fade and word highlighting -->
  <div 
    class="text-container" 
    style="opacity: {textOpacity};"
  >
    <p class="training-text">
      {#each words as word, index}
        <span 
          class="word" 
          class:highlighted={index === currentWordIndex}
        >
          {word}
        </span>
        {#if index < words.length - 1}
          <span class="word-separator"> </span>
        {/if}
      {/each}
    </p>
  </div>
</div>

<style>
  .item-display {
    position: relative;
    width: 100%;
    max-width: 800px;
    height: 500px;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .image-container, .text-container {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: opacity 0.1s ease;
  }

  .training-image {
    max-width: 100%;
    max-height: 100%;
    object-fit: contain;
    border-radius: 12px;
  }

  .training-text {
    font-size: clamp(1.2rem, 3vw, 2rem);
    line-height: 1.6;
    color: #fff;
    text-align: center;
    max-width: 90%;
    font-weight: 400;
    padding: 2rem;
    background: rgba(0,0,0,0.7);
    border-radius: 12px;
    backdrop-filter: blur(10px);
  }

  .word {
    display: inline-block;
    transition: all 0.2s ease;
    font-weight: 400;
  }

  .word.highlighted {
    font-weight: 700;
    color: #fff;
    text-shadow: 0 0 10px rgba(255,255,255,0.5);
  }

  .word-separator {
    white-space: pre;
  }

  @media (max-width: 768px) {
    .item-display {
      height: 400px;
    }
    
    .training-text {
      font-size: 1.1rem;
      padding: 1.5rem;
    }
  }
</style> 