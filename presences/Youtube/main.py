import time
from typing import Optional, Any, Dict
from src.rpc import ActivityType, ClientRPC
from src.runtime import Runtime, Page

# module-level logger that will be set when the worker starts
module_logger: Any = None


def on_load(rpc: ClientRPC):
    rpc.update(
        activity_type=ActivityType.WATCHING,
        details="Initializing..",
        state="YouTube",
        large_image="youtube",
        large_text="YouTube Presence",
        small_image="youtube_small",
        small_text="Watching on YouTube",
        buttons=[],
        end_time=None,
        start_time=None,
    )


def main(rpc: ClientRPC, runtime: Optional[Runtime], stop_event: Any, logger: Any):
    # set module-level logger so helper functions can use it
    global module_logger
    module_logger = logger

    on_load(rpc)

    try:
        while not stop_event.is_set():
            module_logger.debug("YouTube presence worker is running...")
            stop_event.wait(5)
    except Exception as exc:
        raise exc