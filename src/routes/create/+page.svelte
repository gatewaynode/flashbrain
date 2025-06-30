<script>
  import { goto } from '$app/navigation';
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';

  let isLoaded = $state(false);
  let showCreateModal = $state(false);
  let showEditFilePicker = $state(false);
  let showManualPathModal = $state(false);
  let lessonName = $state('');
  let selectedFilePath = $state('');
  let manualFilePath = $state('');

  function goBack() {
    goto('/');
  }

  function handleCreate() {
    showCreateModal = true;
  }

  function handleEdit() {
    showEditFilePicker = true;
  }

  function handleImport() {
    // Use the new Rust file dialog for import
    handleImportFile();
  }

  async function handleCreateSubmit() {
    if (!lessonName.trim()) {
      alert('Please enter a lesson name');
      return;
    }

    try {
      console.log('Creating new lesson:', lessonName);
      // Navigate to editor with new lesson name
      goto(`/editor?mode=create&name=${encodeURIComponent(lessonName.trim())}`);
    } catch (error) {
      console.error('Error creating lesson:', error);
      alert('Failed to create lesson. Please try again.');
    }
  }

  async function handleFileSelect() {
    try {
      console.log('=== Starting file selection process with Rust RFD ===');
      
      // Show loading state
      const selectButton = document.querySelector('.modal-button.primary');
      if (selectButton) {
        selectButton.textContent = 'Selecting...';
        if ('disabled' in selectButton) {
          selectButton.disabled = true;
        }
      }
      
      console.log('Invoking Rust file dialog with options:', {
        title: 'Select Training JSON File',
        filters: [['JSON Files', ['json']]]
      });
      
      // Use the Rust RFD file dialog
      const result = await invoke('open_file_dialog', {
        title: 'Select Training JSON File',
        filters: [['JSON Files', ['json']]]
      });
      
      console.log('Rust file dialog result:', result);
      
      if (result.success && result.file_path) {
        selectedFilePath = result.file_path;
        console.log('✅ File selected successfully:', selectedFilePath);
        goto(`/editor?mode=edit&file=${encodeURIComponent(selectedFilePath)}`);
      } else {
        console.log('File dialog cancelled or failed:', result.error);
        if (result.error && result.error !== 'Dialog was cancelled') {
          alert(`File dialog error: ${result.error}`);
        } else {
          console.log('User cancelled file selection');
        }
      }
    } catch (error) {
      console.error('❌ Error in file selection:', error);
      console.error('Error details:', String(error));
      
      // Show detailed error to user
      let errorMessage = 'Failed to open file dialog. ';
      if (error && typeof error === 'object') {
        errorMessage += String(error);
      } else {
        errorMessage += 'Unknown error occurred.';
      }
      
      alert(errorMessage + '\n\nFalling back to manual path input.');
      showManualPathModal = true;
    } finally {
      // Reset button state
      const selectButton = document.querySelector('.modal-button.primary');
      if (selectButton) {
        selectButton.textContent = 'Select File';
        if ('disabled' in selectButton) {
          selectButton.disabled = false;
        }
      }
    }
  }

  async function handleImportFile() {
    try {
      console.log('=== Starting import file selection process with Rust RFD ===');
      
      // Show loading state
      const importButton = document.querySelector('.modal-button.primary');
      if (importButton) {
        importButton.textContent = 'Selecting...';
        if ('disabled' in importButton) {
          importButton.disabled = true;
        }
      }
      
      console.log('Invoking Rust file dialog for import with options:', {
        title: 'Import Training JSON File',
        filters: [['JSON Files', ['json']]]
      });
      
      // Use the Rust RFD file dialog for import
      const result = await invoke('open_file_dialog', {
        title: 'Import Training JSON File',
        filters: [['JSON Files', ['json']]]
      });
      
      console.log('Rust import file dialog result:', result);
      
      if (result.success && result.file_path) {
        selectedFilePath = result.file_path;
        console.log('✅ Import file selected successfully:', selectedFilePath);
        goto(`/editor?mode=import&file=${encodeURIComponent(selectedFilePath)}`);
      } else {
        console.log('Import file dialog cancelled or failed:', result.error);
        if (result.error && result.error !== 'Dialog was cancelled') {
          alert(`Import file dialog error: ${result.error}`);
        } else {
          console.log('User cancelled import file selection');
        }
      }
    } catch (error) {
      console.error('❌ Error in import file selection:', error);
      console.error('Error details:', String(error));
      
      // Show detailed error to user
      let errorMessage = 'Failed to open import file dialog. ';
      if (error && typeof error === 'object') {
        errorMessage += String(error);
      } else {
        errorMessage += 'Unknown error occurred.';
      }
      
      alert(errorMessage);
    } finally {
      // Reset button state
      const importButton = document.querySelector('.modal-button.primary');
      if (importButton) {
        importButton.textContent = 'Select File';
        if ('disabled' in importButton) {
          importButton.disabled = false;
        }
      }
    }
  }

  function handleManualPathSubmit() {
    if (!manualFilePath.trim()) {
      alert('Please enter a file path');
      return;
    }
    
    console.log('Using manual file path:', manualFilePath);
    selectedFilePath = manualFilePath.trim();
    showManualPathModal = false;
    manualFilePath = '';
    goto(`/editor?mode=edit&file=${encodeURIComponent(selectedFilePath)}`);
  }

  function closeManualPathModal() {
    showManualPathModal = false;
    manualFilePath = '';
  }

  function closeCreateModal() {
    showCreateModal = false;
    lessonName = '';
  }

  function closeEditFilePicker() {
    showEditFilePicker = false;
    selectedFilePath = '';
  }

  onMount(() => {
    setTimeout(() => {
      isLoaded = true;
    }, 100);
  });
