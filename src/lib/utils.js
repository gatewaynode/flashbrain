/**
 * Counts the number of words in a given text string
 * @param {string} text - The text to count words in
 * @returns {number} - The number of words
 */
export function countWords(text) {
  if (!text || typeof text !== 'string') {
    return 0;
  }
  
  // Remove extra whitespace and split by whitespace
  const words = text.trim().split(/\s+/);
  
  // Filter out empty strings and return count
  return words.filter(word => word.length > 0).length;
}

/**
 * Calculates the display duration for text based on words per second
 * @param {string} text - The text to calculate duration for
 * @param {number} secondsPerWord - Seconds to display per word
 * @param {number} minDuration - Minimum duration in milliseconds (default: 2000)
 * @param {number} maxDuration - Maximum duration in milliseconds (default: 10000)
 * @returns {number} - Duration in milliseconds
 */
export function calculateTextDisplayDuration(text, secondsPerWord, minDuration = 2000, maxDuration = 10000) {
  const wordCount = countWords(text);
  const calculatedDuration = wordCount * secondsPerWord * 1000; // Convert to milliseconds
  
  // Ensure duration is within bounds
  return Math.max(minDuration, Math.min(maxDuration, calculatedDuration));
}

/**
 * Formats word count for display
 * @param {number} wordCount - The number of words
 * @returns {string} - Formatted word count string
 */
export function formatWordCount(wordCount) {
  if (wordCount === 1) {
    return '1 word';
  }
  return `${wordCount} words`;
} 