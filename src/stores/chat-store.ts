import { defineStore } from "pinia";
import { ref, computed } from "vue";
import { useIpc } from "@/composables/useIpc";

export interface Message {
  id: string;
  session_id: string;
  role: "user" | "assistant" | "system" | "tool";
  content: string;
  model_used?: string;
  token_count?: number;
  created_at: string;
}

export interface Session {
  id: string;
  title: string;
  created_at: string;
  updated_at: string;
}

export const useChatStore = defineStore("chat", () => {
  const sessions = ref<Session[]>([]);
  const messages = ref<Message[]>([]);
  const activeSessionId = ref<string | null>(null);
  const isStreaming = ref(false);
  const streamingContent = ref("");
  const intentRouting = ref<{ queryType: string; model: string; reason: string } | null>(null);

  const { call } = useIpc();

  const activeMessages = computed(() =>
    messages.value.filter((m) => m.session_id === activeSessionId.value)
  );

  async function loadSessions() {
    sessions.value = await call<Session[]>("list_sessions");
  }

  async function createSession(title: string) {
    const id = await call<string>("create_session", { title });
    await loadSessions();
    activeSessionId.value = id;
    messages.value = [];
    return id;
  }

  async function deleteSession(sessionId: string) {
    await call("delete_session", { sessionId });
    if (activeSessionId.value === sessionId) {
      activeSessionId.value = null;
      messages.value = [];
    }
    await loadSessions();
  }

  async function loadMessages(sessionId: string) {
    messages.value = await call<Message[]>("get_messages", { sessionId });
  }

  async function sendMessage(content: string) {
    if (!activeSessionId.value) return;

    // Add user message optimistically
    const userMsg: Message = {
      id: crypto.randomUUID(),
      session_id: activeSessionId.value,
      role: "user",
      content,
      created_at: new Date().toISOString(),
    };
    messages.value.push(userMsg);

    // Start streaming
    isStreaming.value = true;
    streamingContent.value = "";

    // Send to backend
    await call("send_message", {
      sessionId: activeSessionId.value,
      content,
    });
  }

  function appendToStream(chunk: string) {
    streamingContent.value += chunk;
  }

  function finalizeStream(messageId: string) {
    if (streamingContent.value) {
      messages.value.push({
        id: messageId,
        session_id: activeSessionId.value!,
        role: "assistant",
        content: streamingContent.value,
        created_at: new Date().toISOString(),
      });
    }
    streamingContent.value = "";
    isStreaming.value = false;
  }

  function setIntentRouting(queryType: string, model: string, reason: string) {
    intentRouting.value = { queryType, model, reason };
  }

  return {
    sessions,
    messages,
    activeSessionId,
    isStreaming,
    streamingContent,
    intentRouting,
    activeMessages,
    loadSessions,
    createSession,
    deleteSession,
    loadMessages,
    sendMessage,
    appendToStream,
    finalizeStream,
    setIntentRouting,
  };
});
