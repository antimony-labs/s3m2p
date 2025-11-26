'use client';

import { useEffect, useMemo, useRef, useState } from 'react';
import Link from 'next/link';
import { NotFoundTelemetry, generateNotFoundTelemetry } from '../lib/notFoundTelemetry';

type Props = {
  initialTelemetry: NotFoundTelemetry;
};

type Metric = {
  label: string;
  value: string;
  detail: string;
};

export default function NotFoundExperience({ initialTelemetry }: Props) {
  const [telemetry, setTelemetry] = useState<NotFoundTelemetry>(initialTelemetry);
  const [copied, setCopied] = useState(false);
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const [cursor, setCursor] = useState({ x: 0.5, y: 0.4 });

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;
    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    let animationFrame = 0;
    const stars = Array.from({ length: 360 }, () => ({
      x: (Math.random() - 0.5) * 2,
      y: (Math.random() - 0.5) * 2,
      z: Math.random(),
      velocity: 0.00025 + Math.random() * 0.0006
    }));

    const resize = () => {
      const { clientWidth, clientHeight } = canvas;
      canvas.width = clientWidth;
      canvas.height = clientHeight;
    };

    resize();
    let resizeObserver: ResizeObserver | null = null;
    if (typeof ResizeObserver !== 'undefined') {
      resizeObserver = new ResizeObserver(resize);
      resizeObserver.observe(canvas);
    } else {
      window.addEventListener('resize', resize);
    }

    const render = () => {
      const { width, height } = canvas;
      ctx.fillStyle = 'rgba(2, 4, 12, 0.7)';
      ctx.fillRect(0, 0, width, height);

      ctx.save();
      ctx.translate(width / 2, height / 2);
      stars.forEach((star) => {
        star.z -= star.velocity;
        if (star.z <= 0) star.z = 1;

        const k = 0.6 / star.z;
        const x = star.x * width * k;
        const y = star.y * height * k;

        const size = Math.max((1 - star.z) * 2.4, 0.4);
        const alpha = 0.25 + 0.75 * (1 - star.z);
        ctx.fillStyle = `rgba(153, 230, 255, ${alpha})`;
        ctx.beginPath();
        ctx.arc(x, y, size, 0, Math.PI * 2);
        ctx.fill();
      });
      ctx.restore();

      animationFrame = window.requestAnimationFrame(render);
    };

    render();

    return () => {
      if (resizeObserver) {
        resizeObserver.disconnect();
      } else {
        window.removeEventListener('resize', resize);
      }
      window.cancelAnimationFrame(animationFrame);
    };
  }, []);

  useEffect(() => {
    const handlePointer = (event: PointerEvent) => {
      window.requestAnimationFrame(() => {
        setCursor({
          x: event.clientX / window.innerWidth,
          y: event.clientY / window.innerHeight
        });
      });
    };
    window.addEventListener('pointermove', handlePointer);
    return () => window.removeEventListener('pointermove', handlePointer);
  }, []);

  useEffect(() => {
    const id = window.setInterval(() => {
      setTelemetry(generateNotFoundTelemetry());
    }, 45000);
    return () => window.clearInterval(id);
  }, []);

  const metrics = useMemo<Metric[]>(() => [
    {
      label: 'Signal path delay',
      value: `${telemetry.propagationDelayMin.toFixed(1)} min`,
      detail: 'one-way @ c'
    },
    {
      label: 'Voyager 1 range',
      value: `${telemetry.voyager1DistanceAU.toFixed(1)} AU`,
      detail: 'heliocentric'
    },
    {
      label: 'Voyager 2 range',
      value: `${telemetry.voyager2DistanceAU.toFixed(1)} AU`,
      detail: 'heliocentric'
    },
    {
      label: 'Signal integrity',
      value: `${(telemetry.signalIntegrity * 100).toFixed(1)}%`,
      detail: 'inverse-square attenuation'
    },
    {
      label: 'Heliopause drift',
      value: `${(telemetry.heliopauseDriftAU * 1000).toFixed(2)} mAU`,
      detail: 'beyond modeled boundary'
    },
    {
      label: 'Barycentric baseline',
      value: `${telemetry.barycentricBaselineAU.toFixed(1)} AU`,
      detail: 'Voyager pair centroid'
    }
  ], [telemetry]);

  const propagationStops = useMemo(() => ([
    { label: 'Earth', value: 1 },
    { label: 'Termination shock', value: 94.0 },
    { label: 'Heliopause', value: 121.6 },
    { label: 'Voyager 1', value: telemetry.voyager1DistanceAU }
  ]), [telemetry.voyager1DistanceAU]);

  const maxRadius = telemetry.voyager1DistanceAU + 15;

  const handleCopy = async () => {
    try {
      const payload = JSON.stringify(telemetry, null, 2);
      await navigator.clipboard.writeText(payload);
      setCopied(true);
      window.setTimeout(() => setCopied(false), 1200);
    } catch {
      setCopied(false);
    }
  };

  return (
    <div className="relative isolate min-h-screen overflow-hidden bg-black text-white">
      <canvas ref={canvasRef} className="absolute inset-0 h-full w-full opacity-80" aria-hidden="true" />
      <div
        className="pointer-events-none absolute inset-0 opacity-60"
        style={{
          background: `
            radial-gradient(circle at ${cursor.x * 100}% ${cursor.y * 100}%, rgba(52, 133, 255, 0.35), transparent 55%),
            radial-gradient(circle at 20% 20%, rgba(0, 255, 171, 0.15), transparent 50%),
            radial-gradient(circle at 80% 30%, rgba(255, 184, 108, 0.12), transparent 45%),
            #010309
          `
        }}
      />
      <div className="absolute inset-0 bg-[radial-gradient(circle_at_top,_rgba(255,255,255,0.08),_transparent_55%)]" aria-hidden="true" />

      <div className="relative z-10 mx-auto flex min-h-screen w-full max-w-6xl flex-col gap-10 px-4 py-10 sm:px-8 lg:py-16">
        <div className="space-y-4">
          <p className="text-[0.65rem] uppercase tracking-[0.45em] text-emerald-200/80">
            anomaly class · lost telemetry / 404
          </p>
          <h1 className="text-4xl sm:text-5xl lg:text-6xl">
            Signal not located.
          </h1>
          <p className="max-w-3xl text-base leading-relaxed text-white/80">
            We rerouted the heliosphere solvers, re-spun the magnetohydrodynamic model, and traced every photon.
            This coordinate still resolves to vacuum. You have drifted outside the published atlas—an edge case
            worthy of Voyager-grade attention.
          </p>
        </div>

        <section className="grid gap-3 rounded-3xl border border-white/10 bg-black/60 p-4 backdrop-blur md:grid-cols-3">
          {metrics.map((metric) => (
            <div key={metric.label} className="rounded-2xl border border-white/10 bg-white/5 p-4">
              <p className="text-[0.55rem] uppercase tracking-[0.35em] text-white/50">{metric.label}</p>
              <p className="mt-2 text-2xl font-light">{metric.value}</p>
              <p className="text-xs text-white/60">{metric.detail}</p>
            </div>
          ))}
        </section>

        <section className="grid gap-6 lg:grid-cols-[1.8fr,1fr]">
          <div className="rounded-3xl border border-white/10 bg-black/60 p-6 backdrop-blur">
            <div className="flex items-center justify-between gap-3">
              <div>
                <p className="text-[0.55rem] uppercase tracking-[0.35em] text-white/50">
                  Propagation model
                </p>
                <p className="text-lg text-white/80">
                  Parametric heliopause cross-section
                </p>
              </div>
              <span className="rounded-full border border-white/15 px-3 py-1 text-xs font-mono text-white/70">
                {telemetry.isoTimestamp.replace('T', ' · ').replace('Z', ' UTC')}
              </span>
            </div>
            <div className="mt-6 grid gap-6 md:grid-cols-2">
              <svg
                viewBox="0 0 320 320"
                role="img"
                aria-label="Heliosphere propagation model"
                className="w-full drop-shadow-[0_0_30px_rgba(15,94,255,0.35)]"
              >
                <defs>
                  <radialGradient id="aurora" cx="50%" cy="50%" r="50%">
                    <stop offset="0%" stopColor="rgba(144,205,255,0.4)" />
                    <stop offset="100%" stopColor="rgba(9,12,24,0)" />
                  </radialGradient>
                </defs>
                <rect width="320" height="320" fill="url(#aurora)" />
                {propagationStops.map((stop, index) => {
                  const radius = (stop.value / maxRadius) * 140;
                  return (
                    <circle
                      key={stop.label}
                      cx="160"
                      cy="160"
                      r={Math.max(radius, 6)}
                      fill="none"
                      stroke={`rgba(255,255,255,${0.65 - index * 0.12})`}
                      strokeDasharray={index === propagationStops.length - 1 ? '3 6' : undefined}
                      strokeWidth={index === propagationStops.length - 1 ? 2.4 : 1}
                    />
                  );
                })}
                {propagationStops.map((stop) => {
                  const radius = (stop.value / maxRadius) * 140;
                  return (
                    <text
                      key={`${stop.label}-label`}
                      x="160"
                      y={160 - Math.max(radius, 6) - 6}
                      textAnchor="middle"
                      fontSize="10"
                      fill="rgba(255,255,255,0.75)"
                    >
                      {stop.label}
                    </text>
                  );
                })}
              </svg>
              <div className="space-y-4 text-sm text-white/80">
                <p>
                  We model the heliopause as an anisotropic boundary condition solved via fast marching on a
                  three-dimensional grid. The anomaly you triggered sits beyond the modeled confidence interval,
                  so we fall back to trajectory-aware guidance.
                </p>
                <div className="rounded-2xl border border-white/10 bg-white/5 p-4">
                  <p className="text-[0.55rem] uppercase tracking-[0.35em] text-white/50 mb-2">
                    Recovery heuristics
                  </p>
                  <ul className="space-y-2 text-xs text-white/70">
                    <li>• Reroute through barycentric spline minimizing Δv.</li>
                    <li>• Recalibrate pointing using IBEX neutral atom flux.</li>
                    <li>• Collapse search radius until signal-to-noise ≥ 12 dB.</li>
                  </ul>
                </div>
              </div>
            </div>
          </div>
          <div className="rounded-3xl border border-white/10 bg-black/60 p-6 backdrop-blur">
            <p className="text-[0.55rem] uppercase tracking-[0.35em] text-white/50">
              Navigation options
            </p>
            <div className="mt-4 space-y-4">
              <Link
                href="/"
                className="flex items-center justify-between rounded-2xl border border-white/20 bg-white/10 px-4 py-3 text-sm font-medium text-white hover:border-white/40 hover:bg-white/15 transition"
              >
                <span>Return to Solar Memory Console</span>
                <span aria-hidden="true">↙</span>
              </Link>
              <button
                type="button"
                onClick={() => window.history.back()}
                className="w-full rounded-2xl border border-white/10 bg-transparent px-4 py-3 text-sm font-medium text-white/80 hover:border-white/40 transition"
              >
                Retrace previous trajectory
              </button>
              <button
                type="button"
                onClick={handleCopy}
                className="w-full rounded-2xl border border-emerald-300/40 bg-emerald-400/10 px-4 py-3 text-sm font-medium text-emerald-200 hover:border-emerald-300/70 hover:bg-emerald-400/20 transition"
              >
                {copied ? 'Telemetry copied' : 'Copy telemetry packet'}
              </button>
            </div>
            <div className="mt-6 rounded-2xl border border-white/10 bg-white/5 p-4 text-xs text-white/70">
              <p className="font-mono">
                Σ integrity ≈ {(telemetry.signalIntegrity * 100).toFixed(2)}% · Δheliopause{' '}
                {telemetry.heliopauseDriftAU >= 0 ? '+' : '−'}
                {Math.abs(telemetry.heliopauseDriftAU * 1000).toFixed(2)} mAU
              </p>
              <p className="mt-2">
                If you believe this anomaly should exist, open an issue tagged <span className="text-white">#nav-404</span>
                with your mission parameters.
              </p>
            </div>
          </div>
        </section>
      </div>
    </div>
  );
}
