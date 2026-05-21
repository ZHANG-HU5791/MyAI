export interface MasterSpec {
  global_constraints: GlobalConstraints;
  current_state: CurrentState;
  active_workspace: ActiveWorkspace;
}

export interface GlobalConstraints {
  target_goals: string[];
  blacklisted_approaches: string[];
  user_preferences: Record<string, string>;
}

export interface CurrentState {
  project_description: string;
  tech_stack: string[];
  completed_milestones: string[];
  active_files: string[];
}

export interface ActiveWorkspace {
  recent_turns: WorkspaceTurn[];
  pending_decisions: string[];
}

export interface WorkspaceTurn {
  role: string;
  summary: string;
  timestamp: string;
}
