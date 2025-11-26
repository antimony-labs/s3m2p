const { writeFileSync } = require('fs');
const { createCanvas } = require('canvas');

/**
 * Build-time still generation script.
 * Generates static PNG images for fallback and OG image.
 */

function generateStill(width, height) {
  const canvas = createCanvas(width, height);
  const ctx = canvas.getContext('2d');
  
  // Create a gradient background matching the cosmic indigo theme
  const gradient = ctx.createRadialGradient(width / 2, height / 2, 0, width / 2, height / 2, Math.max(width, height) / 2);
  gradient.addColorStop(0, '#0B0F1A');
  gradient.addColorStop(0.5, '#1a1f2e');
  gradient.addColorStop(1, '#0B0F1A');
  ctx.fillStyle = gradient;
  ctx.fillRect(0, 0, width, height);
  
  // Add subtle starfield effect
  ctx.fillStyle = '#99E6FF';
  for (let i = 0; i < 200; i++) {
    const x = Math.random() * width;
    const y = Math.random() * height;
    const size = Math.random() * 2;
    ctx.beginPath();
    ctx.arc(x, y, size, 0, Math.PI * 2);
    ctx.fill();
  }
  
  // Add text overlay
  ctx.fillStyle = '#99E6FF';
  ctx.font = 'bold 64px sans-serif';
  ctx.textAlign = 'center';
  ctx.textBaseline = 'middle';
  ctx.fillText('too.foo', width / 2, height / 2 - 30);
  ctx.font = '32px sans-serif';
  ctx.fillStyle = 'rgba(153, 230, 255, 0.8)';
  ctx.fillText('Solar Memory Online', width / 2, height / 2 + 30);
  
  return canvas.toBuffer('image/png');
}

function main() {
  try {
    // Generate fallback still (1600x900)
    console.log('Generating heliosphere-still.png...');
    const stillBuffer = generateStill(1600, 900);
    writeFileSync('./public/img/heliosphere-still.png', stillBuffer);
    console.log('✓ Generated heliosphere-still.png');
    
    // Generate OG image (1200x630)
    console.log('Generating og.png...');
    const ogBuffer = generateStill(1200, 630);
    writeFileSync('./public/img/og.png', ogBuffer);
    console.log('✓ Generated og.png');
    
    console.log('\n✓ All still images generated successfully!');
    console.log('\nNote: For actual WebGL rendering, consider using puppeteer');
    console.log('to capture frames from a headless browser instance.');
  } catch (error) {
    console.error('Error generating still images:', error);
    process.exit(1);
  }
}

if (require.main === module) {
  main();
}

module.exports = { generateStill };




