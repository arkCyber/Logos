/**
 * Aerospace-grade unified data platform format for presentations
 * Provides standardized data structure for PPT functionality across different rendering engines
 */

import { z } from 'zod';

// Element types
export enum ElementType {
  TEXT = 'text',
  IMAGE = 'image',
  SHAPE = 'shape',
  CHART = 'chart',
  TABLE = 'table',
  VIDEO = 'video',
  CODE = 'code',
}

// Slide layout types
export enum SlideLayout {
  TITLE = 'title',
  TITLE_AND_CONTENT = 'title_and_content',
  TWO_CONTENT = 'two_content',
  BLANK = 'blank',
  SECTION_HEADER = 'section_header',
  COMPARISON = 'comparison',
  CONTENT_WITH_CAPTION = 'content_with_caption',
}

// Animation types
export enum AnimationType {
  NONE = 'none',
  FADE_IN = 'fade_in',
  SLIDE_IN = 'slide_in',
  ZOOM_IN = 'zoom_in',
  BOUNCE_IN = 'bounce_in',
}

// Transition types
export enum TransitionType {
  NONE = 'none',
  SLIDE_LEFT = 'slide_left',
  SLIDE_RIGHT = 'slide_right',
  SLIDE_UP = 'slide_up',
  SLIDE_DOWN = 'slide_down',
  FADE = 'fade',
  ZOOM = 'zoom',
}

// Position and size
export interface Position {
  x: number;
  y: number;
}

export interface Size {
  width: number;
  height: number;
}

// Base element interface
export interface BaseElement {
  id: string;
  type: ElementType;
  position: Position;
  size: Size;
  rotation?: number;
  opacity?: number;
  zIndex?: number;
  locked?: boolean;
  visible?: boolean;
  animation?: {
    type: AnimationType;
    duration: number;
    delay: number;
    easing?: string;
  };
}

// Text element
export interface TextElement extends BaseElement {
  type: ElementType.TEXT;
  content: string;
  style: {
    fontFamily?: string;
    fontSize?: number;
    fontWeight?: 'normal' | 'bold' | '100' | '200' | '300' | '400' | '500' | '600' | '700' | '800' | '900';
    fontStyle?: 'normal' | 'italic';
    textDecoration?: 'none' | 'underline' | 'line-through';
    color?: string;
    backgroundColor?: string;
    textAlign?: 'left' | 'center' | 'right' | 'justify';
    lineHeight?: number;
    letterSpacing?: number;
  };
}

// Image element
export interface ImageElement extends BaseElement {
  type: ElementType.IMAGE;
  src: string;
  alt?: string;
  style?: {
    objectFit?: 'contain' | 'cover' | 'fill' | 'none';
    borderRadius?: number;
    borderWidth?: number;
    borderColor?: string;
  };
}

// Shape element
export interface ShapeElement extends BaseElement {
  type: ElementType.SHAPE;
  shape: 'rectangle' | 'circle' | 'triangle' | 'star' | 'arrow' | 'line';
  style: {
    fillColor?: string;
    strokeColor?: string;
    strokeWidth?: number;
    borderRadius?: number;
  };
}

// Chart element
export interface ChartElement extends BaseElement {
  type: ElementType.CHART;
  chartType: 'bar' | 'line' | 'pie' | 'scatter' | 'area';
  data: {
    labels: string[];
    datasets: {
      label: string;
      data: number[];
      backgroundColor?: string;
      borderColor?: string;
    }[];
  };
  style?: {
    showLegend?: boolean;
    showGrid?: boolean;
  };
}

// Table element
export interface TableElement extends BaseElement {
  type: ElementType.TABLE;
  rows: number;
  columns: number;
  data: string[][];
  style?: {
    headerBackgroundColor?: string;
    alternateRowColor?: string;
    borderWidth?: number;
    borderColor?: string;
  };
}

// Video element
export interface VideoElement extends BaseElement {
  type: ElementType.VIDEO;
  src: string;
  autoplay?: boolean;
  loop?: boolean;
  muted?: boolean;
  controls?: boolean;
}

// Code element
export interface CodeElement extends BaseElement {
  type: ElementType.CODE;
  content: string;
  language?: string;
  style?: {
    theme?: 'light' | 'dark';
    showLineNumbers?: boolean;
    fontSize?: number;
  };
}

// Union type for all elements
export type SlideElement = 
  | TextElement 
  | ImageElement 
  | ShapeElement 
  | ChartElement 
  | TableElement 
  | VideoElement 
  | CodeElement;

// Slide interface
export interface Slide {
  id: string;
  index: number;
  layout: SlideLayout;
  title?: string;
  elements: SlideElement[];
  notes?: string;
  transition?: {
    type: TransitionType;
    duration: number;
  };
  hidden?: boolean;
  background?: {
    type: 'color' | 'gradient' | 'image';
    value: string;
  };
}

// Theme interface
export interface PresentationTheme {
  name: string;
  colors: {
    primary: string;
    secondary: string;
    background: string;
    text: string;
    accent: string;
  };
  fonts: {
    heading: string;
    body: string;
    code: string;
  };
  spacing: {
    padding: number;
    margin: number;
  };
}

