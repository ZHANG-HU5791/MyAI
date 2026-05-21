<script setup lang="ts">
import { ref, nextTick } from "vue";

const emit = defineEmits<{ send: [content: string] }>();

const input = ref("");
const textarea = ref<HTMLTextAreaElement>();

function handleKeydown(e: KeyboardEvent) {
  if (e.key === "Enter" && !e.shiftKey) {
    e.preventDefault();
    send();
  }
}

function send() {
  const content = input.value.trim();
  if (!content) return;
  emit("send", content);
  input.value = "";
  nextTick(() => autoResize());
}

function autoResize() {
  if (!textarea.value) return;
  textarea.value.style.height = "auto";
  textarea.value.style.height = Math.min(textarea.value.scrollHeight, 200) + "px";
}
</script>

<template>
  <div class="border-t border-surface-800 p-3 shrink-0">
    <div class="flex gap-2 items-end">
      <textarea
        ref="textarea"
        v-model="input"
        placeholder="Type a message... (Shift+Enter for new line)"
        rows="1"
        class="input-field flex-1 resize-none min-h-[36px] max-h-[200px]"
        @input="autoResize"
        @keydown="handleKeydown"
      ></textarea>
      <button
        class="button-primary h-9 px-4"
        :disabled="!input.trim()"
        @click="send"
      >
        Send
      </button>
    </div>
  </div>
</template>
