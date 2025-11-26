/**
 * Simple Three.js test page to verify WebGL works
 */

import { Metadata } from 'next';
import TestSceneClient from './TestSceneClient';

export const metadata: Metadata = {
  title: 'Three.js Test',
};

export default function TestPage() {
  return (
    <main className="min-h-screen bg-black">
      <TestSceneClient />
    </main>
  );
}

