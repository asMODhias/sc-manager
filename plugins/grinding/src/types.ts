export interface MissionSuggestion {
  id: string;
  mission_name: string;
  member_rsi?: string;
  timestamp: string; // ISO
  raw_line: string;
}

export interface MissionProgress {
  id: string;
  mission_id?: string;
  mission_name?: string;
  member_id: string;
  completions: number;
  last_completed_at: string;
  verification_state: 'pending' | 'verified' | 'rejected';
  verification_method: 'manual' | 'log' | 'officer';
  verified_by?: string;
  verified_at?: string;
}