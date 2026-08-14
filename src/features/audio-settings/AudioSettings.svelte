<script lang="ts">
  import SelectField from "$lib/components/form/SelectField.svelte";
  import { Checkbox } from "$lib/components/ui";
  import { useInputDevices, useOutputDevices } from "$lib/queries/audio";
  import { audioSettingsStore } from "$lib/stores/audioSettingsStore.svelte";

  let inputDevicesQuery = useInputDevices();
  let outputDevicesQuery = useOutputDevices();

  let inputDevices = $derived(
    (inputDevicesQuery.data ?? []).map((device) => ({
      value: device.id,
      label: device.name
    }))
  );

  let outputDevices = $derived(
    (outputDevicesQuery.data ?? []).map((device) => ({
      value: device.id,
      label: device.name
    }))
  );

  function getActiveInputDevice() {
    if (audioSettingsStore.microphone_device_id !== null) {
      return audioSettingsStore.microphone_device_id;
    }
    return (inputDevicesQuery.data ?? []).find((device) => device.is_default)?.id ?? "";
  }

  function setActiveInputDevice(deviceId: string) {
    const defaultDeviceId = (inputDevicesQuery.data ?? []).find((device) => device.is_default)?.id;
    if (defaultDeviceId === deviceId) {
      audioSettingsStore.microphone_device_id = null;
    } else {
      audioSettingsStore.microphone_device_id = deviceId;
    }
  }

  function getActiveOutputDevice() {
    if (audioSettingsStore.system_device_id !== null) {
      return audioSettingsStore.system_device_id;
    }
    return (outputDevicesQuery.data ?? []).find((device) => device.is_default)?.id ?? "";
  }

  function setActiveOutputDevice(deviceId: string) {
    const defaultDeviceId = (outputDevicesQuery.data ?? []).find((device) => device.is_default)?.id;
    if (defaultDeviceId === deviceId) {
      audioSettingsStore.system_device_id = null;
    } else {
      audioSettingsStore.system_device_id = deviceId;
    }
  }
</script>

<div>
  <div class="mb-4">
    <SelectField
      label="Microphone"
      type="single"
      placeholder="No devices available"
      items={inputDevices}
      loading={inputDevicesQuery.isPending}
      disabled={!audioSettingsStore.capture_microphone}
      bind:value={getActiveInputDevice, setActiveInputDevice}
    />

    <Checkbox
      labelText="Capture microphone audio"
      bind:checked={audioSettingsStore.capture_microphone}
    />
  </div>

  <SelectField
    label="System"
    type="single"
    placeholder="No devices available"
    items={outputDevices}
    loading={outputDevicesQuery.isPending}
    disabled={!audioSettingsStore.capture_system}
    bind:value={getActiveOutputDevice, setActiveOutputDevice}
  />

  <Checkbox labelText="Capture system audio" bind:checked={audioSettingsStore.capture_system} />
</div>
