'use client';

import { useEffect, useRef, useState } from 'react';

export default function TestSceneClient() {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const [status, setStatus] = useState('Initializing...');
  const [isMounted, setIsMounted] = useState(false);

  useEffect(() => {
    setIsMounted(true);
  }, []);

  useEffect(() => {
    if (!isMounted || !canvasRef.current) return;

    const initScene = async () => {
      try {
        setStatus('Loading Three.js...');
        const THREE = await import('three');
        
        setStatus('Creating renderer...');
        const renderer = new THREE.WebGLRenderer({ 
          canvas: canvasRef.current!,
          antialias: true 
        });
        renderer.setSize(800, 600);
        
        setStatus('Creating scene...');
        const scene = new THREE.Scene();
        scene.background = new THREE.Color(0x000033);
        
        const camera = new THREE.PerspectiveCamera(75, 800 / 600, 0.1, 1000);
        camera.position.z = 5;
        
        setStatus('Creating geometry...');
        const geometry = new THREE.BoxGeometry(1, 1, 1);
        const material = new THREE.MeshBasicMaterial({ color: 0x00ff00, wireframe: true });
        const cube = new THREE.Mesh(geometry, material);
        scene.add(cube);
        
        setStatus('✅ Scene ready - animating...');
        
        let animationId: number;
        const animate = () => {
          animationId = requestAnimationFrame(animate);
          cube.rotation.x += 0.01;
          cube.rotation.y += 0.01;
          renderer.render(scene, camera);
        };
        animate();
        
        return () => {
          cancelAnimationFrame(animationId);
          renderer.dispose();
        };
      } catch (error) {
        setStatus(`❌ Error: ${error instanceof Error ? error.message : String(error)}`);
        console.error('[TestScene] Error:', error);
      }
    };

    initScene();
  }, [isMounted]);

  if (!isMounted) {
    return <div className="text-white p-4">Loading...</div>;
  }

  return (
    <div className="relative w-full h-screen">
      <canvas ref={canvasRef} className="absolute inset-0" suppressHydrationWarning />
      <div className="absolute top-4 left-4 bg-black bg-opacity-75 text-white p-4 rounded">
        <h2 className="font-bold mb-2">Three.js Test</h2>
        <p className="text-sm">{status}</p>
        <p className="text-xs text-gray-400 mt-2">
          You should see a rotating green wireframe cube
        </p>
      </div>
    </div>
  );
}

