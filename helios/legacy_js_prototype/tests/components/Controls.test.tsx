import React from 'react';
import { render, screen, waitFor, act } from '@testing-library/react';
import { describe, expect, it, vi, beforeEach, afterEach } from 'vitest';
import Controls from '@/app/components/Controls';

const mockHeroRef = {
  current: {
    updateScene: vi.fn()
  }
} as React.RefObject<any>;

describe('Controls', () => {
  let requestAnimationFrameSpy: ReturnType<typeof vi.spyOn>;
  let cancelAnimationFrameSpy: ReturnType<typeof vi.spyOn>;
  let rafCallbacks: FrameRequestCallback[] = [];
  let rafIdCounter = 0;

  beforeEach(() => {
    rafCallbacks = [];
    rafIdCounter = 0;
    
    requestAnimationFrameSpy = vi.spyOn(window, 'requestAnimationFrame').mockImplementation((cb) => {
      rafCallbacks.push(cb);
      return ++rafIdCounter;
    });
    
    cancelAnimationFrameSpy = vi.spyOn(window, 'cancelAnimationFrame').mockImplementation(() => {});
    
    vi.clearAllMocks();
    mockHeroRef.current!.updateScene.mockClear();
  });

  afterEach(() => {
    requestAnimationFrameSpy.mockRestore();
    cancelAnimationFrameSpy.mockRestore();
  });

  it('renders controls correctly', () => {
    render(
      <Controls
        heroRef={mockHeroRef}
        onTimeChange={vi.fn()}
        onDirectionChange={vi.fn()}
        onMotionChange={vi.fn()}
        onPauseChange={vi.fn()}
      />
    );

    expect(screen.getByLabelText(/Time/i)).toBeInTheDocument();
  });

  it('starts animation loop when not paused and motion enabled', async () => {
    const onTimeChange = vi.fn();

    render(
      <Controls
        heroRef={mockHeroRef}
        onTimeChange={onTimeChange}
        onDirectionChange={vi.fn()}
        onMotionChange={vi.fn()}
        onPauseChange={vi.fn()}
      />
    );

    await act(async () => {
      // Process a few animation frames
      for (let i = 0; i < 3; i++) {
        if (rafCallbacks.length > 0) {
          const cb = rafCallbacks[rafCallbacks.length - 1];
          cb(performance.now() + i * 16);
        }
        await new Promise(resolve => setTimeout(resolve, 10));
      }
    });

    expect(requestAnimationFrameSpy).toHaveBeenCalled();
    expect(mockHeroRef.current!.updateScene).toHaveBeenCalled();
  });

  it('stops animation loop when paused', async () => {
    const onPauseChange = vi.fn();
    const { rerender } = render(
      <Controls
        heroRef={mockHeroRef}
        onTimeChange={vi.fn()}
        onDirectionChange={vi.fn()}
        onMotionChange={vi.fn()}
        onPauseChange={onPauseChange}
      />
    );

    await act(async () => {
      await new Promise(resolve => setTimeout(resolve, 50));
    });

    const pauseButton = screen.getByRole('button', { name: /pause/i });
    act(() => {
      pauseButton.click();
    });

    await waitFor(() => {
      expect(cancelAnimationFrameSpy).toHaveBeenCalled();
    });
  });

  it('handles errors in animation loop gracefully', async () => {
    const consoleErrorSpy = vi.spyOn(console, 'error').mockImplementation(() => {});

    render(
      <Controls
        heroRef={mockHeroRef}
        onTimeChange={vi.fn()}
        onDirectionChange={vi.fn()}
        onMotionChange={vi.fn()}
        onPauseChange={vi.fn()}
      />
    );

    // Wait for initial render
    await act(async () => {
      await new Promise(resolve => setTimeout(resolve, 50));
    });

    // Set up error after initial render
    mockHeroRef.current!.updateScene.mockImplementationOnce(() => {
      throw new Error('Animation error');
    });

    await act(async () => {
      // Process animation frames
      if (rafCallbacks.length > 0) {
        const cb = rafCallbacks[rafCallbacks.length - 1];
        cb(performance.now());
      }
      await new Promise(resolve => setTimeout(resolve, 50));
    });

    await waitFor(() => {
      expect(consoleErrorSpy).toHaveBeenCalledWith(expect.stringContaining('Error in animation loop'), expect.any(Error));
    }, { timeout: 2000 });

    // Animation should have stopped
    expect(cancelAnimationFrameSpy).toHaveBeenCalled();

    consoleErrorSpy.mockRestore();
  });

  it('handles errors when updating scene on pause', async () => {
    const consoleErrorSpy = vi.spyOn(console, 'error').mockImplementation(() => {});

    render(
      <Controls
        heroRef={mockHeroRef}
        onTimeChange={vi.fn()}
        onDirectionChange={vi.fn()}
        onMotionChange={vi.fn()}
        onPauseChange={vi.fn()}
      />
    );

    // Wait for initial render
    await act(async () => {
      await new Promise(resolve => setTimeout(resolve, 50));
    });

    // Set up error after initial render
    mockHeroRef.current!.updateScene.mockImplementationOnce(() => {
      throw new Error('Update error');
    });

    const pauseButton = screen.getByRole('button', { name: /pause/i });
    await act(async () => {
      pauseButton.click();
      await new Promise(resolve => setTimeout(resolve, 50));
    });

    await waitFor(() => {
      expect(consoleErrorSpy).toHaveBeenCalledWith(expect.stringContaining('Error updating scene on pause'), expect.any(Error));
    }, { timeout: 2000 });

    consoleErrorSpy.mockRestore();
  });

  it('properly cleans up animation frame on unmount', async () => {
    const { unmount } = render(
      <Controls
        heroRef={mockHeroRef}
        onTimeChange={vi.fn()}
        onDirectionChange={vi.fn()}
        onMotionChange={vi.fn()}
        onPauseChange={vi.fn()}
      />
    );

    await act(async () => {
      await new Promise(resolve => setTimeout(resolve, 100));
    });

    await act(async () => {
      unmount();
      await new Promise(resolve => setTimeout(resolve, 50));
    });

    expect(cancelAnimationFrameSpy).toHaveBeenCalled();
  });

  it('stops animation when reduceMotion is enabled', async () => {
    render(
      <Controls
        heroRef={mockHeroRef}
        onTimeChange={vi.fn()}
        onDirectionChange={vi.fn()}
        onMotionChange={vi.fn()}
        onPauseChange={vi.fn()}
      />
    );

    await act(async () => {
      await new Promise(resolve => setTimeout(resolve, 50));
    });

    const reduceMotionButton = screen.getByRole('button', { name: /reduce|motion/i });
    await act(async () => {
      reduceMotionButton.click();
      await new Promise(resolve => setTimeout(resolve, 50));
    });

    await waitFor(() => {
      expect(cancelAnimationFrameSpy).toHaveBeenCalled();
    }, { timeout: 2000 });
  });

  it('updates year when slider changes', async () => {
    const onTimeChange = vi.fn();

    render(
      <Controls
        heroRef={mockHeroRef}
        onTimeChange={onTimeChange}
        onDirectionChange={vi.fn()}
        onMotionChange={vi.fn()}
        onPauseChange={vi.fn()}
      />
    );

    await act(async () => {
      await new Promise(resolve => setTimeout(resolve, 50));
    });

    const slider = screen.getByLabelText(/Time/i) as HTMLInputElement;
    
    await act(async () => {
      Object.defineProperty(slider, 'value', { value: '0.5', writable: true, configurable: true });
      slider.dispatchEvent(new Event('change', { bubbles: true }));
      await new Promise(resolve => setTimeout(resolve, 50));
    });

    await waitFor(() => {
      expect(onTimeChange).toHaveBeenCalled();
    }, { timeout: 2000 });
  });
});

