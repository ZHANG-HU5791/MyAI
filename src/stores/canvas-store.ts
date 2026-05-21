import { defineStore } from "pinia";
import { ref } from "vue";
import type { CanvasContentType } from "@/types/canvas";

export const useCanvasStore = defineStore("canvas", () => {
  const activeSessionId = ref<string | null>(null);
  const contentType = ref<CanvasContentType>("empty");
  const content = ref<unknown>(null);
  const language = ref<string>("");
  const isEditing = ref(false);

  function setContent(type: CanvasContentType, data: unknown, lang?: string) {
    contentType.value = type;
    content.value = data;
    if (lang) language.value = lang;
  }

  function clear() {
    contentType.value = "empty";
    content.value = null;
    language.value = "";
    isEditing.value = false;
  }

  return {
    activeSessionId,
    contentType,
    content,
    language,
    isEditing,
    setContent,
    clear,
  };
});
