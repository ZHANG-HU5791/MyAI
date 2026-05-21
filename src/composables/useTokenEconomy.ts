import { computed } from "vue";
import { useTokenEconomyStore } from "@/stores/token-economy-store";

export function useTokenEconomy() {
  const store = useTokenEconomyStore();

  const cacheHitRate = computed(() => {
    const total = store.totalRequests;
    return total > 0 ? (store.cacheHits / total) * 100 : 0;
  });

  return {
    cacheHitRate,
    totalTokens: computed(() => store.totalTokensUsed),
    totalRequests: computed(() => store.totalRequests),
    estimatedCost: computed(() => store.estimatedCost),
  };
}
