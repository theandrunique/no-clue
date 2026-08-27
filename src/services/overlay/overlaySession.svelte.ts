import { conversationIpc } from "$lib/ipc/conversation";
import { overlayIpc } from "$lib/ipc/overlay";

function createOverlaySessionService() {
  async function startNew() {
    try {
      const conversation = await conversationIpc.create();
      await start(conversation.id);
    } catch {
      console.log("Failed to create conversation and start");
    }
  }

  async function start(conversationId: string) {
    try {
      await overlayIpc.start(conversationId);
    } catch (e) {
      console.log("Error while start()", e);
    }
  }

  async function stop() {
    try {
      await overlayIpc.stop();
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
