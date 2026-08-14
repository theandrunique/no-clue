export interface SystemPrompt {
  id: string;
  name: string;
  prompt: string;
  created_at: string;
  updated_at: string;
}

export interface AudioDevice {
  id: string;
  name: string;
  is_default: boolean;
}
