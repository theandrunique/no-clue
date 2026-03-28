<script setup lang="ts">
import { ref, onMounted, watch, computed } from "vue";
import { useProvidersStore } from "@/stores/providers";
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
import { getFieldTypeString, type ProviderSettings, type FieldType } from "@/types/providers";

const providersStore = useProvidersStore();

const settingsForm = ref<Record<string, string>>({});
const saving = ref(false);

const selectedProvider = computed(() => {
  if (!providersStore.selectedProviderId) return null;
  return providersStore.providers.find(p => p.id === providersStore.selectedProviderId) || null;
});

const currentFields = computed(() => selectedProvider.value?.fields || []);

onMounted(async () => {
  await providersStore.loadProviders();
  providersStore.loadSelectedProvider();

  if (providersStore.selectedProviderId) {
    await providersStore.loadProviderSettings(providersStore.selectedProviderId);
    initForm();
  }
});

watch(() => providersStore.selectedProviderId, async (newId) => {
  if (newId) {
    await providersStore.loadProviderSettings(newId);
    initForm();
  }
});

function initForm() {
  settingsForm.value = {};
  if (!providersStore.selectedProviderSettings) {
    for (const field of currentFields.value) {
      settingsForm.value[field.key] = "";
    }
    return;
  }
  const settings = providersStore.selectedProviderSettings;
  for (const field of currentFields.value) {
    if ("Ollama" === settings.type) {
      settingsForm.value[field.key] = (settings as any)[field.key] || "";
    } else if ("AiTunnel" === settings.type) {
      settingsForm.value[field.key] = (settings as any)[field.key] || "";
    }
  }
}

async function selectProvider(providerId: string | null) {
  if (providerId) {
    providersStore.setSelectedProvider(providerId);
  }
}

async function saveSettings() {
  if (!providersStore.selectedProviderId) return;

  saving.value = true;
  try {
    const settings = buildSettings();
    await providersStore.saveProviderSettings(providersStore.selectedProviderId, settings);
  } catch (e) {
    console.error("[ProvidersTab] Failed to save settings:", e);
  } finally {
    saving.value = false;
  }
}

function buildSettings(): ProviderSettings {
  const providerId = providersStore.selectedProviderId;

  if (providerId === "fake") {
    return { type: "Fake" };
  }

  if (providerId === "ollama") {
    return {
      type: "Ollama",
      base_url: settingsForm.value.base_url || undefined,
      model: settingsForm.value.model,
    };
  }

  if (providerId === "aitunnel") {
    return {
      type: "AiTunnel",
      api_key: settingsForm.value.api_key,
      model: settingsForm.value.model,
    };
  }

  return { type: "Fake" };
}

function getFieldType(field: FieldType): string {
  return getFieldTypeString(field);
}
</script>

<template>
  <div class="h-full p-6 overflow-y-auto">
    <div class="max-w-2xl space-y-6">
      <h2 class="text-lg font-medium text-foreground">Providers</h2>

      <div v-if="providersStore.loading" class="text-muted-foreground">
        Loading providers...
      </div>

      <template v-else>
        <Card>
          <CardHeader>
            <CardTitle>Select AI Provider</CardTitle>
          </CardHeader>
          <CardContent>
            <Select
              :model-value="providersStore.selectedProviderId"
              @update:model-value="(val: unknown) => selectProvider(val as string | null)"
            >
              <SelectTrigger class="w-full">
                <SelectValue placeholder="Select a provider" />
              </SelectTrigger>
              <SelectContent>
                <SelectItem
                  v-for="provider in providersStore.providers"
                  :key="provider.id"
                  :value="provider.id"
                >
                  {{ provider.label }}
                </SelectItem>
              </SelectContent>
            </Select>
          </CardContent>
        </Card>

        <Card v-if="selectedProvider && currentFields.length > 0">
          <CardHeader>
            <CardTitle>{{ selectedProvider.label }} Settings</CardTitle>
          </CardHeader>
          <CardContent class="space-y-4">
            <div
              v-for="field in currentFields"
              :key="field.key"
              class="space-y-2"
            >
              <label :for="field.key" class="text-sm font-medium">
                {{ field.label }}
                <span v-if="field.required" class="text-destructive">*</span>
              </label>
              <Input
                :id="field.key"
                v-model="settingsForm[field.key]"
                :type="getFieldType(field.field_type)"
                :placeholder="field.placeholder"
                :required="field.required"
              />
            </div>

            <Button @click="saveSettings" :disabled="saving">
              {{ saving ? "Saving..." : "Save Settings" }}
            </Button>
          </CardContent>
        </Card>

        <Card v-else-if="selectedProvider && currentFields.length === 0">
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
</template>
