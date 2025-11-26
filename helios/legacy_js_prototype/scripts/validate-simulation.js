#!/usr/bin/env node

/**
 * Run validation tests for the heliosphere simulation
 * This ensures scientific accuracy of the implementation
 */

const { runAndLogValidation } = require('../.next/server/app/lib/tests/ValidationTests');

async function main() {
  console.log('🔬 Scientific Validation Test Suite');
  console.log('=====================================');
  
  try {
    const allPassed = await runAndLogValidation();
    
    if (allPassed) {
      console.log('\n✅ All validation tests passed!');
      console.log('The simulation meets scientific accuracy requirements.');
      process.exit(0);
    } else {
      console.log('\n❌ Some validation tests failed.');
      console.log('Please review the failures and adjust the implementation.');
      process.exit(1);
    }
  } catch (error) {
    console.error('\n❌ Error running validation tests:', error);
    process.exit(1);
  }
}

// Run if called directly
if (require.main === module) {
  main();
}
