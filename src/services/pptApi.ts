//! PPT API Service
//! Handles API interactions with the PPT backend service

// PPT data models
export interface PptSlide {
  id: string;
  title: string;
  content: string;
  layout: string;
  theme: string;
  background: string;
  index: number;
}

export interface PptTheme {
  id: string;
  name: string;
  colors: {
    primary: string;
    secondary: string;
    background: string;
    text: string;
  };
  fonts: {
    title: string;
    body: string;
  };
}

export interface PptExportOptions {
  format: 'pptx' | 'typst';
  embed_fonts: boolean;
  compress_images: boolean;
  image_quality: number;
  include_notes: boolean;
  include_hidden_slides: boolean;
}

export interface PptExportResult {
  pptx_data?: string; // Base64 encoded (for pptx format)
  typst_data?: string; // Typst code (for typst format)
  file_size: number;
  slide_count: number;
  generation_time_ms: number;
  success: boolean;
  error?: string;
}

// PPT API Service
class PptApiService {
  private baseUrl: string;

  constructor() {
    // Default to localhost for development
    this.baseUrl = 'http://localhost:8080/api';
  }

  setBaseUrl(url: string) {
    this.baseUrl = url;
  }

  // Slide operations
  async createSlide(slide: Partial<PptSlide>): Promise<PptSlide> {
    const response = await fetch(`${this.baseUrl}/slides`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify(slide)
    });

    if (!response.ok) {
      throw new Error(`Failed to create slide: ${response.statusText}`);
    }

    return response.json();
  }

  async getSlide(id: string): Promise<PptSlide> {
    const response = await fetch(`${this.baseUrl}/slides/${id}`);

    if (!response.ok) {
      throw new Error(`Failed to get slide: ${response.statusText}`);
    }

    return response.json();
  }

  async updateSlide(id: string, slide: Partial<PptSlide>): Promise<PptSlide> {
    const response = await fetch(`${this.baseUrl}/slides/${id}`, {
      method: 'PUT',
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify(slide)
    });

    if (!response.ok) {
      throw new Error(`Failed to update slide: ${response.statusText}`);
    }

    return response.json();
  }

  async deleteSlide(id: string): Promise<void> {
    const response = await fetch(`${this.baseUrl}/slides/${id}`, {
      method: 'DELETE'
    });

    if (!response.ok) {
      throw new Error(`Failed to delete slide: ${response.statusText}`);
    }
  }

  async getAllSlides(): Promise<PptSlide[]> {
    const response = await fetch(`${this.baseUrl}/slides`);

    if (!response.ok) {
      throw new Error(`Failed to get slides: ${response.statusText}`);
    }

    return response.json();
  }

  // Theme operations
  async getThemes(): Promise<PptTheme[]> {
    const response = await fetch(`${this.baseUrl}/themes`);

    if (!response.ok) {
      throw new Error(`Failed to get themes: ${response.statusText}`);
    }

    return response.json();
  }

  async applyTheme(slideId: string, themeId: string): Promise<void> {
    const response = await fetch(`${this.baseUrl}/slides/${slideId}/theme`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify({ theme_id: themeId })
    });

    if (!response.ok) {
      throw new Error(`Failed to apply theme: ${response.statusText}`);
    }
  }

  // Export operations
  async exportPresentation(slideIds: string[], options: PptExportOptions): Promise<PptExportResult> {
    const response = await fetch(`${this.baseUrl}/export`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify({
        slide_ids: slideIds,
        options
      })
    });

    if (!response.ok) {
      throw new Error(`Failed to export presentation: ${response.statusText}`);
    }

    return response.json();
  }

  // Tauri backend integration (for Rust backend)
  async createSlideTauri(slide: Partial<PptSlide>): Promise<PptSlide> {
    if (typeof window !== 'undefined' && (window as any).__TAURI__) {
      const { invoke } = await import('@tauri-apps/api/core');
      return invoke('create_ppt_slide', { slide });
    }
    throw new Error('Tauri not available');
  }

  async exportPresentationTauri(slideIds: string[], options: PptExportOptions): Promise<PptExportResult> {
    if (typeof window !== 'undefined' && (window as any).__TAURI__) {
      const { invoke } = await import('@tauri-apps/api/core');
      return invoke('export_ppt_presentation', { slideIds, options });
    }
    throw new Error('Tauri not available');
  }

  // Insert operations
  async insertImage(slideId: string, imageData: { type: 'upload' | 'url' | 'library'; data: string }): Promise<void> {
    const response = await fetch(`${this.baseUrl}/slides/${slideId}/images`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify(imageData)
    });

    if (!response.ok) {
      throw new Error(`Failed to insert image: ${response.statusText}`);
    }
  }

  async insertShape(slideId: string, shapeData: { type: string; position: { x: number; y: number }; size: { width: number; height: number } }): Promise<void> {
    const response = await fetch(`${this.baseUrl}/slides/${slideId}/shapes`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify(shapeData)
    });

    if (!response.ok) {
      throw new Error(`Failed to insert shape: ${response.statusText}`);
    }
  }

  async insertTable(slideId: string, tableData: { rows: number; cols: number; data?: string[][] }): Promise<void> {
    const response = await fetch(`${this.baseUrl}/slides/${slideId}/tables`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify(tableData)
    });

    if (!response.ok) {
      throw new Error(`Failed to insert table: ${response.statusText}`);
    }
  }

  // Slide management operations
  async duplicateSlide(slideId: string): Promise<PptSlide> {
    const response = await fetch(`${this.baseUrl}/slides/${slideId}/duplicate`, {
      method: 'POST'
    });

    if (!response.ok) {
      throw new Error(`Failed to duplicate slide: ${response.statusText}`);
    }

    return response.json();
  }

  async reorderSlides(slideIds: string[]): Promise<void> {
    const response = await fetch(`${this.baseUrl}/slides/reorder`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify({ slide_ids: slideIds })
    });

    if (!response.ok) {
      throw new Error(`Failed to reorder slides: ${response.statusText}`);
    }
  }

  // Animation operations
  async applyAnimation(slideId: string, animationType: string, options?: { duration?: number; direction?: string }): Promise<void> {
    const response = await fetch(`${this.baseUrl}/slides/${slideId}/animations`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify({ type: animationType, options })
    });

    if (!response.ok) {
      throw new Error(`Failed to apply animation: ${response.statusText}`);
    }
  }

  async removeAnimation(slideId: string, _animationId?: string): Promise<void> {
    const response = await fetch(`${this.baseUrl}/slides/${slideId}/animations`, {
      method: 'DELETE'
    });

    if (!response.ok) {
      throw new Error(`Failed to remove animation: ${response.statusText}`);
    }
  }

  async applyTransition(slideId: string, transitionType: string, duration?: number): Promise<void> {
    const response = await fetch(`${this.baseUrl}/slides/${slideId}/transition`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify({ type: transitionType, duration })
    });

    if (!response.ok) {
      throw new Error(`Failed to apply transition: ${response.statusText}`);
    }
  }

  // SmartArt operations
  async insertSmartArt(slideId: string, typeOrData: string | { nodes?: string[] } = {}, maybeData?: { nodes?: string[] }): Promise<void> {
    const payload: Record<string, unknown> = typeof typeOrData === 'string'
      ? { type: typeOrData, data: maybeData }
      : { data: typeOrData };
    const response = await fetch(`${this.baseUrl}/slides/${slideId}/smartart`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify(payload)
    });

    if (!response.ok) {
      throw new Error(`Failed to insert SmartArt: ${response.statusText}`);
    }
  }
}

// Export singleton instance
export const pptApi = new PptApiService();
