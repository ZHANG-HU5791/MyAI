import { defineStore } from "pinia";
import { ref } from "vue";

export const useTokenEconomyStore = defineStore("tokenEconomy", () => {
  const totalTokensUsed = ref(0);
  const cacheHits = ref(0);
  const totalRequests = ref(0);
  const estimatedCost = ref(0);

  function recordUsage(tokens: number, model: string, cacheHit: boolean) {
    totalTokensUsed.value += tokens;
    totalRequests.value += 1;
    if (cacheHit) cacheHits.value += 1;

    // Rough cost estimation (per 1M tokens)
    const costPerMillion = model.includes("flash") ? 0.075 : 1.25;
    estimatedCost.value += (tokens / 1_000_000) * costPerMillion;
  }

  function incrementCacheHits() {
    cacheHits.value += 1;
  }

  return {
    totalTokensUsed,
    cacheHits,
    totalRequests,
    estimatedCost,
    recordUsage,
    incrementCacheHits,
  };
});
