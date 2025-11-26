import React from 'react';
import { render, screen, waitFor, act } from '@testing-library/react';
import { describe, expect, it, vi, beforeEach, afterEach } from 'vitest';
import ResearchGradeHero from '@/app/components/ResearchGradeHero';

const mockScene = {
  update: vi.fn(),
  resize: vi.fn(),
  dispose: vi.fn(),
  toggleComponent: vi.fn(),
  getVisibility: vi.fn(() => ({
    heliosphere: true,
    terminationShock: true,
    heliopause: true,
    bowShock: false,
    solarWind: true,
    interstellarWind: true,
    planets: true,
    orbits: true,
    spacecraft: true,
    trajectories: true,
    stars: true,
    coordinateGrid: false,
    distanceMarkers: true,
    dataOverlay: true
  })),
  setTimeMode: vi.fn(),
  getCurrentDate: vi.fn(() => new Date())
};

vi.mock('@/app/lib/ResearchGradeHeliosphereScene', () => ({
  createResearchGradeScene: vi.fn(async () => mockScene)
}));

vi.mock('@/app/lib/services/AstronomicalDataService', () => ({
  getAstronomicalDataService: vi.fn(() => ({
    getDataStore: vi.fn(() => ({
      getSpacecraftPosition: vi.fn((name: string) => ({
        distance: name === 'Voyager 1' ? 163 : 136,
        velocity: { length: () => (name === 'Voyager 1' ? 17.0 : 15.4) },
        lightTime: name === 'Voyager 1' ? 1356 : 1128
      })),
      solarCycle: {
        sunspotNumber: {
          interpolate: vi.fn(() => 100)
        }
      }
    })),
    getSolarWindConditions: vi.fn(() => ({
      speed: 400,
      density: 5,
      temperature: 1.2e5,
      pressure: 2.0,
      magneticField: { length: () => 5 }
    }))
  }))
}));

