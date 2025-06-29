import { writable } from 'svelte/store';

// Global store for current training data
export const currentTrainingData = writable(null);

// Store for current training session state
export const trainingSession = writable({
  isActive: false,
  currentItemIndex: 0,
  isImageVisible: true,
  isTextVisible: false,
  isTransitioning: false
});

// Global settings store
export const globalSettings = writable({
  debugMode: false,
  // Add other settings here as needed
  // autoPlay: true,
  // transitionSpeed: 1000,
  // textDisplayTime: 3000,
});

// Reset training session
export function resetTrainingSession() {
  console.log('Resetting training session');
  trainingSession.set({
    isActive: false,
    currentItemIndex: 0,
    isImageVisible: true,
    isTextVisible: false,
    isTransitioning: false
  });
  currentTrainingData.set(null);
}

// Start training session
export function startTrainingSession(trainingData) {
  console.log('Starting training session with data:', trainingData);
  console.log('Number of items:', trainingData?.items?.length || 0);
  
  currentTrainingData.set(trainingData);
  trainingSession.set({
    isActive: true,
    currentItemIndex: 0,
    isImageVisible: true,
    isTextVisible: false,
    isTransitioning: false
  });
}

// Toggle debug mode
export function toggleDebugMode() {
  globalSettings.update(settings => ({
    ...settings,
    debugMode: !settings.debugMode
  }));
} 