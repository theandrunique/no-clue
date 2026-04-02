<script setup lang="ts">
import { ref, onMounted, watch, computed } from "vue";
import { useAiProvidersStore } from "@/stores/aiProviders";
import { useSttProvidersStore } from "@/stores/sttProviders";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { getFieldTypeString, type AiProviderSettings, type SttProviderSettings } from "@/types/providers";

const aiProvidersStore = useAiProvidersStore();
const sttProvidersStore = useSttProvidersStore();

const aiSettingsForm = ref<Record<string, string>>({});
const sttSettingsForm = ref<Record<string, string>>({});
const saving = ref(false);

const selectedAiProvider = computed(() => {
  if (!aiProvidersStore.selectedProviderId) return null;
  return aiProvidersStore.providers.find(p => p.id === aiProvidersStore.selectedProviderId) || null;
});

const aiCurrentFields = computed(() => selectedAiProvider.value?.fields || []);

const selectedSttProvider = computed(() => {
  if (!sttProvidersStore.selectedProviderId) return null;
  return sttProvidersStore.providers.find(p => p.id === sttProvidersStore.selectedProviderId) || null;
});

const sttCurrentFields = computed(() => selectedSttProvider.value?.fields || []);

const sttInitialized = ref(false);

onMounted(async () => {
  await aiProvidersStore.loadProviders();
  aiProvidersStore.loadSelectedProvider();
  if (aiProvidersStore.selectedProviderId) {
    await aiProvidersStore.loadProviderSettings(aiProvidersStore.selectedProviderId);
    initAiForm();
  }

  await sttProvidersStore.loadProviders();
  sttProvidersStore.loadSelectedProvider();
  await sttProvidersStore.loadProviderSettings();
  initSttForm();
  sttInitialized.value = true;
});

watch(() => aiProvidersStore.selectedProviderId, async (newId) => {
  if (newId) {
    await aiProvidersStore.loadProviderSettings(newId);
    initAiForm();
  }
});

watch(() => sttProvidersStore.selectedProviderId, async (newId, oldId) => {
  // Only reload when provider actually changes and we've finished initial load
  if (sttInitialized.value && newId && newId !== oldId) {
    await sttProvidersStore.loadProviderSettings();
    initSttForm();
  }
});

function initAiForm() {
  aiSettingsForm.value = {};
  if (!aiProvidersStore.selectedProviderSettings) {
    for (const field of aiCurrentFields.value) {
      aiSettingsForm.value[field.key] = "";
    }
    return;
  }
  const settings = aiProvidersStore.selectedProviderSettings;
  for (const field of aiCurrentFields.value) {
    if ("Ollama" === settings.type || "AiTunnel" === settings.type) {
      aiSettingsForm.value[field.key] = (settings as any)[field.key] || "";
    }
  }
}

function initSttForm() {
  sttSettingsForm.value = {};
  if (!sttProvidersStore.selectedProviderSettings) {
    for (const field of sttCurrentFields.value) {
      sttSettingsForm.value[field.key] = "";
    }
    return;
  }
  const settings = sttProvidersStore.selectedProviderSettings;
  for (const field of sttCurrentFields.value) {
    if ("Deepgram" === settings.type) {
      sttSettingsForm.value[field.key] = (settings as any)[field.key] || "";
    }
  }
}

async function selectAiProvider(providerId: string | null) {
  if (providerId) {
    aiProvidersStore.setSelectedProvider(providerId);
  }
}

async function selectSttProvider(providerId: string | null) {
  if (providerId) {
    sttProvidersStore.setSelectedProvider(providerId);
  }
}

async function saveAiSettings() {
  if (!aiProvidersStore.selectedProviderId) return;

  saving.value = true;
  try {
    const settings = buildAiSettings();
    await aiProvidersStore.saveProviderSettings(aiProvidersStore.selectedProviderId, settings);
  } catch (e) {
    console.error("[ProvidersTab] Failed to save AI settings:", e);
  } finally {
    saving.value = false;
  }
}

async function saveSttSettings() {
  saving.value = true;
  try {
    const settings = buildSttSettings();
    await sttProvidersStore.saveProviderSettings(settings);
  } catch (e) {
    console.error("[ProvidersTab] Failed to save STT settings:", e);
  } finally {
    saving.value = false;
  }
}

function buildAiSettings(): AiProviderSettings {
  const providerId = aiProvidersStore.selectedProviderId;

  if (providerId === "fake") {
    return { type: "Fake" };
  }

  if (providerId === "ollama") {
    return {
      type: "Ollama",
      base_url: aiSettingsForm.value.base_url || undefined,
      model: aiSettingsForm.value.model,
    };
  }

  if (providerId === "aitunnel") {
    return {
      type: "AiTunnel",
      api_key: aiSettingsForm.value.api_key,
      model: aiSettingsForm.value.model,
    };
  }

  return { type: "Fake" };
}

