import { useCanvasStore } from "@/stores/canvas-store";
import { useIpc } from "./useIpc";

export function useCanvasInteraction() {
  const { call } = useIpc();
  const canvasStore = useCanvasStore();

  async function requestModification(context: {
    lineRange?: [number, number];
    instruction: string;
  }): Promise<void> {
    await call("apply_canvas_modification", {
      sessionId: canvasStore.activeSessionId,
      modification: context,
    });
  }

  async function copyToClipboard(content: string): Promise<void> {
    await navigator.clipboard.writeText(content);
  }

  return { requestModification, copyToClipboard };
}
