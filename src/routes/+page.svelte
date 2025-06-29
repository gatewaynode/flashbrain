<script>
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';

  // Navigation items
  const menuItems = [
    { id: 'new', label: 'New', description: 'Start a fresh learning session' },
    { id: 'resume', label: 'Resume', description: 'Continue where you left off' },
    { id: 'find', label: 'Find', description: 'Discover existing lessons' },
    { id: 'create', label: 'Create', description: 'Build your own lessons' },
    { id: 'settings', label: 'Settings', description: 'Customize your experience' }
  ];

  let currentIndex = $state(0);
  let isLoaded = $state(false);

  // Handle keyboard navigation
  function handleKeydown(event) {
    switch (event.key) {
      case 'ArrowDown':
        event.preventDefault();
        currentIndex = (currentIndex + 1) % menuItems.length;
        break;
      case 'ArrowUp':
        event.preventDefault();
        currentIndex = currentIndex === 0 ? menuItems.length - 1 : currentIndex - 1;
        break;
      case 'Enter':
        event.preventDefault();
        navigateToItem(menuItems[currentIndex].id);
        break;
    }
  }

  function navigateToItem(id) {
    goto(`/${id}`);
  }

  onMount(() => {
    // Add keyboard listener
    document.addEventListener('keydown', handleKeydown);
    
    // Trigger entrance animation
    setTimeout(() => {
      isLoaded = true;
    }, 100);

    return () => {
      document.removeEventListener('keydown', handleKeydown);
    };
  });
</script>

<svelte:head>
  <title>FlashBrain - Memory Learning App</title>
</svelte:head>

<main class="landing-page" class:loaded={isLoaded}>
  <!-- Background gradient -->
  <div class="background-gradient"></div>
  
  <!-- Main content -->
  <div class="content">
    <!-- Header -->
    <header class="header">
      <h1 class="title">
        <span class="title-line">Flash</span>
        <span class="title-line">Brain</span>
      </h1>
      <p class="subtitle">Memory learning through visual association</p>
    </header>

    <!-- Navigation menu -->
    <nav class="navigation">
      <ul class="menu-list">
        {#each menuItems as item, index}
          <li class="menu-item" class:active={index === currentIndex}>
            <button 
              class="menu-button"
              on:click={() => navigateToItem(item.id)}
              on:mouseenter={() => currentIndex = index}
            >
              <span class="menu-label">{item.label}</span>
              <span class="menu-description">{item.description}</span>
            </button>
          </li>
        {/each}
      </ul>
    </nav>

    <!-- Footer -->
    <footer class="footer">
      <p class="footer-text">
        Use arrow keys to navigate • Press Enter to select
      </p>
    </footer>
  </div>
</main>

<style>
  /* Reset and base styles */
  * {
    margin: 0;
    padding: 0;
    box-sizing: border-box;
  }

  body {
    font-family: 'Inter', -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
    background: #000;
    color: #fff;
    overflow: hidden;
  }

  /* Landing page container */
  .landing-page {
    min-height: 100vh;
    display: flex;
    align-items: center;
    justify-content: center;
    position: relative;
    opacity: 0;
    transform: translateY(20px);
    transition: all 0.8s cubic-bezier(0.16, 1, 0.3, 1);
  }

  .landing-page.loaded {
    opacity: 1;
    transform: translateY(0);
  }

  /* Background gradient */
  .background-gradient {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: linear-gradient(135deg, #0a0a0a 0%, #1a1a1a 50%, #0a0a0a 100%);
    z-index: -1;
  }

  /* Content container */
  .content {
    text-align: center;
    max-width: 800px;
    padding: 2rem;
    width: 100%;
  }

  /* Header styles */
  .header {
    margin-bottom: 4rem;
    opacity: 0;
    transform: translateY(30px);
    animation: fadeInUp 0.8s cubic-bezier(0.16, 1, 0.3, 1) 0.2s forwards;
  }

  .title {
    font-size: clamp(3rem, 8vw, 6rem);
    font-weight: 900;
    line-height: 0.9;
    margin-bottom: 1rem;
    letter-spacing: -0.02em;
  }

  .title-line {
    display: block;
    background: linear-gradient(135deg, #fff 0%, #a0a0a0 100%);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
  }

  .subtitle {
    font-size: 1.2rem;
    color: #888;
    font-weight: 400;
    letter-spacing: 0.02em;
  }

  /* Navigation styles */
  .navigation {
    margin-bottom: 3rem;
    opacity: 0;
    transform: translateY(30px);
    animation: fadeInUp 0.8s cubic-bezier(0.16, 1, 0.3, 1) 0.4s forwards;
  }

  .menu-list {
    list-style: none;
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .menu-item {
    transition: all 0.3s cubic-bezier(0.16, 1, 0.3, 1);
  }

  .menu-button {
    width: 100%;
    background: transparent;
    border: none;
    padding: 1.5rem 2rem;
    text-align: left;
    cursor: pointer;
    border-radius: 12px;
    transition: all 0.3s cubic-bezier(0.16, 1, 0.3, 1);
    position: relative;
    overflow: hidden;
  }

  .menu-button::before {
    content: '';
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: linear-gradient(135deg, rgba(255,255,255,0.05) 0%, rgba(255,255,255,0.02) 100%);
    border: 1px solid rgba(255,255,255,0.1);
    border-radius: 12px;
    opacity: 0;
    transition: all 0.3s cubic-bezier(0.16, 1, 0.3, 1);
  }

  .menu-button:hover::before,
  .menu-item.active .menu-button::before {
    opacity: 1;
    transform: scale(1.02);
  }

  .menu-label {
    display: block;
    font-size: 1.5rem;
    font-weight: 600;
    color: #fff;
    margin-bottom: 0.25rem;
    transition: color 0.3s ease;
  }

  .menu-description {
    display: block;
    font-size: 0.9rem;
    color: #888;
    font-weight: 400;
    transition: color 0.3s ease;
  }

  .menu-button:hover .menu-label,
  .menu-item.active .menu-label {
    color: #fff;
  }

  .menu-button:hover .menu-description,
  .menu-item.active .menu-description {
    color: #aaa;
  }

  /* Footer styles */
  .footer {
    opacity: 0;
    transform: translateY(30px);
    animation: fadeInUp 0.8s cubic-bezier(0.16, 1, 0.3, 1) 0.6s forwards;
  }

  .footer-text {
    font-size: 0.85rem;
    color: #666;
    font-weight: 400;
  }

  /* Animations */
  @keyframes fadeInUp {
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }

  /* Responsive design */
  @media (max-width: 768px) {
    .content {
      padding: 1rem;
    }
    
    .header {
      margin-bottom: 3rem;
    }
    
    .navigation {
      margin-bottom: 2rem;
    }
    
    .menu-button {
      padding: 1.25rem 1.5rem;
    }
    
    .menu-label {
      font-size: 1.25rem;
    }
  }

  /* Focus styles for accessibility */
  .menu-button:focus {
    outline: 2px solid rgba(255,255,255,0.3);
    outline-offset: 2px;
  }

  /* Smooth scrolling */
  html {
    scroll-behavior: smooth;
  }
</style>
