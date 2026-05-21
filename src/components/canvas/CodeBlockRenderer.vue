<script setup lang="ts">
import { ref, onMounted, watch } from "vue";
import { useCanvasInteraction } from "@/composables/useCanvasInteraction";

const props = defineProps<{
  code: string;
  language: string;
}>();

const { copyToClipboard } = useCanvasInteraction();
const lines = ref<string[]>([]);
const copied = ref(false);

function splitLines() {
  lines.value = props.code.split("\n");
}

watch(() => props.code, splitLines);
onMounted(splitLines);

async function copy() {
  await copyToClipboard(props.code);
  copied.value = true;
  setTimeout(() => (copied.value = false), 2000);
}
</script>

<template>
  <div class="relative">
    <!-- Language badge + copy button -->
    <div class="flex items-center justify-between px-4 py-2 bg-surface-900 border-b border-surface-800">
      <span class="text-[10px] text-surface-500 uppercase tracking-wider">{{ language || "text" }}</span>
      <button class="button-ghost text-xs" @click="copy">
        {{ copied ? "Copied!" : "Copy" }}
      </button>
    </div>

    <!-- Code lines -->
    <div class="overflow-x-auto font-mono text-xs leading-5">
      <table class="w-full">
        <tbody>
          <tr
            v-for="(line, i) in lines"
            :key="i"
            class="hover:bg-surface-800/50"
          >
            <td class="w-12 text-right pr-4 pl-4 py-0 text-surface-600 select-none shrink-0">
              {{ i + 1 }}
            </td>
            <td class="py-0 pr-4">
              <pre class="whitespace-pre text-surface-200">{{ line }}</pre>
            </td>
          </tr>
        </tbody>
      </table>
    </div>
  </div>
</template>