// Document metadata
export interface DocumentMetadata {
  id: string;
  type: 'presentation';
  title: string;
  author?: string;
  version: string;
  created_at: string;
  updated_at: string;
  thumbnail?: string;
}

// Complete presentation document
export interface PresentationDocument {
  metadata: DocumentMetadata;
  theme: PresentationTheme;
  slides: Slide[];
  settings: {
    aspectRatio: '16-9' | '4-3' | '1-1';
    showSlideNumbers: boolean;
    enableTransitions: boolean;
    autoPlay?: boolean;
    autoPlayInterval?: number;
  };
}

// Zod schemas for validation
export const PositionSchema = z.object({
  x: z.number(),
  y: z.number()
});

export const SizeSchema = z.object({
  width: z.number().positive(),
  height: z.number().positive()
});

export const BaseElementSchema = z.object({
  id: z.string().uuid(),
  type: z.nativeEnum(ElementType),
  position: PositionSchema,
  size: SizeSchema,
  rotation: z.number().optional(),
  opacity: z.number().min(0).max(1).optional(),
  zIndex: z.number().int().optional(),
  locked: z.boolean().optional(),
  visible: z.boolean().optional(),
  animation: z.object({
    type: z.nativeEnum(AnimationType),
    duration: z.number().positive(),
    delay: z.number().nonnegative(),
    easing: z.string().optional()
  }).optional()
});

export const TextElementSchema = BaseElementSchema.extend({
  type: z.literal(ElementType.TEXT),
  content: z.string(),
  style: z.object({
    fontFamily: z.string().optional(),
    fontSize: z.number().positive().optional(),
    fontWeight: z.string().optional(),
    fontStyle: z.enum(['normal', 'italic']).optional(),
    textDecoration: z.enum(['none', 'underline', 'line-through']).optional(),
    color: z.string().optional(),
    backgroundColor: z.string().optional(),
    textAlign: z.enum(['left', 'center', 'right', 'justify']).optional(),
    lineHeight: z.number().positive().optional(),
    letterSpacing: z.number().optional()
  }).optional()
});

export const ImageElementSchema = BaseElementSchema.extend({
  type: z.literal(ElementType.IMAGE),
  src: z.string().url(),
  alt: z.string().optional(),
  style: z.object({
    objectFit: z.enum(['contain', 'cover', 'fill', 'none']).optional(),
    borderRadius: z.number().nonnegative().optional(),
    borderWidth: z.number().nonnegative().optional(),
    borderColor: z.string().optional()
  }).optional()
});

export const SlideSchema = z.object({
  id: z.string().uuid(),
  index: z.number().int().nonnegative(),
  layout: z.nativeEnum(SlideLayout),
  title: z.string().optional(),
  elements: z.array(z.any()), // Simplified for now
  notes: z.string().optional(),
  transition: z.object({
    type: z.nativeEnum(TransitionType),
    duration: z.number().positive()
  }).optional(),
  hidden: z.boolean().optional(),
  background: z.object({
    type: z.enum(['color', 'gradient', 'image']),
    value: z.string()
  }).optional()
});

export const PresentationDocumentSchema = z.object({
  metadata: z.object({
    id: z.string().uuid(),
    type: z.literal('presentation'),
    title: z.string().min(1).max(255),
    author: z.string().optional(),
    version: z.string(),
    created_at: z.string(),
    updated_at: z.string(),
    thumbnail: z.string().optional()
  }),
  theme: z.object({
    name: z.string(),
    colors: z.object({
      primary: z.string(),
      secondary: z.string(),
      background: z.string(),
      text: z.string(),
      accent: z.string()
    }),
    fonts: z.object({
      heading: z.string(),
      body: z.string(),
      code: z.string()
    }),
    spacing: z.object({
      padding: z.number(),
      margin: z.number()
    })
  }),
  slides: z.array(SlideSchema),
  settings: z.object({
    aspectRatio: z.enum(['16-9', '4-3', '1-1']),
    showSlideNumbers: z.boolean(),
    enableTransitions: z.boolean(),
    autoPlay: z.boolean().optional(),
    autoPlayInterval: z.number().positive().optional()
  })
});

// Helper functions
export function createEmptyPresentation(title: string): PresentationDocument {
  return {
    metadata: {
      id: crypto.randomUUID(),
      type: 'presentation',
      title,
      version: '1.0.0',
      created_at: new Date().toISOString(),
      updated_at: new Date().toISOString()
    },
    theme: {
      name: 'default',
      colors: {
        primary: '#007bff',
        secondary: '#6c757d',
        background: '#ffffff',
        text: '#333333',
        accent: '#28a745'
      },
      fonts: {
        heading: 'Arial',
        body: 'Helvetica',
        code: 'Courier New'
      },
      spacing: {
        padding: 20,
        margin: 10
      }
    },
    slides: [],
    settings: {
      aspectRatio: '16-9',
      showSlideNumbers: true,
      enableTransitions: true
    }
  };
}

export function createEmptySlide(index: number, layout: SlideLayout = SlideLayout.BLANK): Slide {
  return {
    id: crypto.randomUUID(),
    index,
    layout,
    elements: [],
    background: {
      type: 'color',
      value: '#ffffff'
    }
  };
}

export function validatePresentationDocument(doc: unknown): PresentationDocument {
  return PresentationDocumentSchema.parse(doc);
}
