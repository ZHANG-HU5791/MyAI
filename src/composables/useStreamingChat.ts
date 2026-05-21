import { computed } from "vue";
import { useChatStore } from "@/stores/chat-store";
import { useIpc } from "./useIpc";
import type { ChatStreamChunkEvent, ChatCompleteEvent } from "@/types/events";

export function useStreamingChat() {
  const chatStore = useChatStore();
  const { onEvent } = useIpc();

  // Listen for streaming chunks
  onEvent<ChatStreamChunkEvent>("ChatStreamChunk", (payload) => {
    if (payload.session_id === chatStore.activeSessionId) {
      chatStore.appendToStream(payload.chunk);
    }
  });

  // Listen for stream completion
  onEvent<ChatCompleteEvent>("ChatComplete", (payload) => {
    if (payload.session_id === chatStore.activeSessionId) {
      chatStore.finalizeStream(payload.message_id);
    }
  });

  const isStreaming = computed(() => chatStore.isStreaming);

  return { isStreaming };
}