</script>

<svelte:head>
  <title>Create Lessons - FlashBrain</title>
</svelte:head>

<main class="page" class:loaded={isLoaded}>
  <div class="background-gradient"></div>
  
  <div class="content">
    <header class="header">
      <button class="back-button" onclick={goBack}>
        ← Back
      </button>
      <h1 class="title">Create</h1>
      <p class="subtitle">Build your own lessons</p>
    </header>

    <div class="placeholder-content">
      <div class="placeholder-card">
        <h2>Lesson Builder</h2>
        <p>Create custom learning experiences by combining images and text. Design lessons tailored to your specific needs.</p>
        <div class="placeholder-features">
          <div class="feature">🖼️ Upload or select images</div>
          <div class="feature">✍️ Add custom text content</div>
          <div class="feature">⏱️ Set timing for word highlighting</div>
        </div>
        <div class="create-options">
          <button class="create-button" onclick={handleCreate}>
            <span class="button-icon">📝</span>
            <span class="button-text">Create New Lesson</span>
          </button>
          <button class="create-button secondary" onclick={handleEdit}>
            <span class="button-icon">✏️</span>
            <span class="button-text">Edit Existing Lesson</span>
          </button>
          <button class="create-button secondary" onclick={handleImport}>
            <span class="button-icon">📁</span>
            <span class="button-text">Import from File</span>
          </button>
        </div>
      </div>
    </div>
  </div>

  <!-- Create Lesson Modal -->
  {#if showCreateModal}
    <div class="modal-overlay" onclick={closeCreateModal}>
      <div class="modal-content" onclick={(e) => e.stopPropagation()}>
        <h3>Create New Lesson</h3>
        <p>Enter a name for your new lesson. This will be used as the directory name.</p>
        <div class="modal-form">
          <input 
            type="text" 
            placeholder="Lesson name (e.g., 'My First Lesson')"
            bind:value={lessonName}
            class="modal-input"
            onkeydown={(e) => e.key === 'Enter' && handleCreateSubmit()}
          />
          <div class="modal-buttons">
            <button class="modal-button secondary" onclick={closeCreateModal}>Cancel</button>
            <button class="modal-button primary" onclick={handleCreateSubmit}>Create</button>
          </div>
        </div>
      </div>
    </div>
  {/if}

  <!-- Edit File Picker Modal -->
  {#if showEditFilePicker}
    <div class="modal-overlay" onclick={closeEditFilePicker}>
      <div class="modal-content" onclick={(e) => e.stopPropagation()}>
        <h3>Edit Existing Lesson</h3>
        <p>Select a training JSON file to edit.</p>
        <div class="modal-form">
          <div class="modal-buttons">
            <button class="modal-button secondary" onclick={closeEditFilePicker}>Cancel</button>
            <button class="modal-button primary" onclick={handleFileSelect}>Select File</button>
            <button class="modal-button secondary" onclick={() => { console.log('Dialog plugin test:', typeof open); alert('Dialog plugin: ' + typeof open); }}>Test Dialog</button>
          </div>
        </div>
      </div>
    </div>
  {/if}

  <!-- Manual Path Input Modal -->
  {#if showManualPathModal}
    <div class="modal-overlay" onclick={closeManualPathModal}>
      <div class="modal-content" onclick={(e) => e.stopPropagation()}>
        <h3>Enter File Path</h3>
        <p>Please enter the full file path to the training JSON file.</p>
        <div class="modal-form">
          <input 
            type="text" 
            placeholder="File path"
            bind:value={manualFilePath}
            class="modal-input"
            onkeydown={(e) => e.key === 'Enter' && handleManualPathSubmit()}
          />
          <div class="modal-buttons">
            <button class="modal-button secondary" onclick={closeManualPathModal}>Cancel</button>
            <button class="modal-button primary" onclick={handleManualPathSubmit}>Submit</button>
          </div>
        </div>
      </div>
    </div>
  {/if}
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

  .create-options {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    margin-top: 2rem;
  }

  .create-button {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 0.75rem;
    background: linear-gradient(135deg, #fff 0%, #a0a0a0 100%);
    color: #000;
    border: none;
    padding: 1rem 2rem;
    border-radius: 8px;
    font-size: 1rem;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.3s ease;
  }

  .create-button.secondary {
    background: transparent;
    color: #fff;
    border: 1px solid rgba(255,255,255,0.2);
  }

  .create-button:hover {
    transform: translateY(-2px);
    box-shadow: 0 8px 25px rgba(255,255,255,0.2);
  }

  .create-button.secondary:hover {
    background: rgba(255,255,255,0.1);
    border-color: rgba(255,255,255,0.3);
  }

  .button-icon {
    font-size: 1.2rem;
  }

  .button-text {
    font-weight: 500;
  }

  /* Modal Styles */
  .modal-overlay {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(0, 0, 0, 0.8);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
    backdrop-filter: blur(5px);
  }

  .modal-content {
    background: linear-gradient(135deg, rgba(255,255,255,0.1) 0%, rgba(255,255,255,0.05) 100%);
    border: 1px solid rgba(255,255,255,0.2);
    border-radius: 16px;
    padding: 2rem;
    max-width: 500px;
    width: 90%;
    text-align: center;
    backdrop-filter: blur(10px);
  }

  .modal-content h3 {
    font-size: 1.5rem;
    font-weight: 700;
    margin-bottom: 1rem;
    color: #fff;
  }

  .modal-content p {
    color: #aaa;
    line-height: 1.6;
    margin-bottom: 1.5rem;
  }

  .modal-form {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .modal-input {
    background: rgba(255,255,255,0.1);
    border: 1px solid rgba(255,255,255,0.2);
    border-radius: 8px;
    padding: 0.75rem 1rem;
    color: #fff;
    font-size: 1rem;
    outline: none;
    transition: all 0.3s ease;
  }

  .modal-input::placeholder {
    color: #888;
  }

  .modal-input:focus {
    border-color: rgba(255,255,255,0.4);
    background: rgba(255,255,255,0.15);
  }

  .modal-buttons {
    display: flex;
    gap: 1rem;
    justify-content: center;
  }

  .modal-button {
    padding: 0.75rem 1.5rem;
    border-radius: 8px;
    font-size: 0.9rem;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.3s ease;
    border: none;
  }

  .modal-button.primary {
    background: linear-gradient(135deg, #fff 0%, #a0a0a0 100%);
    color: #000;
  }

  .modal-button.secondary {
    background: transparent;
    color: #fff;
    border: 1px solid rgba(255,255,255,0.2);
  }

  .modal-button:hover {
    transform: translateY(-2px);
    box-shadow: 0 4px 15px rgba(255,255,255,0.2);
  }

  .modal-button.secondary:hover {
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
    
    .placeholder-card {
      padding: 2rem;
    }

    .modal-content {
      padding: 1.5rem;
      margin: 1rem;
    }

    .modal-buttons {
      flex-direction: column;
    }
  }
</style> 