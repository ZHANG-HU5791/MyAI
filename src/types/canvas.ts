export type CanvasContentType = "empty" | "code" | "markdown" | "config_table" | "diff";

export interface CanvasContent {
  content_type: CanvasContentType;
  data: unknown;
}

export interface CanvasModification {
  line_range?: [number, number];
  instruction: string;
}
