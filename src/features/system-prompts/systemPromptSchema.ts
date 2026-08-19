import { z } from "zod";

export const createSystemPromptSchema = z.object({
  name: z.string().trim().min(1, "Name is required").max(50, "Max 50 symbols"),
  prompt: z.string().trim().min(1, "Prompt is required")
});
