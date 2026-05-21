import { defineStore } from "pinia";
import { ref } from "vue";
import type { MasterSpec } from "@/types/master-spec";
import { useIpc } from "@/composables/useIpc";

export const useMemoryStore = defineStore("memory", () => {
  const masterSpec = ref<MasterSpec | null>(null);
  const { call } = useIpc();

  async function load(sessionId?: string) {
    masterSpec.value = await call<MasterSpec>("get_master_spec", {
      sessionId: sessionId ?? null,
    });
  }

  async function update(spec: MasterSpec, sessionId?: string) {
    await call("update_master_spec", {
      sessionId: sessionId ?? null,
      spec,
    });
    masterSpec.value = spec;
  }

  async function addGoal(goal: string, sessionId?: string) {
    await call("update_active_goal", {
      sessionId: sessionId ?? null,
      goal,
    });
    await load(sessionId);
  }

  async function addConstraint(constraint: string, sessionId?: string) {
    await call("add_constraint", {
      sessionId: sessionId ?? null,
      constraint,
    });
    await load(sessionId);
  }

  /** Refetch master spec — called by the global event dispatcher. */
  async function refetch() {
    await load();
  }

  return {
    masterSpec,
    load,
    update,
    addGoal,
    addConstraint,
    refetch,
  };
});