function buildSttSettings(): SttProviderSettings {
  const providerId = sttProvidersStore.selectedProviderId;

  if (providerId === "fake") {
    return { type: "Fake" };
  }

  if (providerId === "deepgram") {
    return {
      type: "Deepgram",
      api_key: sttSettingsForm.value.api_key || undefined,
      language: sttSettingsForm.value.language || undefined,
      model: sttSettingsForm.value.model || undefined,
    };
  }

  return { type: "Fake" };
}
</script>

<template>
  <div class="h-full p-6 overflow-y-auto">
    <div class="max-w-2xl space-y-8">
      <h2 class="text-lg font-medium text-foreground">Providers</h2>

      <!-- AI Providers -->
      <div>
        <h3 class="text-md font-medium text-foreground mb-4">AI Provider</h3>

        <div v-if="aiProvidersStore.loading" class="text-muted-foreground">
          Loading AI providers...
        </div>

        <template v-else>
          <Card>
            <CardHeader>
              <CardTitle>Select AI Provider</CardTitle>
            </CardHeader>
            <CardContent>
              <Select
                :model-value="aiProvidersStore.selectedProviderId"
                @update:model-value="(val: unknown) => selectAiProvider(val as string | null)"
              >
                <SelectTrigger class="w-full">
                  <SelectValue placeholder="Select a provider" />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem
                    v-for="provider in aiProvidersStore.providers"
                    :key="provider.id"
                    :value="provider.id"
                  >
                    {{ provider.label }}
                  </SelectItem>
                </SelectContent>
              </Select>
            </CardContent>
          </Card>

          <Card v-if="selectedAiProvider && aiCurrentFields.length > 0">
            <CardHeader>
              <CardTitle>{{ selectedAiProvider.label }} Settings</CardTitle>
            </CardHeader>
            <CardContent class="space-y-4">
              <div
                v-for="field in aiCurrentFields"
                :key="field.key"
                class="space-y-2"
              >
                <label :for="field.key" class="text-sm font-medium">
                  {{ field.label }}
                  <span v-if="field.required" class="text-destructive">*</span>
                </label>
                <Input
                  :id="field.key"
                  v-model="aiSettingsForm[field.key]"
                  :type="getFieldTypeString(field.field_type)"
                  :placeholder="field.placeholder"
                  :required="field.required"
                />
              </div>

              <Button @click="saveAiSettings" :disabled="saving">
                {{ saving ? "Saving..." : "Save Settings" }}
              </Button>
            </CardContent>
          </Card>

          <Card v-else-if="selectedAiProvider && aiCurrentFields.length === 0">
            <CardContent class="py-6">
              <p class="text-muted-foreground">No configuration needed for this provider.</p>
            </CardContent>
          </Card>

          <div v-else class="text-muted-foreground">
            Select a provider to configure.
          </div>
        </template>
      </div>

      <!-- STT Providers -->
      <div>
        <h3 class="text-md font-medium text-foreground mb-4">STT Provider</h3>

        <div v-if="sttProvidersStore.loading" class="text-muted-foreground">
          Loading STT providers...
        </div>

        <template v-else>
          <Card>
            <CardHeader>
              <CardTitle>Select STT Provider</CardTitle>
            </CardHeader>
            <CardContent>
              <Select
                :model-value="sttProvidersStore.selectedProviderId"
                @update:model-value="(val: unknown) => selectSttProvider(val as string | null)"
              >
                <SelectTrigger class="w-full">
                  <SelectValue placeholder="Select a provider" />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem
                    v-for="provider in sttProvidersStore.providers"
                    :key="provider.id"
                    :value="provider.id"
                  >
                    {{ provider.label }}
                  </SelectItem>
                </SelectContent>
              </Select>
            </CardContent>
          </Card>

          <Card v-if="selectedSttProvider && sttCurrentFields.length > 0">
            <CardHeader>
              <CardTitle>{{ selectedSttProvider.label }} Settings</CardTitle>
            </CardHeader>
            <CardContent class="space-y-4">
              <div
                v-for="field in sttCurrentFields"
                :key="field.key"
                class="space-y-2"
              >
                <label :for="field.key" class="text-sm font-medium">
                  {{ field.label }}
                  <span v-if="field.required" class="text-destructive">*</span>
                </label>
                <Input
                  :id="field.key"
                  v-model="sttSettingsForm[field.key]"
                  :type="getFieldTypeString(field.field_type)"
                  :placeholder="field.placeholder"
                  :required="field.required"
                />
              </div>

              <Button @click="saveSttSettings" :disabled="saving">
                {{ saving ? "Saving..." : "Save Settings" }}
              </Button>
            </CardContent>
          </Card>

          <Card v-else-if="selectedSttProvider && sttCurrentFields.length === 0">
            <CardContent class="py-6">
              <p class="text-muted-foreground">No configuration needed for this provider.</p>
            </CardContent>
          </Card>

          <div v-else class="text-muted-foreground">
            Select a provider to configure.
          </div>
        </template>
      </div>
    </div>
  </div>
</template>
