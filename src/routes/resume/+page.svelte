<script>
  import { goto } from '$app/navigation';
  import { onMount } from 'svelte';

  let isLoaded = $state(false);

  function goBack() {
    goto('/');
  }

  onMount(() => {
    setTimeout(() => {
      isLoaded = true;
    }, 100);
  });
</script>

<svelte:head>
  <title>Resume Session - FlashBrain</title>
</svelte:head>

<main class="page" class:loaded={isLoaded}>
  <div class="background-gradient"></div>
  
  <div class="content">
    <header class="header">
      <button class="back-button" onclick={goBack}>
        ← Back
      </button>
      <h1 class="title">Resume</h1>
      <p class="subtitle">Continue your learning journey</p>
    </header>

    <div class="placeholder-content">
      <div class="placeholder-card">
        <h2>No Active Sessions</h2>
        <p>You don't have any active learning sessions to resume. Start a new session to begin your learning journey.</p>
        <div class="placeholder-features">
          <div class="feature">📖 Continue from last lesson</div>
          <div class="feature">📊 View progress history</div>
          <div class="feature">🎯 Pick up where you left off</div>
        </div>
        <button class="cta-button" onclick={() => goto('/new')}>
          Start New Session
        </button>
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

  .cta-button {
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

  .cta-button:hover {
    transform: translateY(-2px);
    box-shadow: 0 8px 25px rgba(255,255,255,0.2);
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