import { Tabs as TabsBits } from "bits-ui";
import TabsList from "./TabsList.svelte";
import TabsTrigger from "./TabsTrigger.svelte";
import TabsRoot from "./TabsRoot.svelte";

export const Tabs = {
  Root: TabsRoot,
  List: TabsList,
  Trigger: TabsTrigger,
  Content: TabsBits.Content
};
