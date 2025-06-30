<script>
  import { goto } from '$app/navigation';
  import { onMount } from 'svelte';
  import { globalSettings, toggleDebugMode, updateLessonsFilePath } from '$lib/stores.js';

  let isLoaded = $state(false);
  let debugMode = $state(false);
  let lessonsFilePath = $state('~/Documents/flashbrain');
  let showPathInput = $state(false);
  let tempPath = $state('');

  function goBack() {
    goto('/');
  }

  function handleDebugToggle() {
    toggleDebugMode();
  }

  function handlePathEdit() {
    tempPath = lessonsFilePath;
    showPathInput = true;
  }

  function handlePathSave() {
    if (tempPath.trim()) {
      updateLessonsFilePath(tempPath.trim());
      lessonsFilePath = tempPath.trim();
      showPathInput = false;
    }
  }

  function handlePathCancel() {
    showPathInput = false;
    tempPath = lessonsFilePath;
  }

  onMount(() => {
    setTimeout(() => {
      isLoaded = true;
    }, 100);

    // Subscribe to global settings
    const unsubscribe = globalSettings.subscribe(settings => {
      debugMode = settings.debugMode;
      lessonsFilePath = settings.lessonsFilePath;
    });

    return unsubscribe;
  });
</script>

<svelte:head>
  <title>Settings - FlashBrain</title>
</svelte:head>

<main class="page" class:loaded={isLoaded}>
  <div class="background-gradient"></div>
  
  <div class="content">
    <header class="header">
      <button class="back-button" onclick={goBack}>
        ← Back
      </button>
      <h1 class="title">Settings</h1>
      <p class="subtitle">Customize your experience</p>
    </header>

    <div class="placeholder-content">
      <div class="placeholder-card">
        <h2>Preferences</h2>
        <p>Configure your learning environment and personalize your FlashBrain experience.</p>
        <div class="placeholder-features">
          <div class="feature">🎨 Theme and appearance</div>
          <div class="feature">⏱️ Timing and speed settings</div>
          <div class="feature">🔊 Audio and accessibility</div>
          <div class="feature">🐛 Debug and development tools</div>
          <div class="feature">📁 File and storage management</div>
        </div>
        <div class="settings-grid">
          <div class="setting-item">
            <span class="setting-label">Dark Mode</span>
            <div class="setting-toggle">
              <div class="toggle-slider"></div>
            </div>
          </div>
          <div class="setting-item">
            <span class="setting-label">Auto-play</span>
            <div class="setting-toggle">
              <div class="toggle-slider"></div>
            </div>
          </div>
          <div class="setting-item">
            <span class="setting-label">Sound Effects</span>
            <div class="setting-toggle">
              <div class="toggle-slider"></div>
            </div>
          </div>
          <div class="setting-item">
            <span class="setting-label">Debug Mode</span>
            <div 
              class="setting-toggle" 
              class:active={debugMode}
              onclick={handleDebugToggle}
            >
              <div class="toggle-slider"></div>
            </div>
          </div>
          <div class="setting-item">
            <span class="setting-label">Lessons File Path</span>
            <div class="path-display">
              {#if showPathInput}
                <div class="path-input-group">
                  <input 
                    type="text" 
                    bind:value={tempPath}
                    class="path-input"
                    placeholder="Enter lessons directory path"
                    onkeydown={(e) => e.key === 'Enter' && handlePathSave()}
                  />
                  <div class="path-actions">
                    <button class="path-button save" onclick={handlePathSave}>✓</button>
                    <button class="path-button cancel" onclick={handlePathCancel}>✕</button>
                  </div>
                </div>
              {:else}
                <span class="path-value">{lessonsFilePath}</span>
                <button class="path-edit-button" onclick={handlePathEdit}>Edit</button>
              {/if}
            </div>
          </div>
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
    max-width: 800px;
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

  .placeholder-content {
    display: flex;
    justify-content: center;
  }

  .placeholder-card {
    background: linear-gradient(135deg, rgba(255,255,255,0.05) 0%, rgba(255,255,255,0.02) 100%);
    border: 1px solid rgba(255,255,255,0.1);
    border-radius: 16px;
    padding: 3rem;
    max-width: 500px;
    text-align: center;
  }

  .placeholder-card h2 {
    font-size: 1.8rem;
    font-weight: 700;
    margin-bottom: 1rem;
    color: #fff;
  }

  .placeholder-card p {
    color: #aaa;
    line-height: 1.6;
    margin-bottom: 2rem;
  }

  .placeholder-features {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
    margin-bottom: 2rem;
  }

  .feature {
    color: #888;
    font-size: 0.95rem;
  }

  .settings-grid {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    margin-top: 2rem;
  }

  .setting-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1rem;
    background: rgba(255,255,255,0.03);
    border-radius: 8px;
    border: 1px solid rgba(255,255,255,0.05);
  }

  .setting-label {
    color: #fff;
    font-size: 0.95rem;
    font-weight: 500;
  }

  .setting-toggle {
    width: 50px;
    height: 24px;
    background: rgba(255,255,255,0.1);
    border-radius: 12px;
    position: relative;
    cursor: pointer;
    transition: all 0.3s ease;
  }

  .setting-toggle.active {
    background: #007acc;
  }

  .toggle-slider {
    width: 20px;
    height: 20px;
    background: #fff;
    border-radius: 50%;
    position: absolute;
    top: 2px;
    left: 2px;
    transition: all 0.3s ease;
  }

  .setting-toggle.active .toggle-slider {
    transform: translateX(26px);
  }

  .setting-toggle:hover {
    background: rgba(255,255,255,0.2);
  }

  .setting-toggle.active:hover {
    background: #005a9e;
  }

  .setting-toggle:hover .toggle-slider {
    transform: scale(1.1);
  }

  .setting-toggle.active:hover .toggle-slider {
    transform: translateX(26px) scale(1.1);
  }

  .path-display {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1rem;
    background: rgba(255,255,255,0.03);
    border-radius: 8px;
    border: 1px solid rgba(255,255,255,0.05);
  }

  .path-input-group {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .path-input {
    width: 200px;
    padding: 0.5rem;
    border: 1px solid rgba(255,255,255,0.1);
    border-radius: 8px;
    background: rgba(255,255,255,0.03);
    color: #fff;
  }

  .path-actions {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .path-button {
    padding: 0.5rem 1rem;
    border: 1px solid rgba(255,255,255,0.1);
    border-radius: 8px;
    background: rgba(255,255,255,0.03);
    color: #fff;
    cursor: pointer;
    transition: all 0.3s ease;
  }

  .path-button:hover {
    background: rgba(255,255,255,0.1);
  }

  .path-value {
    color: #fff;
    font-size: 0.95rem;
    font-weight: 500;
  }

  .path-edit-button {
    padding: 0.5rem 1rem;
    border: 1px solid rgba(255,255,255,0.1);
    border-radius: 8px;
    background: rgba(255,255,255,0.03);
    color: #fff;
    cursor: pointer;
    transition: all 0.3s ease;
  }

  .path-edit-button:hover {
    background: rgba(255,255,255,0.1);
  }

  @media (max-width: 768px) {
    .content {
      padding: 1rem;
    }
    
    .back-button {
      top: 1rem;
      left: 1rem;
    }
    
    .placeholder-card {
      padding: 2rem;
    }
  }
</style> 