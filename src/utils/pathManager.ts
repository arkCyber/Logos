/**
 * Path Manager Utility
 * Manages application file paths and directory structure
 */

export interface PathConfig {
  documents: {
    text: string;
    spreadsheets: string;
    presentations: string;
    templates: string;
  };
  exports: {
    pdf: string;
    svg: string;
    png: string;
  };
  assets: {
    images: string;
    fonts: string;
    icons: string;
  };
  system: {
    cache: string;
    backups: string;
    autosave: string;
    config: string;
    logs: string;
  };
}

class PathManager {
  private config: PathConfig;

  constructor() {
    this.config = {
      documents: {
        text: 'documents/text',
        spreadsheets: 'documents/spreadsheets',
        presentations: 'documents/presentations',
        templates: 'documents/templates'
      },
      exports: {
        pdf: 'exports/pdf',
        svg: 'exports/svg',
        png: 'exports/png'
      },
      assets: {
        images: 'assets/images',
        fonts: 'assets/fonts',
        icons: 'assets/icons'
      },
      system: {
        cache: 'cache',
        backups: 'backups',
        autosave: 'autosave',
        config: 'config',
        logs: 'logs'
      }
    };
  }

  /**
   * Get document path based on type
   */
  getDocumentPath(type: 'text' | 'spreadsheets' | 'presentations' | 'templates'): string {
    return this.config.documents[type];
  }

  /**
   * Get export path based on format
   */
  getExportPath(format: 'pdf' | 'svg' | 'png'): string {
    return this.config.exports[format];
  }

  /**
   * Get asset path based on type
   */
  getAssetPath(type: 'images' | 'fonts' | 'icons'): string {
    return this.config.assets[type];
  }

  /**
   * Get system path based on type
   */
  getSystemPath(type: 'cache' | 'backups' | 'autosave' | 'config' | 'logs'): string {
    return this.config.system[type];
  }

  /**
   * Get full path for a file
   */
  getFullPath(basePath: string, filename: string): string {
    return `${basePath}/${filename}`;
  }

  /**
   * Get all paths
   */
  getAllPaths(): PathConfig {
    return this.config;
  }
}

export const pathManager = new PathManager();
