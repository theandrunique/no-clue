import { conversationApi } from "$lib/api/conversation";
import { overlayApi } from "$lib/api/overlay";

function createOverlaySessionService() {
  async function startNew() {
    try {
      const conversation = await conversationApi.create();
      await start(conversation.id);
    } catch {
      console.log("Failed to create conversation and start");
    }
  }

  async function start(conversationId: string) {
    try {
      await overlayApi.start(conversationId);
    } catch (e) {
      console.log("Error while start()", e);
    }
  }

  async function stop() {
    try {
      await overlayApi.stop();
    } catch (e) {
      console.log("Error while stop()", e);
    }
  }

  return {
    startNew,
    start,
    stop
  };
}

export const overlaySessionService = createOverlaySessionService();
