import { writeFileSync } from 'fs';
import { createCanvas } from 'canvas';

/**
 * Build-time still generation script.
 * 
 * This script generates static PNG images for:
 * - Fallback image (/public/img/heliosphere-still.png)
 * - OpenGraph image (/public/img/og.png)
 * 
 * Note: This uses Node.js canvas as a fallback. For headless GL rendering,
 * you may need to use puppeteer or a similar tool to capture from a browser.
 */

async function generateStill(width: number, height: number): Promise<Buffer> {
  // Since we can't easily use WebGL in Node.js without headless-gl or puppeteer,
  // we'll create a placeholder script that can be run with puppeteer in CI/CD
  // For now, this is a placeholder that documents the approach
  
  const canvas = createCanvas(width, height);
  const ctx = canvas.getContext('2d');
  
  // Create a simple gradient background as placeholder
  const gradient = ctx.createLinearGradient(0, 0, width, height);
  gradient.addColorStop(0, '#0B0F1A');
  gradient.addColorStop(1, '#1a1f2e');
  ctx.fillStyle = gradient;
  ctx.fillRect(0, 0, width, height);
  
  // Add text overlay
  ctx.fillStyle = '#99E6FF';
  ctx.font = 'bold 48px sans-serif';
  ctx.textAlign = 'center';
  ctx.fillText('too.foo', width / 2, height / 2 - 20);
  ctx.font = '24px sans-serif';
  ctx.fillText('Solar Memory Online', width / 2, height / 2 + 20);
  
  return canvas.toBuffer('image/png');
}

async function main() {
  try {
    // Generate fallback still (1600x900)
    console.log('Generating heliosphere-still.png...');
    const stillBuffer = await generateStill(1600, 900);
    writeFileSync('./public/img/heliosphere-still.png', stillBuffer);
    console.log('✓ Generated heliosphere-still.png');
    
    // Generate OG image (1200x630)
    console.log('Generating og.png...');
    const ogBuffer = await generateStill(1200, 630);
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

export { generateStill };

