/**
 * pptApi Service Tests
 * Tests for the PPT API service including newly added methods
 */

import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { pptApi as api } from '../pptApi';

describe('PptApiService', () => {
  let fetchSpy: ReturnType<typeof vi.spyOn>;

  beforeEach(() => {
    fetchSpy = vi.spyOn(global, 'fetch');
  });

  afterEach(() => {
    fetchSpy.mockReset();
  });

  const mockResponse = (data: unknown, ok = true) => {
    return {
      ok,
      json: () => Promise.resolve(data),
      statusText: ok ? 'OK' : 'Error'
    };
  };

  // === Slide Operations ===

  describe('createSlide', () => {
    it('should create a slide with POST request', async () => {
      const slide = { title: 'Test Slide', content: 'Hello' };
      fetchSpy.mockResolvedValue(mockResponse({ id: 'slide-1', ...slide }));

      const result = await api.createSlide(slide);

      expect(fetchSpy).toHaveBeenCalledWith(
        'http://localhost:8080/api/slides',
        expect.objectContaining({ method: 'POST' })
      );
      expect(result.id).toBe('slide-1');
      expect(result.title).toBe('Test Slide');
    });

    it('should throw error when creation fails', async () => {
      fetchSpy.mockResolvedValue({
        ok: false,
        statusText: 'Server Error'
      } as Response);

      await expect(api.createSlide({ title: 'Test' })).rejects.toThrow('Failed to create slide');
    });
  });

  describe('getSlide', () => {
    it('should fetch a slide by id', async () => {
      fetchSpy.mockResolvedValue(mockResponse({ id: 'slide-1', title: 'Slide 1' }));

      const result = await api.getSlide('slide-1');

      expect(fetchSpy).toHaveBeenCalledWith('http://localhost:8080/api/slides/slide-1');
      expect(result.id).toBe('slide-1');
    });
  });

  describe('getAllSlides', () => {
    it('should fetch all slides', async () => {
      const slides = [
        { id: 'slide-1', title: 'Slide 1' },
        { id: 'slide-2', title: 'Slide 2' }
      ];
      fetchSpy.mockResolvedValue(mockResponse(slides));

      const result = await api.getAllSlides();

      expect(result).toHaveLength(2);
      expect(result[0].id).toBe('slide-1');
    });
  });

  describe('updateSlide', () => {
    it('should update a slide with PUT request', async () => {
      const updates = { title: 'Updated Title' };
      fetchSpy.mockResolvedValue(mockResponse({ id: 'slide-1', ...updates }));

      const result = await api.updateSlide('slide-1', updates);

      expect(fetchSpy).toHaveBeenCalledWith(
        'http://localhost:8080/api/slides/slide-1',
        expect.objectContaining({ method: 'PUT' })
      );
      expect(result.title).toBe('Updated Title');
    });
  });

  describe('deleteSlide', () => {
    it('should delete a slide with DELETE request', async () => {
      fetchSpy.mockResolvedValue(mockResponse(null));

      await api.deleteSlide('slide-1');

      expect(fetchSpy).toHaveBeenCalledWith(
        'http://localhost:8080/api/slides/slide-1',
        expect.objectContaining({ method: 'DELETE' })
      );
    });

    it('should throw error when deletion fails', async () => {
      fetchSpy.mockResolvedValue({ ok: false, statusText: 'Not Found' } as Response);

      await expect(api.deleteSlide('nonexistent')).rejects.toThrow('Failed to delete slide');
    });
  });

  describe('duplicateSlide', () => {
    it('should duplicate a slide', async () => {
      fetchSpy.mockResolvedValue(mockResponse({ id: 'slide-copy', title: 'Slide Copy' }));

      const result = await api.duplicateSlide('slide-1');

      expect(fetchSpy).toHaveBeenCalledWith(
        'http://localhost:8080/api/slides/slide-1/duplicate',
        expect.objectContaining({ method: 'POST' })
      );
      expect(result.id).toBe('slide-copy');
    });
  });

  describe('reorderSlides', () => {
    it('should reorder slides', async () => {
      fetchSpy.mockResolvedValue(mockResponse(null));

      await api.reorderSlides(['slide-3', 'slide-1', 'slide-2']);

      expect(fetchSpy).toHaveBeenCalledWith(
        'http://localhost:8080/api/slides/reorder',
        expect.objectContaining({
          method: 'POST',
          body: JSON.stringify({ slide_ids: ['slide-3', 'slide-1', 'slide-2'] })
        })
      );
    });
  });

  // === Theme Operations ===

  describe('getThemes', () => {
    it('should fetch available themes', async () => {
      const themes = [
        { id: 'theme-1', name: 'Dark Theme' },
        { id: 'theme-2', name: 'Light Theme' }
      ];
      fetchSpy.mockResolvedValue(mockResponse(themes));

      const result = await api.getThemes();

      expect(result).toHaveLength(2);
    });
  });

  describe('applyTheme', () => {
    it('should apply theme to slide', async () => {
      fetchSpy.mockResolvedValue(mockResponse(null));

      await api.applyTheme('slide-1', 'dark-theme');

      expect(fetchSpy).toHaveBeenCalledWith(
        'http://localhost:8080/api/slides/slide-1/theme',
        expect.objectContaining({
          method: 'POST',
          body: JSON.stringify({ theme_id: 'dark-theme' })
        })
      );
    });
  });

  // === Animation Operations (NEW) ===

  describe('applyAnimation', () => {
    it('should apply animation to a slide', async () => {
      fetchSpy.mockResolvedValue(mockResponse(null));

      await api.applyAnimation('slide-1', 'fade');

      expect(fetchSpy).toHaveBeenCalledWith(
        'http://localhost:8080/api/slides/slide-1/animation',
        expect.objectContaining({
          method: 'POST',
          body: JSON.stringify({ type: 'fade', options: undefined })
        })
      );
    });

    it('should apply animation with options', async () => {
      fetchSpy.mockResolvedValue(mockResponse(null));

      await api.applyAnimation('slide-1', 'fly-in', { duration: 500, direction: 'left' });

      expect(fetchSpy).toHaveBeenCalledWith(
        'http://localhost:8080/api/slides/slide-1/animation',
        expect.objectContaining({
          method: 'POST',
          body: JSON.stringify({ type: 'fly-in', options: { duration: 500, direction: 'left' } })
        })
      );
    });

    it('should throw error when animation fails', async () => {
      fetchSpy.mockResolvedValue({ ok: false, statusText: 'Server Error' } as Response);

      await expect(api.applyAnimation('slide-1', 'fade')).rejects.toThrow('Failed to apply animation');
    });
  });

  describe('removeAnimation', () => {
    it('should remove animation from a slide', async () => {
      fetchSpy.mockResolvedValue(mockResponse(null));

      await api.removeAnimation('slide-1');

      expect(fetchSpy).toHaveBeenCalledWith(
        'http://localhost:8080/api/slides/slide-1/animation',
        expect.objectContaining({ method: 'DELETE' })
      );
    });

    it('should throw error when removal fails', async () => {
      fetchSpy.mockResolvedValue({ ok: false, statusText: 'Server Error' } as Response);

      await expect(api.removeAnimation('slide-1')).rejects.toThrow('Failed to remove animation');
    });
  });

  // === Transition Operations (NEW) ===

  describe('applyTransition', () => {
    it('should apply transition to a slide', async () => {
      fetchSpy.mockResolvedValue(mockResponse(null));

      await api.applyTransition('slide-1', 'fade');

      expect(fetchSpy).toHaveBeenCalledWith(
        'http://localhost:8080/api/slides/slide-1/transition',
        expect.objectContaining({
          method: 'POST',
          body: JSON.stringify({ type: 'fade', duration: undefined })
        })
      );
    });

    it('should apply transition with duration', async () => {
      fetchSpy.mockResolvedValue(mockResponse(null));

      await api.applyTransition('slide-1', 'wipe', 2.5);

      expect(fetchSpy).toHaveBeenCalledWith(
        'http://localhost:8080/api/slides/slide-1/transition',
        expect.objectContaining({
          method: 'POST',
          body: JSON.stringify({ type: 'wipe', duration: 2.5 })
        })
      );
    });

    it('should throw error when transition fails', async () => {
      fetchSpy.mockResolvedValue({ ok: false, statusText: 'Server Error' } as Response);

      await expect(api.applyTransition('slide-1', 'fade')).rejects.toThrow('Failed to apply transition');
    });
  });

  // === SmartArt Operations (NEW) ===

  describe('insertSmartArt', () => {
    it('should insert SmartArt into a slide', async () => {
      fetchSpy.mockResolvedValue(mockResponse(null));

      await api.insertSmartArt('slide-1', 'process');

      expect(fetchSpy).toHaveBeenCalledWith(
        'http://localhost:8080/api/slides/slide-1/smartart',
        expect.objectContaining({
          method: 'POST',
          body: JSON.stringify({ type: 'process', data: undefined })
        })
      );
    });

    it('should insert SmartArt with data', async () => {
      fetchSpy.mockResolvedValue(mockResponse(null));
      const smartArtData = { nodes: ['Step 1', 'Step 2', 'Step 3'] };

      await api.insertSmartArt('slide-1', 'cycle', smartArtData);

      expect(fetchSpy).toHaveBeenCalledWith(
        'http://localhost:8080/api/slides/slide-1/smartart',
        expect.objectContaining({
          method: 'POST',
          body: JSON.stringify({ type: 'cycle', data: smartArtData })
        })
      );
    });

    it('should throw error when SmartArt insertion fails', async () => {
      fetchSpy.mockResolvedValue({ ok: false, statusText: 'Server Error' } as Response);

      await expect(api.insertSmartArt('slide-1', 'process')).rejects.toThrow('Failed to insert SmartArt');
    });
  });

  // === Insert Operations ===

  describe('insertImage', () => {
    it('should insert image into a slide', async () => {
      fetchSpy.mockResolvedValue(mockResponse(null));
      const imageData = { type: 'url' as const, data: 'https://example.com/image.png' };

      await api.insertImage('slide-1', imageData);

      expect(fetchSpy).toHaveBeenCalledWith(
        'http://localhost:8080/api/slides/slide-1/images',
        expect.objectContaining({
          method: 'POST',
          body: JSON.stringify(imageData)
        })
      );
    });
  });

  describe('insertShape', () => {
    it('should insert shape into a slide', async () => {
      fetchSpy.mockResolvedValue(mockResponse(null));
      const shapeData = {
        type: 'rectangle',
        position: { x: 100, y: 200 },
        size: { width: 150, height: 100 }
      };

      await api.insertShape('slide-1', shapeData);

      expect(fetchSpy).toHaveBeenCalledWith(
        'http://localhost:8080/api/slides/slide-1/shapes',
        expect.objectContaining({
          method: 'POST',
          body: JSON.stringify(shapeData)
        })
      );
    });
  });

  describe('insertTable', () => {
    it('should insert table into a slide', async () => {
      fetchSpy.mockResolvedValue(mockResponse(null));
      const tableData = { rows: 3, cols: 3 };

      await api.insertTable('slide-1', tableData);

      expect(fetchSpy).toHaveBeenCalledWith(
        'http://localhost:8080/api/slides/slide-1/tables',
        expect.objectContaining({
          method: 'POST',
          body: JSON.stringify(tableData)
        })
      );
    });
  });

  // === Export Operations ===

  describe('exportPresentation', () => {
    it('should export presentation with options', async () => {
      const exportResult = {
        pptx_data: 'base64data',
        file_size: 1024,
        slide_count: 5,
        generation_time_ms: 200,
        success: true
      };
      fetchSpy.mockResolvedValue(mockResponse(exportResult));
      const options = {
        format: 'pptx' as const,
        embed_fonts: true,
        compress_images: false,
        image_quality: 85,
        include_notes: true,
        include_hidden_slides: false
      };

      const result = await api.exportPresentation(['slide-1', 'slide-2'], options);

      expect(fetchSpy).toHaveBeenCalledWith(
        'http://localhost:8080/api/export',
        expect.objectContaining({
          method: 'POST',
          body: JSON.stringify({
            slide_ids: ['slide-1', 'slide-2'],
            options
          })
        })
      );
      expect(result.success).toBe(true);
      expect(result.slide_count).toBe(5);
    });
  });

  // === setBaseUrl ===

  describe('setBaseUrl', () => {
    it('should change the base URL', () => {
      api.setBaseUrl('https://custom-server.com/api');

      expect((api as any).baseUrl).toBe('https://custom-server.com/api');
    });
  });
});