describe('ResearchGradeHero', () => {
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
    
    // Reset all mocks
    vi.clearAllMocks();
    mockScene.update.mockClear();
    mockScene.dispose.mockClear();
  });

  afterEach(() => {
    requestAnimationFrameSpy.mockRestore();
    cancelAnimationFrameSpy.mockRestore();
  });

  it('marks the canvas as ready after scene initialization', async () => {
    render(<ResearchGradeHero />);
    const canvas = await screen.findByTestId('research-scene-canvas');

    await waitFor(() =>
      expect(canvas).toHaveAttribute('data-scene-ready', 'true')
    );
  });

  it('properly cleans up animation frame on unmount', async () => {
    const { unmount } = render(<ResearchGradeHero />);
    
    await waitFor(() => {
      expect(screen.getByTestId('research-scene-canvas')).toHaveAttribute('data-scene-ready', 'true');
    });

    const playButton = screen.getByRole('button', { name: /play|pause/i });
    act(() => {
      playButton.click();
    });

    unmount();

    await waitFor(() => {
      expect(mockScene.dispose).toHaveBeenCalled();
    });
  });

  it('handles WebGL context failure gracefully', () => {
    const getContextSpy = vi.spyOn(HTMLCanvasElement.prototype, 'getContext');
    getContextSpy.mockReturnValue(null);

    render(<ResearchGradeHero />);
    
    expect(screen.getByText(/WebGL Not Supported/i)).toBeInTheDocument();
    
    getContextSpy.mockRestore();
  });

  it('handles scene initialization errors gracefully', async () => {
    const { createResearchGradeScene } = await import('@/app/lib/ResearchGradeHeliosphereScene');
    vi.mocked(createResearchGradeScene).mockRejectedValueOnce(new Error('Initialization failed'));

    render(<ResearchGradeHero />);
    
    await waitFor(() => {
      expect(screen.getByText(/WebGL Not Supported/i)).toBeInTheDocument();
    });
  });

  it('stops animation loop when isPlaying is false', async () => {
    render(<ResearchGradeHero />);
    
    await waitFor(() => {
      expect(screen.getByTestId('research-scene-canvas')).toHaveAttribute('data-scene-ready', 'true');
    });

    // Animation should not be running initially (isPlaying defaults to false)
    await act(async () => {
      await new Promise(resolve => setTimeout(resolve, 100));
    });

    // Should not have called update with motion enabled
    expect(mockScene.update).toHaveBeenCalledWith(expect.any(Date), 0, false);
  });

  it('handles resize events correctly', async () => {
    render(<ResearchGradeHero />);
    
    await waitFor(() => {
      expect(screen.getByTestId('research-scene-canvas')).toHaveAttribute('data-scene-ready', 'true');
    });

    act(() => {
      window.dispatchEvent(new Event('resize'));
    });

    await waitFor(() => {
      expect(mockScene.resize).toHaveBeenCalled();
    });
  });

  it('handles errors in resize handler gracefully', async () => {
    mockScene.resize.mockImplementationOnce(() => {
      throw new Error('Resize error');
    });

    render(<ResearchGradeHero />);
    
    await waitFor(() => {
      expect(screen.getByTestId('research-scene-canvas')).toHaveAttribute('data-scene-ready', 'true');
    });

    const consoleErrorSpy = vi.spyOn(console, 'error').mockImplementation(() => {});

    act(() => {
      window.dispatchEvent(new Event('resize'));
    });

    await waitFor(() => {
      expect(consoleErrorSpy).toHaveBeenCalledWith(expect.stringContaining('Error in resize handler'), expect.any(Error));
    });

    consoleErrorSpy.mockRestore();
  });

  it('handles errors in animation loop gracefully', async () => {
    // First call succeeds (during initialization), second call throws (in animation loop)
    let callCount = 0;
    mockScene.update.mockImplementation(() => {
      callCount++;
      if (callCount === 2) {
        throw new Error('Animation error');
      }
    });

    render(<ResearchGradeHero />);
    
    await waitFor(() => {
      expect(screen.getByTestId('research-scene-canvas')).toHaveAttribute('data-scene-ready', 'true');
    });

    const consoleErrorSpy = vi.spyOn(console, 'error').mockImplementation(() => {});

    const playButton = screen.getByRole('button', { name: /play|pause/i });
    act(() => {
      playButton.click();
    });

    await waitFor(() => {
      expect(rafCallbacks.length).toBeGreaterThan(0);
    });

    await act(async () => {
      const callbacks = [...rafCallbacks];
      rafCallbacks = [];
      callbacks.forEach(cb => cb(performance.now()));
      await new Promise(resolve => setTimeout(resolve, 10));
    });

    await waitFor(() => {
      expect(consoleErrorSpy).toHaveBeenCalledWith(expect.stringContaining('Error in animation loop'), expect.any(Error));
    });

    consoleErrorSpy.mockRestore();
  });

  it('handles errors in data overlay update gracefully', async () => {
    const { getAstronomicalDataService } = await import('@/app/lib/services/AstronomicalDataService');
    const mockService = vi.mocked(getAstronomicalDataService)();
    vi.mocked(mockService.getDataStore).mockImplementationOnce(() => {
      throw new Error('Data service error');
    });

    render(<ResearchGradeHero />);
    
    await waitFor(() => {
      expect(screen.getByTestId('research-scene-canvas')).toHaveAttribute('data-scene-ready', 'true');
    });

    const consoleErrorSpy = vi.spyOn(console, 'error').mockImplementation(() => {});

    const playButton = screen.getByRole('button', { name: /play|pause/i });
    act(() => {
      playButton.click();
    });

    await waitFor(() => {
      expect(rafCallbacks.length).toBeGreaterThan(0);
    });

    await act(async () => {
      const callbacks = [...rafCallbacks];
      rafCallbacks = [];
      callbacks.forEach(cb => cb(performance.now()));
      await new Promise(resolve => setTimeout(resolve, 10));
    });

    await waitFor(() => {
      expect(consoleErrorSpy).toHaveBeenCalledWith(expect.stringContaining('Error updating data overlay'), expect.any(Error));
    });

    consoleErrorSpy.mockRestore();
  });

  it('properly disposes scene on cleanup', async () => {
    const { unmount } = render(<ResearchGradeHero />);
    
    await waitFor(() => {
      expect(screen.getByTestId('research-scene-canvas')).toHaveAttribute('data-scene-ready', 'true');
    });

    unmount();

    await waitFor(() => {
      expect(mockScene.dispose).toHaveBeenCalled();
    });
  });

  it('removes resize event listener on cleanup', async () => {
    const removeEventListenerSpy = vi.spyOn(window, 'removeEventListener');
    
    const { unmount } = render(<ResearchGradeHero />);
    
    await waitFor(() => {
      expect(screen.getByTestId('research-scene-canvas')).toHaveAttribute('data-scene-ready', 'true');
    });

    unmount();

    await waitFor(() => {
      expect(removeEventListenerSpy).toHaveBeenCalledWith('resize', expect.any(Function));
    });

    removeEventListenerSpy.mockRestore();
  });
});
